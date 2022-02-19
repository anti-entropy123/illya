// #[macro_use]
// extern crate log;
use std::fs;
use std::io::prelude::*;
use log::{Record, Metadata};

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
        let mut file = fs::OpenOptions::new()
                        .append(true)
                        .open("/home/yjn/illya_log").unwrap();

        file.write(format!("{}:{} - {}", record.level(), record.target(), record.args()).as_bytes()).unwrap();
    }

    fn flush(&self) {}
}
