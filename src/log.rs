pub use log::*;
use simple_logger;
use std::env;
use std::str::FromStr;

const DEFAULT_LOG_LEVEL: &str = "DEBUG";

pub fn init() {
    let log_level_str = env::var("LOG_LEVEL")
        .unwrap_or(DEFAULT_LOG_LEVEL.to_owned());
    
    if let Ok(log_level) = Level::from_str(&log_level_str) {
        simple_logger::init_with_level(log_level).ok();
    }
}
