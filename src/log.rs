use std::{
    io::prelude::*,
    fs
};
use log::{Record, Metadata};
use chrono;

pub struct MyLogger {
}

impl MyLogger {
    pub fn new() -> MyLogger{
        MyLogger {}
    }
}

impl log::Log for MyLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let utc = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let mut file = fs::OpenOptions::new()
                        .append(true)
                        .open(std::env::var("HOME").unwrap() + "/illya_log").unwrap();

        file.write(format!("{}:{}:{} - {}\n", 
            record.level(), 
            record.target(), 
            utc,
            record.args()
        ).as_bytes()).unwrap();
    }

    fn flush(&self) {}
}
