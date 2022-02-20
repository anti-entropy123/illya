use std::{
    process,
};
use log::{error};

pub trait Executable {
    fn execute(&self);
}

pub struct Context {
    pub root: String,
}

pub fn match_command(matchs: clap::ArgMatches) -> Box<dyn Executable> {
    let command = match matchs.subcommand() {
        Some(("delete", sub_matchs)) => {
            delete::Command::new(sub_matchs)
        },
        Some(("create", sub_matchs)) => {
            create::Command::new(sub_matchs)
        },
        Some(("init", sub_matchs)) => {
            init::Command::new(sub_matchs)
        },
        Some(("start", sub_matchs)) => {
            start::Command::new(sub_matchs)
        },
        Some(("spec", sub_matchs)) => {
            spec::Command::new(sub_matchs)
        },
        _ => {
           error!("unimplement subcommand: {:?}", matchs);
           process::exit(1);
        }
    };
    command
}

pub mod delete;
pub mod create;
pub mod init;
pub mod start;
pub mod spec;