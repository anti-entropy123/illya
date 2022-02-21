extern crate clap;
use clap::{App, Arg};
use std::process;
use log::{error};
use super::{Executable, Context};

pub fn subcommand<'a>() -> App<'a> {
    App::new("delete")
        .about("delete container")
        .version("0.1")
        .arg(Arg::new("force")
            .help("force mode")
            .long("force")
            .short('f')
            .takes_value(false))
        .arg(Arg::new("container")
            .help("containers id need deleted")
            .multiple_occurrences(false))
            // .required(true))
}

#[derive(Debug)]
pub struct Command {
    pub container_id: String,
    pub force: bool,
}

pub fn new (sub_matchs: &clap::ArgMatches) -> Box<dyn Executable> {
    let force: bool;
    if sub_matchs.is_present("force") {
        force = true;
    }
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
        force: false,
    })
}

impl Executable for Command {
    fn execute (&self,) {
    }
}