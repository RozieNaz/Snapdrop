pub struct ScreenshotService;

impl ScreenshotService {
    pub fn new() -> Self {
        Self
    }
}

pub fn capture_fullscreen() -> Result<String, String> {
    Ok(String::from("Fullscreen capture requested"))
}
