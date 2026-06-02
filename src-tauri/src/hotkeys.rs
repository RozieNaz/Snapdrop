use crate::capture;
use tauri::{App, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

pub const FULLSCREEN_SHORTCUT: &str = "Ctrl+Shift+4";

pub fn shortcut_description() -> String {
    format!("Fullscreen capture: {}", FULLSCREEN_SHORTCUT)
}

pub fn register(app: &mut App) -> Result<(), String> {
    let shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::Digit4);

    app.handle()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |_app, pressed, event| {
                    if pressed == &shortcut && event.state() == ShortcutState::Pressed {
                        let _ = capture::capture_fullscreen();
                    }
                })
                .build(),
        )
        .map_err(|error| format!("Failed to initialise global shortcut plugin: {error}"))?;

    app.global_shortcut()
        .register(shortcut)
        .map_err(|error| format!("Failed to register global shortcut: {error}"))?;

    Ok(())
}
