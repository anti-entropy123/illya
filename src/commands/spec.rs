use clap::{App, Arg};
use super::{Executable, Context};
use std::env;

pub fn subcommand<'a>() -> App<'a> {
    App::new("spec")
        .about("create a new specification file")
        .version("0.1")
        .arg(Arg::new("bundle")
            .help("path to the root of the bundle directory")
            .takes_value(true)
            .long("bundle")
            .short('b'))
}

#[derive(Debug)]
pub struct Command {
    bundle: String
}

pub fn new (sub_matchs: &clap::ArgMatches) -> Box<dyn Executable> {
    let bundle = if let Some(v) = sub_matchs.value_of("bundle") {
        String::from(v)
    } else {
        String::from(String::from(env::var("PWD").unwrap()))
    };
    Box::from(Command{
        bundle: String::from(bundle),
    })
}


impl Executable for Command {
    fn execute (&self,) {
    }
}