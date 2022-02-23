use chrono;
use log::{Metadata, Record};
use std::{fs, io::prelude::*};

pub struct MyLogger {
    log_file: String,
}

pub fn new() -> MyLogger {
    // let mut home = std::env::var("HOME").unwrap();
    let mut home = String::from("/home/yjn");
    home.push(std::path::MAIN_SEPARATOR);
    MyLogger {
        log_file: home + "illya_log",
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
            .open(&self.log_file)
            .unwrap();

        let msg = format!(
            "{}:{}:{} - {}\n",
            record.level(),
            record.target(),
            utc,
            record.args()
        );
        file.write(msg.as_bytes()).unwrap();
        print!("{}", record.args());
    }

    fn flush(&self) {}
}
