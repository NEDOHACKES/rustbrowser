use std::sync::Arc;
use winit::{
    dpi::LogicalSize,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};

pub struct BrowserWindow {
    pub window: Arc<Window>,
}

impl BrowserWindow {
    pub fn create(
        event_loop: &ActiveEventLoop,
        title: &str,
        width: f64,
        height: f64,
    ) -> Result<Self, winit::error::OsError> {
        let attributes = WindowAttributes::default()
            .with_title(title)
            .with_inner_size(LogicalSize::new(width, height));

        let window = event_loop.create_window(attributes)?;
        Ok(Self {
            window: Arc::new(window),
        })
    }
}
