use arboard::Clipboard;

pub fn copy_text(text: &str) -> Result<(), String> {
    let mut clipboard = Clipboard::new()
        .map_err(|error| format!("Failed to access clipboard: {error}"))?;

    clipboard
        .set_text(text)
        .map_err(|error| format!("Failed to update clipboard: {error}"))
}
