use clap::{App};
use illya::commands::*;
use illya::log as logger;
use log::{debug, LevelFilter, error};
use std::env;
use std::path;
use std::process;

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
      ])
      .get_matches();

   match matchs.subcommand() {
      Some(("delete", sub_matchs)) => {
         delete::Command::new(sub_matchs).execute();
      },
      Some(("create", sub_matchs)) => {
         create::Command::new(sub_matchs).execute();
      },
      Some(("init", _)) => {
         init::Command::new().execute();
      },
      Some(("start", _)) => {
         start::Command::new().execute();
      },
      _ => {
         error!("no subcommand: {:?}", matchs);
      }
   }
}

fn set_log() {
   log::set_boxed_logger(Box::new(logger::MyLogger::new()))
      .expect("fail to init log");
   log::set_max_level(LevelFilter::Trace);
}

fn before_exec() {
   let _root: String;
   match env::var("XDG_RUNTIME_DIR") {
      Ok(mut v) if v != "" => {
         v.push(path::MAIN_SEPARATOR);
         _root = v + "illya";
      },
      _ => {
         error!("no XDG_RUNTIME_DIR");
         process::exit(1);
      }
   }

}

fn main() {
   set_log();
   debug!("{:?}", std::env::args());
   before_exec();
   execute();
}
