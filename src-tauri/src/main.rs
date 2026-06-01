mod capture;
mod history;
mod settings;
mod tray;

#[tauri::command]
fn app_status() -> String {
    String::from("Snapdrop backend is running")
}

#[tauri::command]
fn list_history() -> Vec<history::CaptureEntry> {
    history::empty_history()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![app_status, list_history])
        .setup(|app| {
            tray::setup_tray(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run Snapdrop");
}
