#[derive(Debug, Clone)]
pub struct AppSettings {
    pub save_directory: String,
    pub auto_copy: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            save_directory: String::from("Screenshots"),
            auto_copy: true,
        }
    }
}
