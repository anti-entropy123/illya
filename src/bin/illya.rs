use clap::{App,};
use illya::{
   commands,
   commands::*,
   log as logger,
};
use log::{debug, LevelFilter};

fn execute() {
   let matchs = App::new("illya")
      .version("0.1")
      .author("闹钟大魔王")
      .about("newer OCI runtime")
      .subcommands(vec![
         create::subcommand(),
         init::subcommand(),
         delete::subcommand(),
         start::subcommand(),
         spec::subcommand(),
      ])
      .get_matches();
   
   let subcommand = commands::match_command(matchs);
   subcommand.execute();
}

fn set_log() {
   log::set_boxed_logger(Box::new(logger::MyLogger::new()))
      .expect("fail to init log");
   log::set_max_level(LevelFilter::Trace);
}

// fn before_exec() -> Context {
//    let root: String;
//    match env::var("XDG_RUNTIME_DIR") {
//       Ok(mut v) if v != "" => {
//          v.push(path::MAIN_SEPARATOR);
//          root = v + "illya"; // "/run/user/1000/illya"
//       },
//       _ => {
//          error!("no XDG_RUNTIME_DIR");
//          process::exit(1);
//       }
//    }
//    Context{
//       root: root,
//    }
// }

fn main() {
   set_log();
   debug!("{:?}", std::env::args());
   execute();
}

