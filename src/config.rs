#[derive(Debug, Clone)]
pub struct Config {
    pub window_title: String,
    pub width: f64,
    pub height: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_title: "RustBrowser".to_string(),
            width: 1280.0,
            height: 800.0,
        }
    }
}
