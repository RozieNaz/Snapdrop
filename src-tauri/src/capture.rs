use crate::history::{self, CaptureEntry};
use chrono::Local;
use screenshots::Screen;
use std::fs;
use std::path::PathBuf;

pub struct ScreenshotService;

impl ScreenshotService {
    pub fn new() -> Self {
        Self
    }
}

pub fn capture_fullscreen() -> Result<CaptureEntry, String> {
    let screens = Screen::all().map_err(|error| format!("Failed to list displays: {error}"))?;
    let screen = screens
        .first()
        .ok_or_else(|| String::from("No display was found"))?;

    let image = screen
        .capture()
        .map_err(|error| format!("Failed to capture display: {error}"))?;

    let directory = screenshots_directory()?;
    fs::create_dir_all(&directory)
        .map_err(|error| format!("Failed to create screenshot folder: {error}"))?;

    let created_at = Local::now();
    let file_name = format!("Snapdrop-{}.png", created_at.format("%Y-%m-%d_%H-%M-%S"));
    let file_path = directory.join(&file_name);

    image
        .save(&file_path)
        .map_err(|error| format!("Failed to save screenshot: {error}"))?;

    let entry = CaptureEntry {
        file_name,
        created_at: created_at.to_rfc3339(),
        file_path: file_path.to_string_lossy().into_owned(),
    };

    history::add_entry(entry.clone())?;
    Ok(entry)
}

fn screenshots_directory() -> Result<PathBuf, String> {
    let pictures = dirs::picture_dir()
        .or_else(dirs::document_dir)
        .ok_or_else(|| String::from("Could not locate a Pictures or Documents folder"))?;

    Ok(pictures.join("Snapdrop"))
}
