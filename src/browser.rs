use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    window::{Window, WindowId},
};

use wry::{
    dpi::{LogicalPosition, LogicalSize},
    Rect, WebView, WebViewBuilder,
};

use crate::config::Config;

const TOOLBAR_HEIGHT: u32 = 58;
const HOME_URL: &str = "https://duckduckgo.com";

#[derive(Debug)]
enum BrowserEvent {
    Navigate(String),
    Back,
    Forward,
    Reload,
    UrlChanged(String),
}

pub struct Browser {
    config: Config,
}

impl Browser {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn run(self) {
        let event_loop = EventLoop::<BrowserEvent>::with_user_event()
            .build()
            .expect("failed to create event loop");

        let proxy = event_loop.create_proxy();

        let mut app = BrowserApp {
            config: self.config,
            window: None,
            ui: None,
            page: None,
            proxy,
        };

        event_loop
            .run_app(&mut app)
            .expect("browser event loop failed");
    }
}

struct BrowserApp {
    config: Config,
    window: Option<Window>,
    ui: Option<WebView>,
    page: Option<WebView>,
    proxy: EventLoopProxy<BrowserEvent>,
}

impl ApplicationHandler<BrowserEvent> for BrowserApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title(&self.config.window_title)
                    .with_inner_size(LogicalSize::new(
                        self.config.width,
                        self.config.height,
                    )),
            )
            .expect("failed to create browser window");

        let proxy = self.proxy.clone();

        let ui = WebViewBuilder::new()
            .with_html(browser_ui_html())
            .with_ipc_handler(move |request| {
                let message = request.body().clone();

                if let Some(url) = message.strip_prefix("navigate:") {
                    let _ = proxy.send_event(BrowserEvent::Navigate(url.to_string()));
                } else {
                    match message.as_str() {
                        "back" => {
                            let _ = proxy.send_event(BrowserEvent::Back);
                        }
                        "forward" => {
                            let _ = proxy.send_event(BrowserEvent::Forward);
                        }
                        "reload" => {
                            let _ = proxy.send_event(BrowserEvent::Reload);
                        }
                        _ => {}
                    }
                }
            })
            .with_devtools(true)
            .with_focused(false)
            .with_bounds(toolbar_bounds(&window))
            .build_as_child(&window)
            .expect("failed to create browser toolbar");

        let proxy = self.proxy.clone();

        let mut page_builder = WebViewBuilder::new()
            .with_url(HOME_URL)
            .with_navigation_handler(move |url| {
                let _ = proxy.send_event(BrowserEvent::UrlChanged(url));
                true
            })
            .with_devtools(true);

        if let Some(proxy_url) = self.config.proxy.as_deref() {
            page_builder = page_builder.with_additional_browser_args(
                format!("--proxy-server={proxy_url} --disable-features=msSmartScreenProtection"),
            );
        }

        let page = page_builder
            .with_bounds(page_bounds(&window))
            .build_as_child(&window)
            .expect("failed to create web page view");

        self.window = Some(window);
        self.ui = Some(ui);
        self.page = Some(page);
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: BrowserEvent) {
        match event {
            BrowserEvent::Navigate(input) => {
                let url = normalize_url(&input);

                if let Some(page) = self.page.as_ref() {
                    if let Err(error) = page.load_url(&url) {
                        eprintln!("navigation failed: {error}");
                        return;
                    }
                }

                self.update_address_bar(&url);
            }
            BrowserEvent::Back => {
                if let Some(page) = self.page.as_ref() {
                    let _ = page.go_back();
                }
            }
            BrowserEvent::Forward => {
                if let Some(page) = self.page.as_ref() {
                    let _ = page.go_forward();
                }
            }
            BrowserEvent::Reload => {
                if let Some(page) = self.page.as_ref() {
                    let _ = page.reload();
                }
            }
            BrowserEvent::UrlChanged(url) => {
                self.update_address_bar(&url);
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = self.window.as_ref() else {
            return;
        };

        if window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(_) => {
                let _ = self.resize_webviews();
            }
            _ => {}
        }
    }
}

