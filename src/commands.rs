use {
    crate::utils,
    log::{error, info},
    std::{env, fs, path, process},
};

pub trait Executable {
    fn execute(&self);
}

#[derive(Clone)]
pub struct Context {
    pub runtime_dir: String,
    pub containers_dir: String,
}

impl Context {
    pub fn new() -> Self {
        let mut runtime_dir = env::var("XDG_RUNTIME_DIR").expect("no env var XDG_RUNTIME_DIR");
        // "/run/user/1000/illya"
        runtime_dir = String::from(
            path::Path::new(&runtime_dir)
                .join("illya")
                .to_str()
                .unwrap(),
        );
        if !utils::is_directory(&runtime_dir) {
            fs::create_dir_all(&runtime_dir).expect("failed to create run directory");
        };
        let containers_dir = path::Path::new(&runtime_dir).join("containers");
        let containers_dir = String::from(containers_dir.to_str().unwrap());
        info!("containers_dir is {}", containers_dir);
        Context {
            runtime_dir,
            containers_dir,
        }
    }
}

pub fn match_command(matchs: clap::ArgMatches, ctx: Context) -> Box<dyn Executable> {
    let command = match matchs.subcommand() {
        Some(("delete", sub_matchs)) => delete::new(sub_matchs),
        Some(("create", sub_matchs)) => create::new(sub_matchs, ctx),
        Some(("init", sub_matchs)) => init::new(sub_matchs, ctx),
        Some(("start", sub_matchs)) => start::new(sub_matchs, ctx),
        Some(("spec", sub_matchs)) => spec::new(sub_matchs),
        Some(("state", sub_matchs)) => state::new(sub_matchs, ctx),
        Some(("list", sub_matchs)) => list::new(sub_matchs, ctx),
        _ => {
            error!("unimplement subcommand: {:?}", matchs);
            process::exit(1);
        }
    };
    command
}

pub mod create;
pub mod delete;
pub mod init;
pub mod list;
pub mod spec;
pub mod start;
pub mod state;
