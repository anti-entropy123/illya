use {
   clap::App,
   illya::{commands, commands::*, log as logger},
   log::{info, LevelFilter},
   std::env,
};

fn execute() {
   let matchs = App::new("illya")
      .version("0.1")
      .author("闹钟大魔王")
      .about("newer OCI runtime")
      .subcommands(vec![
         create::subcommand(),
         init::subcommand(),
         delete::subcommand(),
         start::subcommand(),
         spec::subcommand(),
         state::subcommand(),
         list::subcommand(),
      ])
      .get_matches();
   let ctx = Context::new();
   let subcommand = commands::match_command(matchs, ctx);
   subcommand.execute();
}

fn set_log() {
   log::set_boxed_logger(Box::new(logger::new())).expect("fail to init log");
   log::set_max_level(LevelFilter::Info);
}

fn main() {
   set_log();
   info!("{:?}", env::args());
   execute();
}
