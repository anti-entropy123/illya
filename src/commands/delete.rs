extern crate clap;
use clap::{App, Arg};
use std::process;
use log::{error};

pub fn subcommand<'a>() -> App<'a> {
    App::new("delete")
        .about("delete container")
        .version("0.1")
        .arg(Arg::new("force")
            .help("force mode")
            .long("force")
            .short('f')
            .takes_value(false))
        .arg(Arg::new("containers")
            .help("containers id need deleted")
            .multiple_occurrences(true))
            // .required(true))
}

#[derive(Debug)]
pub struct Command<'a> {
    pub container_ids: Vec<&'a str>,
    pub force: bool,
}

impl<'a> Command<'a> {
    pub fn execute (&self,) {
    }

    pub fn new (sub_matchs: &'a clap::ArgMatches) -> Command<'a> {
        let mut command = Command{
            container_ids: vec![],
            force: false,
        };
        if sub_matchs.is_present("force") {
            command.force = true;
        }
        match sub_matchs.values_of("containers") {
            Some(ids) => {
                let ids: Vec<_> = ids.collect();
                command.container_ids = ids;
            },
            None => {
                error!("no input containers.");
                process::exit(1);
            }
        }
        command
    }
}