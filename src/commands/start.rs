use clap::{App, Arg};

pub fn subcommand<'a>() -> App<'a> {
    App::new("start")
        .about("start container")
        .version("0.1")
        .arg(Arg::new("containers"))
}

#[derive(Debug)]
pub struct Command {
}

impl Command {
    pub fn execute (&self,) {
    
    }

    pub fn new () -> Command {
        Command{}
    }
}