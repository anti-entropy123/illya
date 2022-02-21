use clap::{App, Arg};
use super::{Executable, Context};
use log::{error};
use std::{process};

pub fn subcommand<'a>() -> App<'a> {
    App::new("start")
        .about("start container")
        .version("0.1")
        .arg(Arg::new("container"))
}

#[derive(Debug)]
pub struct Command {
    container_id: String,
}

pub fn new (sub_matchs: &clap::ArgMatches) -> Box<dyn Executable> {
    let container_id: String;
    match sub_matchs.value_of("container") {
        Some(id) => {
            container_id = String::from(id);
        },
        None => {
            error!("no input container.");
            process::exit(1);
        }
    }
    Box::from(Command{
        container_id: container_id,
    })
}


impl Executable for Command {
    fn execute (&self,) {
    }
}