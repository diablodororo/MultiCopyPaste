mod clipboard;
mod models;
mod state;

use clipboard::ClipboardMonitor;
use models::{ClipboardItem, SequenceState};
use state::AppState;
use std::sync::Arc;
use tauri::menu::{Menu, MenuItem, Submenu};
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

            let target_len_2 = MenuItem::with_id(app.handle(), "target_len_2", "2 筆 / 2 Items", true, None::<&str>)?;
            let target_len_3 = MenuItem::with_id(app.handle(), "target_len_3", "3 筆 / 3 Items", true, None::<&str>)?;
            let target_len_5 = MenuItem::with_id(app.handle(), "target_len_5", "5 筆 / 5 Items", true, None::<&str>)?;
            let target_len_10 = MenuItem::with_id(app.handle(), "target_len_10", "10 筆 / 10 Items", true, None::<&str>)?;
            let target_len_20 = MenuItem::with_id(app.handle(), "target_len_20", "20 筆 / 20 Items", true, None::<&str>)?;
            let submenu_target_len = Submenu::with_items(
                app.handle(),
                "序列循環長度 / Sequence Length",
                true,
                &[&target_len_2, &target_len_3, &target_len_5, &target_len_10, &target_len_20],
            )?;

            let repeat_1 = MenuItem::with_id(app.handle(), "repeat_1", "1 次 (貼完清空) / 1 Time (Clear after paste)", true, None::<&str>)?;
            let repeat_2 = MenuItem::with_id(app.handle(), "repeat_2", "2 次 / 2 Times", true, None::<&str>)?;
            let repeat_3 = MenuItem::with_id(app.handle(), "repeat_3", "3 次 / 3 Times", true, None::<&str>)?;
            let repeat_5 = MenuItem::with_id(app.handle(), "repeat_5", "5 次 / 5 Times", true, None::<&str>)?;
            let repeat_0 = MenuItem::with_id(app.handle(), "repeat_0", "無限循環 (不清空) / Infinite (No clear)", true, None::<&str>)?;
            let submenu_repeat = Submenu::with_items(
                app.handle(),
                "重複貼上循環次數 / Repeat Paste Cycles",
                true,
                &[&repeat_1, &repeat_2, &repeat_3, &repeat_5, &repeat_0],
            )?;

            let tray_menu = Menu::with_items(app.handle(), &[&show_i, &submenu_target_len, &submenu_repeat, &quit_i])?;

            let tray_app_state = app_state.clone();
            let mut tray_builder = TrayIconBuilder::new()
                .menu(&tray_menu)
                .show_menu_on_left_click(true)
                .on_menu_event(move |app: &AppHandle, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    "target_len_2" => {
                        let new_state = tray_app_state.set_target_length(2);
                        let _ = app.emit("sequence-updated", new_state);
                    }
                    "target_len_3" => {
                        let new_state = tray_app_state.set_target_length(3);
                        let _ = app.emit("sequence-updated", new_state);
                    }
                    "target_len_5" => {
                        let new_state = tray_app_state.set_target_length(5);
                        let _ = app.emit("sequence-updated", new_state);
                    }
                    "target_len_10" => {
                        let new_state = tray_app_state.set_target_length(10);
                        let _ = app.emit("sequence-updated", new_state);
                    }
                    "target_len_20" => {
                        let new_state = tray_app_state.set_target_length(20);
                        let _ = app.emit("sequence-updated", new_state);
                    }
                    "repeat_1" => {
                        let new_state = tray_app_state.set_repeat_count(1);
                        let _ = app.emit("sequence-updated", new_state);
                    }
                    "repeat_2" => {
                        let new_state = tray_app_state.set_repeat_count(2);
                        let _ = app.emit("sequence-updated", new_state);
                    }
                    "repeat_3" => {
                        let new_state = tray_app_state.set_repeat_count(3);
                        let _ = app.emit("sequence-updated", new_state);
                    }
                    "repeat_5" => {
                        let new_state = tray_app_state.set_repeat_count(5);
                        let _ = app.emit("sequence-updated", new_state);
                    }
                    "repeat_0" => {
                        let new_state = tray_app_state.set_repeat_count(0);
                        let _ = app.emit("sequence-updated", new_state);
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
