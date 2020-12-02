use log::info;
use std::fs;

pub fn parse_lines(path: &str) -> String {
    info!("Attempting to read file {}", path);
    let content = fs::read_to_string(path).expect("Failed to read file");
    info!("Contents: {}", content);
    return content;
}
