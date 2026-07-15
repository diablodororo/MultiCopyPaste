use crate::state::AppState;
use arboard::Clipboard;
use parking_lot::Mutex;
use std::panic;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

#[cfg(target_os = "macos")]
use core_graphics::event::{CGEvent, CGEventFlags, CGEventTapLocation, CGKeyCode};
#[cfg(target_os = "macos")]
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

#[cfg(not(target_os = "macos"))]
use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};

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
            // Update system clipboard
            if let Ok(mut cb) = Clipboard::new() {
                let _ = cb.set_text(item.content.clone());
                *self.last_text.lock() = item.content.clone();
            }

            // Emit updated sequence to UI
            let state = app_state.get_state();
            let _ = app_handle.emit("sequence-updated", state);

            // Execute keyboard paste in background thread with catch_unwind
            thread::spawn(move || {
                // Short sleep to allow physical modifier keys to be released
                thread::sleep(Duration::from_millis(100));

                let _ = panic::catch_unwind(|| {
                    #[cfg(target_os = "macos")]
                    {
                        if let Ok(source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
                            // Keycode 9 is 'v' on macOS standard keyboard layout
                            if let Ok(v_down) = CGEvent::new_keyboard_event(source.clone(), 9 as CGKeyCode, true) {
                                if let Ok(v_up) = CGEvent::new_keyboard_event(source, 9 as CGKeyCode, false) {
                                    v_down.set_flags(CGEventFlags::CGEventFlagCommand);
                                    v_up.set_flags(CGEventFlags::CGEventFlagCommand);

                                    v_down.post(CGEventTapLocation::HID);
                                    v_up.post(CGEventTapLocation::HID);
                                }
                            }
                        }
                    }

                    #[cfg(target_os = "windows")]
                    {
                        if let Ok(mut enigo) = Enigo::new(&Settings::default()) {
                            let _ = enigo.key(Key::Control, Press);
                            let _ = enigo.key(Key::V, Click);
                            let _ = enigo.key(Key::Control, Release);
                        }
                    }

                    #[cfg(all(unix, not(target_os = "macos")))]
                    {
                        if let Ok(mut enigo) = Enigo::new(&Settings::default()) {
                            let _ = enigo.key(Key::Control, Press);
                            let _ = enigo.key(Key::Unicode('v'), Click);
                            let _ = enigo.key(Key::Control, Release);
                        }
                    }
                });
            });
        }
    }
}
