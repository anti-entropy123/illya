use clap::{App, Arg};
use std::process;
use std::io::prelude::*;
use nix::sys::socket;
use std::os::unix::io::{FromRawFd};
use std::fs::File;
use log::{error};

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
        .arg(Arg::new("containers")
            .multiple_occurrences(true)
    )
}


#[derive(Debug)]
pub struct Command<'a> {
    pub container_ids: Vec<&'a str>,
    pub pid_file: &'a str,
    pub bundle: &'a str,
}


impl<'a> Command<'a> {
    pub fn execute (&self,) {
        let (init_p, init_c) = socket::socketpair(
                                socket::AddressFamily::Unix, 
                                socket::SockType::Stream, None, 
                                socket::SockFlag::empty()
                            ).expect("create sockpair fail");

        let child = process::Command::new("/proc/self/exe")
                        .env("_LIBCONTAINER_INITPIPE", format!("{}", init_c))
                        .env("_CONTAINER_BUNDLE", self.bundle)
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

    pub fn new (sub_matchs: &'a clap::ArgMatches) -> Command<'a> {
        let mut command = Command{
            container_ids: vec![],
            pid_file: "",
            bundle: ""
        };

        if let Some(pid_file) = sub_matchs.value_of("pid-file") {
            command.pid_file = pid_file
        } else {
            error!("no pid-file");
            process::exit(1);
        }

        if let Some(bundle) = sub_matchs.value_of("bundle") {
            command.bundle = bundle
        } else {
            error!("no bundle");
            process::exit(1);
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