
// Std imports
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::Write;
// External imports
use simplelog::{WriteLogger, Config};
use log::LevelFilter;

const LOG_PATH: &str = "./oxide.log";

fn get_log_file() -> File {
    let path = Path::new(LOG_PATH);
    let file_result = OpenOptions::new().append(true).create(true).read(false).open(path);
    if file_result.is_err() {
        panic!("Failed to open log file: {}", file_result.err().unwrap())
    }

    let mut file = file_result.unwrap();
    file.write(b"\n").expect("Failed to write to log file");

    file
}

pub fn init_logging() {
    let file = get_log_file();

    let result = WriteLogger::init(LevelFilter::Trace, Config::default(), file);
    if result.is_err() {
        panic!("Failed to create log instance: {}", result.err().unwrap());
    }
}
