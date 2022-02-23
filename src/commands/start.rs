use {
    crate::commands::{Context, Executable},
    clap::{App, Arg},
    log::error,
    std::{fs, io::prelude::*, path},
};
pub fn subcommand<'a>() -> App<'a> {
    App::new("start")
        .about("start container")
        .version("0.1")
        .arg(Arg::new("container"))
}

#[derive(Debug)]
pub struct Command {
    container_id: String,
    container_rt_dir: String,
}

pub fn new(sub_matchs: &clap::ArgMatches, ctx: Context) -> Box<dyn Executable> {
    let container_id = sub_matchs
        .value_of("container")
        .expect("no input container.")
        .to_string();
    Box::from(Command {
        container_rt_dir: ctx.container_rt_dir(&container_id),
        container_id: container_id,
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
        send_byte_to_fifo(&self.container_rt_dir);
    }
}
