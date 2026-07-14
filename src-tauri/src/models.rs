use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClipboardItem {
    pub id: String,
    pub content: String,
    pub copied_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceState {
    pub target_length: usize,
    pub current_index: usize,
    pub items: Vec<ClipboardItem>,
    pub history: Vec<ClipboardItem>,
    pub shortcut: String,
    pub is_enabled: bool,
}

impl Default for SequenceState {
    fn default() -> Self {
        Self {
            target_length: 3,
            current_index: 0,
            items: Vec::new(),
            history: Vec::new(),
            shortcut: "Ctrl+Option+V".to_string(),
            is_enabled: true,
        }
    }
}
