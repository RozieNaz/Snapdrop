use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureEntry {
    pub file_name: String,
    pub created_at: String,
    pub file_path: String,
}

pub fn empty_history() -> Vec<CaptureEntry> {
    Vec::new()
}
