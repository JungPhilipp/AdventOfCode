use log::debug;
use std::fs::File;
use std::io::{self, BufRead};

pub fn lines_iter(path_to_file: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path_to_file)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_lines(path: &str) -> Vec<String> {
    debug!("Attempting to read file {}", path);
    let lines = lines_iter(path).expect("Error reading file");
    let file_content = lines.map(|line| line.unwrap()).collect();
    debug!("Contents: {:?}", file_content);
    return file_content;
}

pub fn parse_to<T>(path: &str) -> Vec<T>
where
    T: std::str::FromStr + std::fmt::Debug,
    T::Err: std::fmt::Debug,
{
    debug!("Attempting to read file {}", path);
    let lines = lines_iter(path).expect("Error reading file");
    let file_content = lines
        .map(|line| line.unwrap().parse::<T>().unwrap())
        .collect();
    debug!("Contents: {:?}", file_content);
    return file_content;
}
