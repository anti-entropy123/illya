use clap::{App, Arg};
use std::{
    process,
    io::prelude::*,
    os::unix::io::{FromRawFd},
    fs::File,
    path,
};
use path_filetype::*;
use nix::sys::socket;
use log::{error, info};
use super::{Executable, Context};
use crate::{
    utils, models::config as Config,
};
use serde_json::{Result as JsonResult};

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

pub fn new (sub_matchs: &clap::ArgMatches) -> Box<dyn Executable> {
    let pidfile: String;
    if let Some(pid_file) = sub_matchs.value_of("pid-file") {
        pidfile = String::from(pid_file);
    } else {
        error!("no pid-file");
        process::exit(1);
    }

    let bundle = if let Some(b) = sub_matchs.value_of("bundle") {b} else {"."};
    let mut bundle = utils::abs_path(bundle).unwrap();
    if match path::Path::new(&bundle).filetype() {
        Ok(filetype) if filetype != FileType::Directory => {true},
        Err(e) => {true},
        _ => {false}
    } {
        error!("{} is not exist or is not directory", bundle);
        process::exit(1);
    }

    bundle = utils::last_must_separator(bundle);

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

fn loadOCIConfig(bundle: &String) -> JsonResult<Config::OCIConfig> {
    let config_path = String::from(bundle) + "config.json";
    let mut config_file = File::open(config_path)
                            .expect(format!("can't open config.json in {}", bundle).as_str());
    
    let mut config: String = String::new();
    config_file.read_to_string(&mut config).expect("read config.json fail");
    let config: Config::OCIConfig = serde_json::from_str(&config)?;
    Ok(config)
}

impl Command {
    fn run_init_and_wait(&self,) {
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


impl Executable for Command {
    fn execute (&self,) {
        let oci_config = loadOCIConfig(&self.bundle).expect("parse config.json fail");
        info!("oci config is {:?}", oci_config);
        self.run_init_and_wait();
    }
}