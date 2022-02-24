use {
    crate::utils,
    log::{Metadata, Record},
    std::{fs, io::prelude::*},
};

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
        let utc = utils::now_utc();
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
        print!("{} {}", record.level(), record.args());
    }

    fn flush(&self) {}
}
