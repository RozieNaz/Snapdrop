pub fn open_file(path: &str) -> Result<(), String> {
    open::that(path)
        .map(|_| ())
        .map_err(|error| format!("Failed to open file: {error}"))
}
