use crate::models::{ClipboardItem, SequenceState};
use chrono::Utc;
use parking_lot::Mutex;
use std::collections::HashSet;
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
        let requested_ids: HashSet<&str> = items.iter().map(|item| item.id.as_str()).collect();
        let current_ids: HashSet<&str> = state.items.iter().map(|item| item.id.as_str()).collect();
        if items.len() != state.items.len()
            || requested_ids.len() != items.len()
            || requested_ids != current_ids
        {
            return state.clone();
        }

        let old_active_id = state.items.get(state.current_index).map(|i| i.id.clone());
        let canonical_items = state.items.clone();
        state.items = items
            .iter()
            .filter_map(|requested| {
                canonical_items
                    .iter()
                    .find(|item| item.id == requested.id)
                    .cloned()
            })
            .collect();
        let items_len = state.items.len();

        // Synchronize history tail with the reordered items so subsequent copy events preserve order
        let history_len = state.history.len();
        if history_len >= items_len && items_len > 0 {
            let start_idx = history_len - items_len;
            let reordered_items = state.items.clone();
            for (i, item) in reordered_items.into_iter().enumerate() {
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

    pub fn delete_item(&self, id: String) -> SequenceState {
        let mut state = self.inner.lock();
        if let Some(pos) = state.items.iter().position(|i| i.id == id) {
            state.items.remove(pos);
            if state.items.is_empty() {
                state.current_index = 0;
            } else if pos < state.current_index {
                state.current_index -= 1;
            } else if state.current_index >= state.items.len() {
                state.current_index = 0;
            }
        }
        if let Some(h_pos) = state.history.iter().position(|i| i.id == id) {
            state.history.remove(h_pos);
        }
        state.clone()
    }

    pub fn update_item_content(&self, id: String, new_content: String) -> SequenceState {
        let mut state = self.inner.lock();
        if let Some(item) = state.items.iter_mut().find(|i| i.id == id) {
            item.content = new_content.clone();
        }
        if let Some(h_item) = state.history.iter_mut().find(|i| i.id == id) {
            h_item.content = new_content;
        }
        state.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(state: &AppState, content: &str) {
        assert!(state.record_copy(content.to_string()));
    }

    #[test]
    fn record_copy_preserves_whitespace_and_multiline_content() {
        let state = AppState::new();
        let content = "  indented\nsecond line\n";

        record(&state, content);

        let snapshot = state.get_state();
        assert_eq!(snapshot.items[0].content, content);
        assert_eq!(snapshot.history[0].content, content);
    }

    #[test]
    fn record_copy_only_rejects_an_exact_immediate_duplicate() {
        let state = AppState::new();

        record(&state, " value ");
        assert!(!state.record_copy(" value ".to_string()));
        record(&state, "value");
        record(&state, "other");
        record(&state, "value");

        let snapshot = state.get_state();
        assert_eq!(snapshot.history.len(), 4);
        assert_eq!(snapshot.history[0].content, " value ");
        assert_eq!(snapshot.items[0].content, "value");
        assert_eq!(snapshot.items[1].content, "other");
        assert_eq!(snapshot.items[2].content, "value");
        assert_eq!(snapshot.history[3].content, "value");
    }

    #[test]
    fn queue_tracks_latest_target_length_and_resets_index_on_copy() {
        let state = AppState::new();
        state.set_target_length(2);
        record(&state, "A");
        record(&state, "B");
        state.set_sequence_index(1);
        record(&state, "C");

        let snapshot = state.get_state();
        let contents: Vec<_> = snapshot
            .items
            .iter()
            .map(|item| item.content.as_str())
            .collect();
        assert_eq!(contents, vec!["B", "C"]);
        assert_eq!(snapshot.current_index, 0);
    }

    #[test]
    fn advance_cycles_in_queue_order() {
        let state = AppState::new();
        record(&state, "A");
        record(&state, "B");
        record(&state, "C");

        let pasted: Vec<_> = (0..4)
            .map(|_| state.advance_and_get_paste().unwrap().content)
            .collect();

        assert_eq!(pasted, vec!["A", "B", "C", "A"]);
        assert_eq!(state.get_state().current_index, 1);
    }

    #[test]
    fn reordering_preserves_the_active_item_and_future_queue_order() {
        let state = AppState::new();
        record(&state, "A");
        record(&state, "B");
        record(&state, "C");
        state.set_sequence_index(1);

        let mut reordered = state.get_state().items;
        reordered.swap(0, 2);
        let snapshot = state.set_items(reordered);

        let contents: Vec<_> = snapshot
            .items
            .iter()
            .map(|item| item.content.as_str())
            .collect();
        assert_eq!(contents, vec!["C", "B", "A"]);
        assert_eq!(snapshot.items[snapshot.current_index].content, "B");
        let history_tail: Vec<_> = snapshot
            .history
            .iter()
            .map(|item| item.content.as_str())
            .collect();
        assert_eq!(history_tail, vec!["C", "B", "A"]);
    }

    #[test]
    fn reordering_rejects_non_permutations_and_forged_content() {
        let state = AppState::new();
        record(&state, "A");
        record(&state, "B");
        let original = state.get_state();

        let mut duplicate = original.items.clone();
        duplicate[1] = duplicate[0].clone();
        assert_eq!(state.set_items(duplicate).items, original.items);

        let mut missing = original.items.clone();
        missing.pop();
        assert_eq!(state.set_items(missing).items, original.items);

        let mut forged = original.items.clone();
        forged.reverse();
        forged[0].content = "forged".to_string();
        let snapshot = state.set_items(forged);
        assert_eq!(snapshot.items[0].content, "B");
        assert_eq!(snapshot.items[1].content, "A");
        assert!(snapshot.items.iter().all(|item| item.content != "forged"));
        assert!(snapshot.history.iter().all(|item| item.content != "forged"));
    }

    #[test]
    fn history_is_capped_without_breaking_the_active_queue() {
        let state = AppState::new();
        for index in 0..101 {
            record(&state, &format!("item-{index}"));
        }

        let snapshot = state.get_state();
        assert_eq!(snapshot.history.len(), 100);
        assert_eq!(snapshot.history[0].content, "item-1");
        let contents: Vec<_> = snapshot
            .items
            .iter()
            .map(|item| item.content.as_str())
            .collect();
        assert_eq!(contents, vec!["item-98", "item-99", "item-100"]);
        assert_eq!(snapshot.current_index, 0);
    }

    #[test]
    fn deleting_items_keeps_current_index_valid() {
        let state = AppState::new();
        record(&state, "A");
        record(&state, "B");
        record(&state, "C");
        state.set_sequence_index(2);

        let a_id = state.get_state().items[0].id.clone();
        let snapshot = state.delete_item(a_id);
        assert_eq!(snapshot.current_index, 1);
        assert_eq!(snapshot.items[snapshot.current_index].content, "C");

        let c_id = snapshot.items[1].id.clone();
        let snapshot = state.delete_item(c_id);
        assert_eq!(snapshot.current_index, 0);
        assert_eq!(snapshot.items[0].content, "B");
    }

    #[test]
    fn editing_preserves_exact_content_in_queue_and_history() {
        let state = AppState::new();
        record(&state, "before");
        let id = state.get_state().items[0].id.clone();
        let content = "  after\nwith newline\n";

        let snapshot = state.update_item_content(id, content.to_string());

        assert_eq!(snapshot.items[0].content, content);
        assert_eq!(snapshot.history[0].content, content);
    }
}
