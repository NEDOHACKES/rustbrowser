use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

use crate::{
    config::Config,
    navigation::Url,
    tabs::TabManager,
    window::BrowserWindow,
};

pub struct Browser {
    config: Config,
}

impl Browser {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn run(self) {
        let event_loop = EventLoop::new().expect("failed to create event loop");

        let mut handler = BrowserHandler {
            state: BrowserState {
                config: self.config,
                window: None,
                tabs: {
                    let mut tabs = TabManager::new();
                    tabs.open(Url::parse("example.com"));
                    tabs
                },
            },
        };

        event_loop
            .run_app(&mut handler)
            .expect("browser event loop failed");
    }
}

struct BrowserState {
    config: Config,
    window: Option<BrowserWindow>,
    tabs: TabManager,
}

struct BrowserHandler {
    state: BrowserState,
}

impl ApplicationHandler for BrowserHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.window.is_none() {
            self.state.window = Some(
                BrowserWindow::create(
                    event_loop,
                    &self.state.config.window_title,
                    self.state.config.width,
                    self.state.config.height,
                )
                .expect("failed to create browser window"),
            );
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = self.state.window.as_ref() else {
            return;
        };

        if window.window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let _ = self.state.tabs.active();
            }
            _ => {}
        }
    }
}
