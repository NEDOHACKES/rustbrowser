mod browser;
mod config;

use browser::Browser;
use config::Config;

fn main() {
    Browser::new(Config::default()).run();
}
