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
            reset_sequence_index,
            set_sequence_index,
            update_sequence_items,
            clear_sequence,
            delete_sequence_item,
            update_sequence_item,
            manual_paste_next
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

            // Build System Tray Icon & Menu
            let show_i = MenuItem::with_id(app.handle(), "show", "顯示視窗 / Show Window", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app.handle(), "quit", "離開 / Quit", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app.handle(), &[&show_i, &quit_i])?;

            let mut tray_builder = TrayIconBuilder::new()
                .menu(&tray_menu)
                .show_menu_on_left_click(true)
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
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                });

            if let Ok(icon) = tauri::image::Image::from_bytes(include_bytes!("../icons/tray_icon.png")) {
                tray_builder = tray_builder.icon(icon).icon_as_template(true);
            } else if let Some(icon) = app.default_window_icon() {
                tray_builder = tray_builder.icon(icon.clone());
            }

            let _tray = tray_builder.build(app.handle())?;

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
