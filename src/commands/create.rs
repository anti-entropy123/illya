use clap::{App, Arg};
use std::{
    process,
    io::prelude::*,
    os::unix::io::{FromRawFd},
    fs::File
};
use nix::sys::socket;
use log::{error};
use super::{Executable, Context};

pub fn subcommand<'a>() -> App<'a> {
    App::new("create")
        .about("create container")
        .version("0.1")
        .arg(Arg::new("bundle")
            .help("path to the root of the bundle directory, defaults to the current directory")
            .takes_value(true)
            .long("bundle"))
        .arg(Arg::new("pid-file")
            .help("filename that the container pid will be written to")
            .takes_value(true)
            .long("pid-file"))
        .arg(Arg::new("container")
            .multiple_occurrences(false)
    )
}


#[derive(Debug)]
pub struct Command {
    pub container_id: String,
    pub pid_file: String,
    pub bundle: String,
}

impl Command {
    pub fn new (sub_matchs: &clap::ArgMatches) -> Box<dyn Executable> {
        let pidfile: String;
        if let Some(pid_file) = sub_matchs.value_of("pid-file") {
            pidfile = String::from(pid_file);
        } else {
            error!("no pid-file");
            process::exit(1);
        }

        let bundle: String;
        if let Some(b) = sub_matchs.value_of("bundle") {
            bundle = String::from(b);
        } else {
            error!("no bundle");
            process::exit(1);
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
            bundle: bundle,
            pid_file: pidfile,
        })
    }
}

impl Executable for Command {
    fn execute (&self,) {
        let (init_p, init_c) = socket::socketpair(
                                socket::AddressFamily::Unix, 
                                socket::SockType::Stream, None, 
                                socket::SockFlag::empty()
                            ).expect("create sockpair fail");

        let child = process::Command::new("/proc/self/exe")
                        .env("_LIBCONTAINER_INITPIPE", format!("{}", init_c))
                        .env("_CONTAINER_BUNDLE", &self.bundle)
                        .arg("init")
                        .spawn();

        let mut init_pipe = unsafe { File::from_raw_fd(init_p)};
        write!(init_pipe, "{}", format!("{}", self.pid_file)).unwrap();
        // let mut output = String::new();
        match child {
            Err(why) => panic!("couldn't spawn: {}", why),
            Ok(mut child) => {
                child.wait().expect("fail to wait child exit");
                // debug!("child exit with {}", code);
            },
        };
    }
}