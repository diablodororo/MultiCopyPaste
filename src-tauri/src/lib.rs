mod clipboard;
mod models;
mod state;

use clipboard::ClipboardMonitor;
use models::{ClipboardItem, SequenceState};
use state::AppState;
use std::sync::Arc;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_global_shortcut::GlobalShortcutExt;

#[tauri::command]
fn get_sequence_state(state: State<'_, AppState>) -> SequenceState {
    state.get_state()
}

#[tauri::command]
fn set_target_length(length: usize, state: State<'_, AppState>) -> SequenceState {
    state.set_target_length(length)
}

#[tauri::command]
fn reset_sequence_index(state: State<'_, AppState>) -> SequenceState {
    state.reset_index();
    state.get_state()
}

#[tauri::command]
fn update_sequence_items(items: Vec<ClipboardItem>, state: State<'_, AppState>) -> SequenceState {
    state.set_items(items)
}

#[tauri::command]
fn clear_sequence(state: State<'_, AppState>) -> SequenceState {
    state.clear_all()
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
            get_sequence_state,
            set_target_length,
            reset_sequence_index,
            update_sequence_items,
            clear_sequence,
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
