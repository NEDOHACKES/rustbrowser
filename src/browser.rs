use std::cell::RefCell;
use std::rc::Rc;

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
        let state = BrowserState {
            config: self.config,
            window: None,
            tabs: {
                let mut tabs = TabManager::new();
                tabs.open(Url::parse("example.com"));
                tabs
            },
        };

        let state = Rc::new(RefCell::new(state));
        let handler = BrowserHandler {
            state: Rc::clone(&state),
        };

        event_loop
            .run(handler)
            .expect("browser event loop failed");
    }
}

struct BrowserState {
    config: Config,
    window: Option<BrowserWindow>,
    tabs: TabManager,
}

struct BrowserHandler {
    state: Rc<RefCell<BrowserState>>,
}

impl ApplicationHandler for BrowserHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut state = self.state.borrow_mut();

        if state.window.is_none() {
            state.window = Some(
                BrowserWindow::create(
                    event_loop,
                    &state.config.window_title,
                    state.config.width,
                    state.config.height,
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
        let mut state = self.state.borrow_mut();

        let Some(window) = state.window.as_ref() else {
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
                let _ = state.tabs.active();
            }
            _ => {}
        }
    }
}
