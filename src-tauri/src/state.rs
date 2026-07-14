use crate::models::{ClipboardItem, SequenceState};
use chrono::Utc;
use parking_lot::Mutex;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<Mutex<SequenceState>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(SequenceState::default())),
        }
    }

    pub fn get_state(&self) -> SequenceState {
        self.inner.lock().clone()
    }

    pub fn record_copy(&self, text: String) -> bool {
        let mut state = self.inner.lock();
        if !state.is_enabled {
            return false;
        }

        // Avoid duplicate immediate re-record
        if let Some(last) = state.history.last() {
            if last.content == text {
                return false;
            }
        }

        let item = ClipboardItem {
            id: Uuid::new_v4().to_string(),
            content: text,
            copied_at: Utc::now(),
        };

        state.history.push(item.clone());
        if state.history.len() > 100 {
            state.history.remove(0);
        }

        // Update active sequence queue (take the last `target_length` items)
        let target_len = state.target_length;
        let history_len = state.history.len();
        if history_len > 0 {
            let start_idx = if history_len > target_len {
                history_len - target_len
            } else {
                0
            };
            state.items = state.history[start_idx..].to_vec();
            // Reset index to 0 when queue updates so user starts from first item A
            state.current_index = 0;
        }

        true
    }

    pub fn advance_and_get_paste(&self) -> Option<ClipboardItem> {
        let mut state = self.inner.lock();
        if state.items.is_empty() {
            return None;
        }

        let idx = state.current_index;
        let item = state.items.get(idx).cloned();

        if !state.items.is_empty() {
            state.current_index = (state.current_index + 1) % state.items.len();
        }

        item
    }

    pub fn reset_index(&self) {
        let mut state = self.inner.lock();
        state.current_index = 0;
    }

    pub fn set_sequence_index(&self, index: usize) -> SequenceState {
        let mut state = self.inner.lock();
        if !state.items.is_empty() {
            state.current_index = index % state.items.len();
        } else {
            state.current_index = 0;
        }
        state.clone()
    }

    pub fn set_target_length(&self, length: usize) -> SequenceState {
        let mut state = self.inner.lock();
        state.target_length = length.max(1);
        let history_len = state.history.len();
        if history_len > 0 {
            let start_idx = if history_len > state.target_length {
                history_len - state.target_length
            } else {
                0
            };
            state.items = state.history[start_idx..].to_vec();
        }
        state.current_index = 0;
        state.clone()
    }

    pub fn set_items(&self, items: Vec<ClipboardItem>) -> SequenceState {
        let mut state = self.inner.lock();
        let old_active_id = state.items.get(state.current_index).map(|i| i.id.clone());
        let items_len = items.len();
        state.items = items.clone();

        // Synchronize history tail with the reordered items so subsequent copy events preserve order
        let history_len = state.history.len();
        if history_len >= items_len && items_len > 0 {
            let start_idx = history_len - items_len;
            for (i, item) in items.into_iter().enumerate() {
                state.history[start_idx + i] = item;
            }
        } else {
            state.history = state.items.clone();
        }

        if let Some(id) = old_active_id {
            if let Some(pos) = state.items.iter().position(|i| i.id == id) {
                state.current_index = pos;
            } else {
                state.current_index = 0;
            }
        } else {
            state.current_index = 0;
        }

        state.clone()
    }

    pub fn clear_all(&self) -> SequenceState {
        let mut state = self.inner.lock();
        state.history.clear();
        state.items.clear();
        state.current_index = 0;
        state.clone()
    }
}
