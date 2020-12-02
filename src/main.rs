mod problem0;
mod util;

use log::{debug, error, info, log_enabled, trace, warn, Level};

fn main() {
    env_logger::init();
    println!("Hello, world!");
    println!("Log Level {}", Level::max());
    println!("Log level {}", log_enabled!(Level::Info));
    println!("Log level {}", log_enabled!(Level::Debug));
    println!("Log level {}", log_enabled!(Level::Error));
    println!("Log level {}", log_enabled!(Level::Warn));
    println!("Log level {}", log_enabled!(Level::Trace));

    info!("[INFO] Level");
    warn!("[WARN] Level");
    error!("[ERROR] Level");
    debug!("[DEBUG] Level");
    trace!("[DEBUG] Level");
}
