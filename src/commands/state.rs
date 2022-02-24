use {
    log::error,
    serde::{Deserialize, Serialize},
    serde_json as json,
    std::process,
};

use {
    crate::{
        commands::{Context, Executable},
        container,
    },
    clap::{App, Arg},
};

pub fn subcommand<'a>() -> App<'a> {
    App::new("state")
        .about("output the state of a container")
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

impl Executable for Command {
    fn execute(&self) {
        let state = match self.container.load_state_file() {
            Ok(v) => v,
            Err(s) => {
                error!("{}", s);
                process::exit(1)
            }
        };

        let (annos, bundle) = state.annotations();
        println!(
            "{}",
            json::ser::to_string_pretty(&Output {
                id: self.container.id.clone(),
                pid: state.init_process_pid,
                status: self.container.status().to_string(),
                bundle: bundle,
                rootfs: state.config.rootfs,
                created: state.created,
                annotations: annos,
            })
            .expect("failed to serialize container state info")
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Output {
    id: String,
    pid: u32,
    status: String,
    bundle: String,
    rootfs: String,
    created: String,
    annotations: Vec<String>,
}
