use {
    crate::{commands::Executable, models::oci, utils},
    clap::{App, Arg},
    log::{error, info},
    serde_json as json,
    std::{env, fs, io::prelude::*, process},
};

pub fn subcommand<'a>() -> App<'a> {
    App::new("spec")
        .about("create a new specification file")
        .version("0.1")
        .arg(
            Arg::new("bundle")
                .help("path to the root of the bundle directory")
                .takes_value(true)
                .long("bundle")
                .short('b'),
        )
}

#[derive(Debug)]
pub struct Command {
    bundle: String,
}

pub fn new(sub_matchs: &clap::ArgMatches) -> Box<dyn Executable> {
    let mut bundle = if let Some(v) = sub_matchs.value_of("bundle") {
        String::from(v)
    } else {
        String::from(String::from(env::var("PWD").unwrap()))
    };

    if !utils::is_directory(&bundle) {
        error!("{} is not exist or is not directory", bundle);
        process::exit(1);
    }
    bundle = utils::last_must_separator(bundle);

    Box::from(Command {
        bundle: String::from(bundle),
    })
}

impl Executable for Command {
    fn execute(&self) {
        let config = oci::Config {
            root: oci::Root {
                path: String::new(),
            },
            process: oci::Process { args: vec![] },
            annotations: vec![],
        };
        let config_val = json::ser::to_string_pretty(&config).expect("failed to genarete json");
        let config_path = String::from(&self.bundle) + "config.json";
        if utils::is_exist(&config_path) {
            error!("{}", "config.json is existed");
            process::exit(-1);
        }
        info!("config path is: {}", config_path);
        let mut file = fs::File::create(config_path).expect("failed to create config.json");

        file.write(config_val.as_bytes())
            .expect("failed to write config.json");
    }
}
