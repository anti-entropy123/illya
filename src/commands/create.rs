use {
    crate::{
        commands::{Context, Executable},
        container,
        models::{init, oci},
        utils,
    },
    clap::{App, Arg},
    log::error,
    nix::sys::socket,
    serde::Serialize,
    serde_json::ser,
    std::{fs::File, io::prelude::*, os::unix::io::FromRawFd, process},
};

pub fn subcommand<'a>() -> App<'a> {
    App::new("create")
        .about("create container")
        .version("0.1")
        .arg(
            Arg::new("bundle")
                .help("path to the root of the bundle directory, defaults to the current directory")
                .takes_value(true)
                .long("bundle"),
        )
        .arg(
            Arg::new("pid-file")
                .help("filename that the container pid will be written to")
                .takes_value(true)
                .long("pid-file"),
        )
        .arg(
            Arg::new("container")
                .multiple_occurrences(false)
                .required(true),
        )
}

pub struct Command {
    pub pid_file: String,
    pub bundle: String,
    pub container: container::Container,
}

pub fn new(sub_matchs: &clap::ArgMatches, ctx: Context) -> Box<dyn Executable> {
    let pid_file = sub_matchs.value_of("pid-file").expect("no pid-file");

    let bundle = if let Some(b) = sub_matchs.value_of("bundle") {
        b
    } else {
        "."
    };
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
        }
        None => {
            error!("no input container.");
            process::exit(1);
        }
    }

    Box::from(Command {
        bundle: bundle,
        pid_file: pid_file.to_string(),
        container: container::Container::new(&container_id, Box::new(ctx)),
    })
}

impl Command {
    fn run_init(&self, init_fd_c: i32, wait: bool) {
        let mut child = process::Command::new("/proc/self/exe")
            .env("_LIBCONTAINER_INITPIPE", format!("{}", init_fd_c))
            .arg("init")
            .spawn()
            .expect("failed to spawn subprocess");
        if wait {
            child.wait().expect("failed to wait child exit");
            // debug!("child exit with {}", code);
        };
    }

    fn make_init_param(&self, oci_config: oci::Config) -> init::Parameter {
        init::Parameter {
            container_id: self.container.id.clone(),
            root_path: oci_config.root.path,
            args: oci_config.process.args,
            bundle: self.bundle.clone(),
            pid_file: self.pid_file.clone(),
        }
    }
}

fn send_to_pipe(pipe_fd: i32, data: impl Serialize) -> Result<(), String> {
    let mut init_pipe = unsafe { File::from_raw_fd(pipe_fd) };
    let init_param = match ser::to_string(&data) {
        Ok(param) => param,
        Err(e) => return Err(format!("failed serialize init param to json: {}", e)),
    };
    if let Err(e) = write!(init_pipe, "{}", init_param) {
        return Err(format!("failed write to init pipe: {}", e));
    };
    Ok(())
}

impl Executable for Command {
    fn execute(&self) {
        let oci_config = container::load_oci_config(&self.bundle).expect("parse config.json fail");
        // info!("oci config is {:?}", oci_config);

        let (init_p, init_c) = socket::socketpair(
            socket::AddressFamily::Unix,
            socket::SockType::Stream,
            None,
            socket::SockFlag::empty(),
        )
        .expect("create sockpair fail");

        let init_param = self.make_init_param(oci_config.clone());
        send_to_pipe(init_p, init_param).expect("failed to write init pipe");
        self.run_init(init_c, false);
    }
}
