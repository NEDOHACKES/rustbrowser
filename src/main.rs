mod browser;
mod config;
mod navigation;
mod tabs;
mod window;

use browser::Browser;
use config::Config;

fn main() {
    let config = Config::default();
    let browser = Browser::new(config);
    browser.run();
}
