use crate::state::AppState;
use arboard::Clipboard;
use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

pub struct ClipboardMonitor {
    last_text: Arc<Mutex<String>>,
}

impl ClipboardMonitor {
    pub fn new() -> Self {
        Self {
            last_text: Arc::new(Mutex::new(String::new())),
        }
    }

    pub fn start_monitoring(&self, app_state: AppState, app_handle: AppHandle) {
        let last_text = Arc::clone(&self.last_text);
        thread::spawn(move || {
            let mut clipboard = match Clipboard::new() {
                Ok(cb) => cb,
                Err(e) => {
                    eprintln!("Failed to initialize clipboard: {}", e);
                    return;
                }
            };

            loop {
                if let Ok(text) = clipboard.get_text() {
                    let text = text.trim().to_string();
                    if !text.is_empty() {
                        let mut last = last_text.lock();
                        if *last != text {
                            *last = text.clone();
                            drop(last);

                            if app_state.record_copy(text) {
                                let state = app_state.get_state();
                                let _ = app_handle.emit("sequence-updated", state);
                            }
                        }
                    }
                }
                thread::sleep(Duration::from_millis(300));
            }
        });
    }

    pub fn trigger_paste(&self, app_state: AppState, app_handle: AppHandle) {
        if let Some(item) = app_state.advance_and_get_paste() {
            if let Ok(mut cb) = Clipboard::new() {
                let _ = cb.set_text(item.content.clone());
                *self.last_text.lock() = item.content.trim().to_string();
            }

            let state = app_state.get_state();
            let _ = app_handle.emit("sequence-updated", state);

            thread::spawn(move || {
                thread::sleep(Duration::from_millis(50));
                if let Ok(mut enigo) = Enigo::new(&Settings::default()) {
                    #[cfg(target_os = "macos")]
                    {
                        let _ = enigo.key(Key::Meta, Press);
                        let _ = enigo.key(Key::Unicode('v'), Click);
                        let _ = enigo.key(Key::Meta, Release);
                    }
                    #[cfg(not(target_os = "macos"))]
                    {
                        let _ = enigo.key(Key::Control, Press);
                        let _ = enigo.key(Key::Unicode('v'), Click);
                        let _ = enigo.key(Key::Control, Release);
                    }
                }
            });
        }
    }
}
