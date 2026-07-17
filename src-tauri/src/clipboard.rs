use crate::state::AppState;
use arboard::Clipboard;
use parking_lot::Mutex;
use std::panic;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
#[cfg(target_os = "macos")]
use std::time::Instant;
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
    paste_lock: Arc<Mutex<()>>,
}

/// Polls the combined hardware modifier state until Ctrl/Option/Shift/Cmd are all
/// released, or the timeout elapses. A synthetic Cmd+V posted while the physical
/// shortcut chord (Ctrl+Option) is still held gets merged with the hardware
/// modifier state, so the target app sees Cmd+Ctrl+Option+V and ignores it.
#[cfg(target_os = "macos")]
fn wait_for_modifiers_released(timeout: Duration) {
    let modifier_mask = CGEventFlags::CGEventFlagControl
        | CGEventFlags::CGEventFlagAlternate
        | CGEventFlags::CGEventFlagShift
        | CGEventFlags::CGEventFlagCommand;
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        let held = CGEventSource::new(CGEventSourceStateID::HIDSystemState)
            .ok()
            .and_then(|source| CGEvent::new(source).ok())
            .map(|event| event.get_flags().intersects(modifier_mask))
            .unwrap_or(false);
        if !held {
            return;
        }
        thread::sleep(Duration::from_millis(10));
    }
}

impl ClipboardMonitor {
    pub fn new() -> Self {
        Self {
            last_text: Arc::new(Mutex::new(String::new())),
            paste_lock: Arc::new(Mutex::new(())),
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
            // Emit updated sequence to UI
            let state = app_state.get_state();
            let _ = app_handle.emit("sequence-updated", state);

            let last_text = Arc::clone(&self.last_text);
            let paste_lock = Arc::clone(&self.paste_lock);

            // Execute clipboard write + keyboard paste in background thread
            thread::spawn(move || {
                // Serialize pastes: each clipboard write must land together with its
                // own keystroke, otherwise rapid shortcut presses overwrite the
                // clipboard before the previous synthetic paste fires, pasting the
                // same (latest) content multiple times and dropping earlier items.
                let _guard = paste_lock.lock();

                #[cfg(target_os = "macos")]
                wait_for_modifiers_released(Duration::from_millis(600));
                #[cfg(not(target_os = "macos"))]
                thread::sleep(Duration::from_millis(100));

                // Update last_text before the clipboard so the polling monitor never
                // observes the pasted content ahead of the dedup marker.
                *last_text.lock() = item.content.clone();
                if let Ok(mut cb) = Clipboard::new() {
                    let _ = cb.set_text(item.content.clone());
                }
                // Give the pasteboard a moment to settle before the keystroke lands.
                thread::sleep(Duration::from_millis(30));

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
                            // The physical Ctrl+Alt+V chord may still be held; a held
                            // Alt turns the synthetic Ctrl+V into Ctrl+Alt+V, which
                            // target apps ignore. Neutralize Alt/Shift first.
                            let _ = enigo.key(Key::Alt, Release);
                            let _ = enigo.key(Key::Shift, Release);
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

                // Hold the lock a beat longer so the target app reads the clipboard
                // before the next queued paste overwrites it.
                thread::sleep(Duration::from_millis(120));
            });
        }
    }
}
