use clap::{App, Arg};
use std::{
    process,
    io::prelude::*,
    os::unix::io::{FromRawFd},
    fs::File,
};
use nix::sys::socket;
use log::{error, info};
use crate::{
    utils, 
    models::{oci, init},
    commands::Executable
};
use serde_json::{Result as JsonResult, ser};

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
            .required(true)
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
    if !utils::is_directory(&bundle) {
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

fn load_oci_config(bundle: &String) -> JsonResult<oci::Config> {
    let config_path = String::from(bundle) + "config.json";
    let mut config_file = File::open(config_path)
                            .expect(format!("can't open config.json in {}", bundle).as_str());
    
    let mut config: String = String::new();
    config_file.read_to_string(&mut config).expect("read config.json fail");
    let config: oci::Config = serde_json::from_str(&config)?;
    Ok(config)
}

impl Command {
    fn run_init_and_wait(&self, init_fd_c: i32) {
        let child = process::Command::new("/proc/self/exe")
                            .env("_LIBCONTAINER_INITPIPE", format!("{}", init_fd_c))
                            .arg("init")
                            .spawn();
        
        // let mut output = String::new();
        match child {
            Err(why) => panic!("couldn't spawn: {}", why),
            Ok(mut child) => {
                child.wait().expect("fail to wait child exit");
                // debug!("child exit with {}", code);
            },
        };
    }

    fn make_init_param(&self, oci_config: oci::Config) -> init::Parameter {
        init::Parameter{
            root_path: oci_config.root.path,
            args: oci_config.process.args,
            bundle: self.bundle.clone(),
            pid_file: self.pid_file.clone(),
        }
    }
}


impl Executable for Command {
    fn execute (&self,) {
        let oci_config = load_oci_config(&self.bundle).expect("parse config.json fail");
        info!("oci config is {:?}", oci_config);

        let (init_p, init_c) = socket::socketpair(
                                    socket::AddressFamily::Unix, 
                                    socket::SockType::Stream, None, 
                                    socket::SockFlag::empty()
                                ).expect("create sockpair fail");

        let mut init_pipe = unsafe { File::from_raw_fd(init_p) };
        let init_param = self.make_init_param(oci_config.clone());
        let init_param = ser::to_string(&init_param).expect("failed serialize init para to json");
        let mut init_param: Vec<u8> = Vec::from(init_param.as_bytes());
        // init_param.push(0);
        init_pipe.write(&init_param[..]).expect("failed write to init pipe");

        self.run_init_and_wait(init_c);
    }
}