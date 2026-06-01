mod capture;
mod clipboard;
mod history;
mod settings;
mod tray;

#[tauri::command]
fn app_status() -> String {
    String::from("Snapdrop backend is running")
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            app_status,
            list_history,
            capture_fullscreen,
            copy_path_to_clipboard,
        ])
        .setup(|app| {
            tray::setup_tray(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run Snapdrop");
}