impl BrowserApp {
    fn update_address_bar(&self, url: &str) {
        let Some(ui) = self.ui.as_ref() else {
            return;
        };

        let value = serde_json::to_string(url).unwrap_or_else(|_| "\"\"".to_string());
        let script = format!(
            "document.getElementById('address').value = {}; document.getElementById('status').textContent = 'Ready';",
            value
        );

        let _ = ui.evaluate_script(&script);
    }

    fn resize_webviews(&self) -> Result<(), wry::Error> {
        let Some(window) = self.window.as_ref() else {
            return Ok(());
        };

        if let Some(ui) = self.ui.as_ref() {
            ui.set_bounds(toolbar_bounds(window))?;
        }

        if let Some(page) = self.page.as_ref() {
            page.set_bounds(page_bounds(window))?;
        }

        Ok(())
    }
}

fn normalize_url(input: &str) -> String {
    let value = input.trim();

    if value.is_empty() {
        return HOME_URL.to_string();
    }

    if value.starts_with("http://")
        || value.starts_with("https://")
        || value.starts_with("about:")
    {
        return value.to_string();
    }

    if value.contains(' ') || !value.contains('.') {
        return format!("https://www.google.com/search?q={}", urlencoding(value));
    }

    format!("https://{value}")
}

fn urlencoding(value: &str) -> String {
    value
        .bytes()
        .map(|byte| match byte {
            b'A'..=b'Z'
            | b'a'..=b'z'
            | b'0'..=b'9'
            | b'-'
            | b'_'
            | b'.'
            | b'~' => (byte as char).to_string(),
            _ => format!("%{byte:02X}"),
        })
        .collect()
}

fn toolbar_bounds(window: &Window) -> Rect {
    let size = window.inner_size().to_logical::<u32>(window.scale_factor());

    Rect {
        position: LogicalPosition::new(0, 0).into(),
        size: LogicalSize::new(size.width, TOOLBAR_HEIGHT).into(),
    }
}

fn page_bounds(window: &Window) -> Rect {
    let size = window.inner_size().to_logical::<u32>(window.scale_factor());
    let page_height = size.height.saturating_sub(TOOLBAR_HEIGHT);

    Rect {
        position: LogicalPosition::new(0, TOOLBAR_HEIGHT).into(),
        size: LogicalSize::new(size.width, page_height).into(),
    }
}

fn browser_ui_html() -> String {
    r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<style>
* { box-sizing: border-box; }
html, body {
    margin: 0;
    width: 100%;
    height: 100%;
    overflow: hidden;
    font-family: "Segoe UI", Arial, sans-serif;
    background: #15171a;
    color: #e8eaed;
}
body {
    display: flex;
    align-items: center;
    padding: 8px;
    gap: 6px;
}
button {
    border: 0;
    background: #22262b;
    color: #e8eaed;
    width: 38px;
    height: 42px;
    border-radius: 9px;
    font-size: 18px;
    cursor: pointer;
}
button:hover { background: #30353b; }
button:active { background: #3a4149; }
#address {
    flex: 1;
    height: 42px;
    min-width: 100px;
    border: 1px solid #353a41;
    border-radius: 12px;
    background: #202328;
    color: #f5f7fa;
    outline: none;
    padding: 0 15px;
    font-size: 15px;
}
#address:focus {
    border-color: #5d8cff;
    background: #25292f;
}
#status {
    width: 62px;
    text-align: center;
    color: #8f969f;
    font-size: 11px;
}
</style>
</head>
<body>
<button onclick="send('back')" title="Back">←</button>
<button onclick="send('forward')" title="Forward">→</button>
<button onclick="send('reload')" title="Reload">↻</button>
<input id="address" value="https://www.google.com" spellcheck="false"
       autocomplete="off" autocapitalize="off">
<span id="status">Ready</span>
<script>
const address = document.getElementById('address');

function send(message) {
    window.ipc.postMessage(message);
}

address.addEventListener('keydown', (event) => {
    if (event.key === 'Enter') {
        send('navigate:' + address.value);
        address.blur();
    }
});
</script>
</body>
</html>"#
        .to_string()
}
