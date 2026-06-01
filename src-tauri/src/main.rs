mod capture;
mod clipboard;
mod files;
mod history;
mod hotkeys;
mod settings;
mod tray;

#[tauri::command]
fn app_status() -> String {
    format!("Snapdrop backend is running | {}", hotkeys::shortcut_description())
}

#[tauri::command]
fn list_history() -> Result<Vec<history::CaptureEntry>, String> {
    history::list_history()
}

#[tauri::command]
fn capture_fullscreen() -> Result<history::CaptureEntry, String> {
    capture::capture_fullscreen()
}

#[tauri::command]
fn copy_path_to_clipboard(path: String) -> Result<(), String> {
    clipboard::copy_text(&path)
}

#[tauri::command]
fn open_capture(path: String) -> Result<(), String> {
    files::open_file(&path)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            app_status,
            list_history,
            capture_fullscreen,
            copy_path_to_clipboard,
            open_capture,
        ])
        .setup(|app| {
            tray::setup_tray(app)?;
            hotkeys::register(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run Snapdrop");
}
