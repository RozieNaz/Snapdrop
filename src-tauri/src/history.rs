use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureEntry {
    pub file_name: String,
    pub created_at: String,
    pub file_path: String,
}

pub fn list_history() -> Result<Vec<CaptureEntry>, String> {
    let path = history_file()?;
    if !path.exists() {
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(&path)
        .map_err(|error| format!("Failed to read history file: {error}"))?;

    serde_json::from_str(&contents)
        .map_err(|error| format!("Failed to parse history file: {error}"))
}

pub fn add_entry(entry: CaptureEntry) -> Result<(), String> {
    let mut history = list_history()?;
    history.insert(0, entry);
    history.truncate(100);

    let path = history_file()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("Failed to create history folder: {error}"))?;
    }

    let json = serde_json::to_string_pretty(&history)
        .map_err(|error| format!("Failed to serialise history: {error}"))?;

    fs::write(&path, json)
        .map_err(|error| format!("Failed to write history file: {error}"))
}

fn history_file() -> Result<PathBuf, String> {
    let base = dirs::data_local_dir()
        .ok_or_else(|| String::from("Could not locate the local app-data folder"))?;

    Ok(base.join("Snapdrop").join("history.json"))
}
