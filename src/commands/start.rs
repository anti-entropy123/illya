use {
    crate::{
        commands::{Context, Executable},
        container,
    },
    clap::{App, Arg},
    std::{fs, io::prelude::*, path},
};
pub fn subcommand<'a>() -> App<'a> {
    App::new("start")
        .about("start container")
        .version("0.1")
        .arg(Arg::new("container"))
}

pub struct Command {
    container: container::Container,
}

pub fn new(sub_matchs: &clap::ArgMatches, ctx: Context) -> Box<dyn Executable> {
    let container_id = sub_matchs
        .value_of("container")
        .expect("no input container.")
        .to_string();
    Box::from(Command {
        container: container::Container::new(&container_id, Box::new(ctx)),
    })
}

fn send_byte_to_fifo(crt_dir: &String) {
    let exec_fifo = path::Path::new(crt_dir).join("exec.fifo");
    let mut fifo = fs::File::options()
        .write(true)
        .open(exec_fifo)
        .expect("failed to open exec.fifo");
    fifo.write(&[0; 1]).expect("failed to write exec.fifo");
}

impl Executable for Command {
    fn execute(&self) {
        send_byte_to_fifo(&self.container.crt_dir());
    }
}
