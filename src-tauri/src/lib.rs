mod clipboard;
mod models;
mod state;

use clipboard::ClipboardMonitor;
use models::{ClipboardItem, SequenceState};
use state::AppState;
use std::sync::Arc;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_global_shortcut::GlobalShortcutExt;

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

struct TrayMenuItems {
    show: MenuItem<tauri::Wry>,
    quit: MenuItem<tauri::Wry>,
}

#[tauri::command]
fn set_ui_language(lang: String, menu_items: State<'_, TrayMenuItems>) {
    let (show_text, quit_text) = if lang == "en" {
        ("Show Main Window", "Quit")
    } else {
        ("顯示主視窗", "離開")
    };
    let _ = menu_items.show.set_text(show_text);
    let _ = menu_items.quit.set_text(quit_text);
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

    tauri::Builder::default()
        .manage(app_state.clone())
        .manage(monitor.clone())
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
            set_ui_language,
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

            // Build System Tray Icon & Menu.
            // Labels are single-language and follow the UI language via set_ui_language;
            // quick settings live in the slider panel (left-click), not in the menu.
            let show_i = MenuItem::with_id(app.handle(), "show", "顯示主視窗", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app.handle(), "quit", "離開", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app.handle(), &[&show_i, &quit_i])?;
            app.manage(TrayMenuItems {
                show: show_i,
                quit: quit_i,
            });

            let mut tray_builder = TrayIconBuilder::new()
                .menu(&tray_menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app: &AppHandle, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray: &TrayIcon, event| {
                    // Left-click toggles the quick-settings slider panel anchored to the tray icon
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        position,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(panel) = app.get_webview_window("quickset") {
                            if panel.is_visible().unwrap_or(false) {
                                let _ = panel.hide();
                            } else {
                                let size = panel
                                    .outer_size()
                                    .unwrap_or(tauri::PhysicalSize::new(320, 268));
                                let x = (position.x - size.width as f64 / 2.0).max(8.0);
                                // Menu bar tray sits at the top of the screen (open downward);
                                // a taskbar tray sits at the bottom (open upward).
                                let y = if position.y < 400.0 {
                                    position.y + 12.0
                                } else {
                                    position.y - size.height as f64 - 12.0
                                };
                                let _ = panel.set_position(tauri::PhysicalPosition::new(x, y));
                                let _ = panel.show();
                                let _ = panel.set_focus();
                            }
                        }
                    }
                });

            if let Ok(icon) = tauri::image::Image::from_bytes(include_bytes!("../icons/tray_icon.png")) {
                tray_builder = tray_builder.icon(icon).icon_as_template(true);
            } else if let Some(icon) = app.default_window_icon() {
                tray_builder = tray_builder.icon(icon.clone());
            }

            let _tray = tray_builder.build(app.handle())?;

            // Quick-settings panel dismisses itself when it loses focus
            if let Some(panel) = app.get_webview_window("quickset") {
                let panel_handle = panel.clone();
                panel.on_window_event(move |event| {
                    if let tauri::WindowEvent::Focused(false) = event {
                        let _ = panel_handle.hide();
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
