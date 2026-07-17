mod clipboard;
mod models;
mod state;

use clipboard::ClipboardMonitor;
use models::{ClipboardItem, SequenceState};
use state::AppState;
use std::sync::Arc;
use tauri::tray::{MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_global_shortcut::GlobalShortcutExt;
#[cfg(target_os = "macos")]
use tauri_nspanel::panel_delegate;

#[tauri::command]
fn log_debug(msg: String) {
    println!("[UI LOG] {}", msg);
}

#[tauri::command]
fn get_sequence_state(state: State<'_, AppState>) -> SequenceState {
    state.get_state()
}

#[tauri::command]
fn set_target_length(length: usize, state: State<'_, AppState>, app: AppHandle) -> SequenceState {
    let new_state = state.set_target_length(length);
    let _ = app.emit("sequence-updated", new_state.clone());
    new_state
}

#[tauri::command]
fn set_repeat_count(count: usize, state: State<'_, AppState>, app: AppHandle) -> SequenceState {
    let new_state = state.set_repeat_count(count);
    let _ = app.emit("sequence-updated", new_state.clone());
    new_state
}

#[tauri::command]
fn reset_sequence_index(state: State<'_, AppState>, app: AppHandle) -> SequenceState {
    state.reset_index();
    let new_state = state.get_state();
    let _ = app.emit("sequence-updated", new_state.clone());
    new_state
}

#[tauri::command]
fn set_sequence_index(index: usize, state: State<'_, AppState>, app: AppHandle) -> SequenceState {
    let new_state = state.set_sequence_index(index);
    let _ = app.emit("sequence-updated", new_state.clone());
    new_state
}

#[tauri::command]
fn update_sequence_items(items: Vec<ClipboardItem>, state: State<'_, AppState>, app: AppHandle) -> SequenceState {
    let new_state = state.set_items(items);
    let _ = app.emit("sequence-updated", new_state.clone());
    new_state
}

#[tauri::command]
fn clear_sequence(state: State<'_, AppState>, app: AppHandle) -> SequenceState {
    let new_state = state.clear_all();
    let _ = app.emit("sequence-updated", new_state.clone());
    new_state
}

#[tauri::command]
fn delete_sequence_item(id: String, state: State<'_, AppState>, app: AppHandle) -> SequenceState {
    let new_state = state.delete_item(id);
    let _ = app.emit("sequence-updated", new_state.clone());
    new_state
}

#[tauri::command]
fn update_sequence_item(id: String, content: String, state: State<'_, AppState>, app: AppHandle) -> SequenceState {
    let new_state = state.update_item_content(id, content);
    let _ = app.emit("sequence-updated", new_state.clone());
    new_state
}

#[tauri::command]
fn manual_paste_next(
    app: AppHandle,
    state: State<'_, AppState>,
    monitor: State<'_, Arc<ClipboardMonitor>>,
) {
    monitor.trigger_paste(state.inner().clone(), app);
}

#[tauri::command]
fn quit_app(app: AppHandle) {
    app.exit(0);
}

/// Whether the quick-settings popover is pinned: a pinned popover stays on
/// top and is not dismissed when it loses key/focus status.
struct QuicksetPinned(std::sync::atomic::AtomicBool);

#[tauri::command]
fn set_quickset_pinned(pinned: bool, state: State<'_, QuicksetPinned>) {
    state.0.store(pinned, std::sync::atomic::Ordering::SeqCst);
}

fn is_quickset_pinned(app: &AppHandle) -> bool {
    app.state::<QuicksetPinned>()
        .0
        .load(std::sync::atomic::Ordering::SeqCst)
}

/// Synthetic keyboard events are silently dropped by macOS unless the app has
/// been granted Accessibility permission — and that grant is invalidated
/// whenever the (ad-hoc) code signature changes, e.g. after every rebuild.
/// Check at startup and trigger the system prompt so the failure is visible
/// instead of pastes silently doing nothing.
#[cfg(target_os = "macos")]
fn ensure_accessibility_permission() {
    use core_foundation::base::TCFType;
    use core_foundation::boolean::CFBoolean;
    use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
    use core_foundation::string::{CFString, CFStringRef};

    #[link(name = "ApplicationServices", kind = "framework")]
    extern "C" {
        fn AXIsProcessTrustedWithOptions(options: CFDictionaryRef) -> bool;
        static kAXTrustedCheckOptionPrompt: CFStringRef;
    }

    unsafe {
        let key = CFString::wrap_under_get_rule(kAXTrustedCheckOptionPrompt);
        let options = CFDictionary::from_CFType_pairs(&[(
            key.as_CFType(),
            CFBoolean::true_value().as_CFType(),
        )]);
        let trusted = AXIsProcessTrustedWithOptions(options.as_concrete_TypeRef());
        if trusted {
            eprintln!("[MultiCopyPaste] Accessibility permission OK");
        } else {
            eprintln!(
                "[MultiCopyPaste] Accessibility permission MISSING — paste injection will not work. \
                 Grant it in System Settings > Privacy & Security > Accessibility, then relaunch."
            );
        }
    }
}

/// Positions the quick-settings panel under the tray icon.
///
/// The anchor comes from `AppHandle::cursor_position()` (tao's coordinate
/// space) rather than the tray event's coordinates: tray-icon's macOS math
/// mixes logical points with the main display's pixel height, producing
/// positions on the wrong monitor in multi-screen setups. Cursor position and
/// `monitor_from_point` share one coordinate space, so the popover opens on
/// the monitor that was actually clicked.
fn position_quick_panel(app: &AppHandle, panel: &tauri::WebviewWindow) {
    let anchor = app.cursor_position().ok();
    let Some(anchor) = anchor else {
        let _ = panel.center();
        return;
    };

    let size = panel
        .outer_size()
        .unwrap_or(tauri::PhysicalSize::new(320, 460));
    let mut x = anchor.x - size.width as f64 / 2.0;
    let mut y = anchor.y + 8.0;

    if let Ok(Some(monitor)) = app.monitor_from_point(anchor.x, anchor.y) {
        let m_pos = monitor.position();
        let m_size = monitor.size();
        // Menu bar tray sits near the monitor's top edge (open downward);
        // a taskbar tray sits near the bottom (open upward).
        let near_top = anchor.y - m_pos.y as f64 <= m_size.height as f64 * 0.25;
        if !near_top {
            y = anchor.y - size.height as f64 - 8.0;
        }
        let min_x = m_pos.x as f64 + 8.0;
        let max_x = (m_pos.x as f64 + m_size.width as f64 - size.width as f64 - 8.0).max(min_x);
        let min_y = m_pos.y as f64 + 8.0;
        let max_y =
            (m_pos.y as f64 + m_size.height as f64 - size.height as f64 - 8.0).max(min_y);
        x = x.clamp(min_x, max_x);
        y = y.clamp(min_y, max_y);
    }
    let _ = panel.set_position(tauri::PhysicalPosition::new(x, y));
}

/// Toggles the quick-settings popover. On macOS the window is backed by a
/// non-activating NSPanel so it behaves like a real menu bar popover: the
/// frontmost app keeps focus, and clicking anywhere else dismisses it.
fn toggle_quick_panel(app: &AppHandle) {
    let Some(window) = app.get_webview_window("quickset") else {
        return;
    };

    #[cfg(target_os = "macos")]
    {
        use tauri_nspanel::ManagerExt as _;
        if let Ok(panel) = app.get_webview_panel("quickset") {
            if panel.is_visible() {
                panel.order_out(None);
            } else {
                position_quick_panel(app, &window);
                panel.show();
            }
            return;
        }
    }

    if window.is_visible().unwrap_or(false) {
        let _ = window.hide();
    } else {
        position_quick_panel(app, &window);
        let _ = window.show();
        let _ = window.set_focus();
    }
}

#[tauri::command]
fn show_main_window(app: AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::new();
    let monitor = Arc::new(ClipboardMonitor::new());

    let builder = tauri::Builder::default()
        .manage(app_state.clone())
        .manage(monitor.clone())
        .manage(QuicksetPinned(std::sync::atomic::AtomicBool::new(false)));

    #[cfg(target_os = "macos")]
    let builder = builder.plugin(tauri_nspanel::init());

    builder
        .invoke_handler(tauri::generate_handler![
            log_debug,
            get_sequence_state,
            set_target_length,
            set_repeat_count,
            reset_sequence_index,
            set_sequence_index,
            update_sequence_items,
            clear_sequence,
            delete_sequence_item,
            update_sequence_item,
            manual_paste_next,
            quit_app,
            set_quickset_pinned,
            show_main_window
        ])
        .setup(move |app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Start clipboard monitor thread
            monitor.start_monitoring(app_state.clone(), app.handle().clone());

            // Verify Accessibility permission up-front so paste failures are
            // diagnosable instead of silent (rebuilds invalidate the grant).
            #[cfg(target_os = "macos")]
            ensure_accessibility_permission();

            // Build System Tray Icon — deliberately WITHOUT a native menu.
            // With a menu attached, macOS shows it on left click regardless of
            // show_menu_on_left_click(false), hiding the popover. Any click
            // (left or right) toggles the quick-settings popover instead; the
            // Show Main Window / Quit actions live inside the popover.
            let mut tray_builder = TrayIconBuilder::new().on_tray_icon_event(
                |tray: &TrayIcon, event| {
                    if let TrayIconEvent::Click {
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        toggle_quick_panel(tray.app_handle());
                    }
                },
            );

            if let Ok(icon) = tauri::image::Image::from_bytes(include_bytes!("../icons/tray_icon.png")) {
                tray_builder = tray_builder.icon(icon).icon_as_template(true);
            } else if let Some(icon) = app.default_window_icon() {
                tray_builder = tray_builder.icon(icon.clone());
            }

            let _tray = tray_builder.build(app.handle())?;

            // Turn the quick-settings window into a real menu bar popover:
            // a non-activating NSPanel above the menu bar, visible on every
            // space, dismissed as soon as it loses key status.
            #[cfg(target_os = "macos")]
            if let Some(window) = app.get_webview_window("quickset") {
                use tauri_nspanel::cocoa::appkit::NSWindowCollectionBehavior;
                use tauri_nspanel::WebviewWindowExt as _;

                let panel = window.to_panel()?;
                const NS_WINDOW_STYLE_MASK_NON_ACTIVATING_PANEL: i32 = 1 << 7;
                const NS_MAIN_MENU_WINDOW_LEVEL: i32 = 24;
                panel.set_level(NS_MAIN_MENU_WINDOW_LEVEL + 1);
                panel.set_style_mask(NS_WINDOW_STYLE_MASK_NON_ACTIVATING_PANEL);
                panel.set_collection_behaviour(
                    NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
                        | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary
                        | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary,
                );

                let delegate = panel_delegate!(QuickPanelDelegate {
                    window_did_resign_key
                });
                let panel_for_hide = panel.clone();
                let handle_for_hide = app.handle().clone();
                delegate.set_listener(Box::new(move |delegate_name: String| {
                    if delegate_name.as_str() == "window_did_resign_key"
                        && !is_quickset_pinned(&handle_for_hide)
                    {
                        panel_for_hide.order_out(None);
                    }
                }));
                panel.set_delegate(delegate);
            }

            // Non-macOS: plain window that dismisses itself when it loses focus
            #[cfg(not(target_os = "macos"))]
            if let Some(panel) = app.get_webview_window("quickset") {
                let panel_handle = panel.clone();
                let app_handle = app.handle().clone();
                panel.on_window_event(move |event| {
                    if let tauri::WindowEvent::Focused(false) = event {
                        if !is_quickset_pinned(&app_handle) {
                            let _ = panel_handle.hide();
                        }
                    }
                });
            }

            // Intercept window close event to hide to tray instead of exiting
            if let Some(window) = app.get_webview_window("main") {
                let window_handle = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        let _ = window_handle.hide();
                        api.prevent_close();
                    }
                });
            }

            // Register global shortcut plugin
            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_handler(move |app, _shortcut, event| {
                        if event.state() == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                            if let (Some(app_state), Some(monitor)) = (
                                app.try_state::<AppState>(),
                                app.try_state::<Arc<ClipboardMonitor>>(),
                            ) {
                                monitor.trigger_paste(app_state.inner().clone(), app.clone());
                            }
                        }
                    })
                    .build(),
            )?;

            // Register default Control+Alt+V (Ctrl+Option+V) shortcut
            #[cfg(target_os = "macos")]
            let default_shortcut = "Control+Alt+V";
            #[cfg(not(target_os = "macos"))]
            let default_shortcut = "Control+Alt+V";

            if let Err(e) = app.handle().global_shortcut().register(default_shortcut) {
                eprintln!("Failed to register global shortcut {}: {}", default_shortcut, e);
            } else {
                println!("Registered global shortcut: {}", default_shortcut);
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
