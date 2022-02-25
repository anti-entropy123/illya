use {
    crate::{
        commands::{Context, Executable},
        container, utils,
    },
    clap::App,
    log::{info, warn},
    std::io::Write,
    tabwriter::TabWriter,
};

pub fn subcommand<'a>() -> App<'a> {
    App::new("list")
        .about("lists containers started by runc with the given root")
        .version("0.1")
}

pub struct Command {
    ctx: Context,
}

pub fn new(_sub_matchs: &clap::ArgMatches, ctx: Context) -> Box<dyn Executable> {
    Box::new(Command { ctx })
}

impl Executable for Command {
    fn execute(&self) {
        let mut tw = TabWriter::new(vec![]);
        write!(&mut tw, "ID\tPID\tSTATUS\tBUNDLE\tCREATED\n").unwrap();
        if let Ok(items) = utils::dir_item_names(&self.ctx.containers_dir) {
            for item in &items {
                info!("list item: {}", &item);
                let c = container::Container::new(item, Box::new(self.ctx.clone()));
                match c.load_state_file() {
                    Ok(state) => {
                        info!("state is {:?}", state);
                        let (_, bundle) = state.annotations();
                        write!(
                            &mut tw,
                            "{}\t{}\t{}\t{}\t{}\n",
                            state.id,
                            state.init_process_pid,
                            c.status().to_string(),
                            bundle,
                            state.created,
                        )
                        .unwrap();
                    }
                    Err(e) => {
                        warn!("failed to load_state_file: {}", e)
                    }
                };
            }
        }
        tw.flush().unwrap();

        let written = String::from_utf8(tw.into_inner().unwrap()).unwrap();
        println!("{}", written);
    }
}
