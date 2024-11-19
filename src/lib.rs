use clap::{Arg, ArgAction, ArgMatches};
use env_logger::Env;
use env_logger::Target::Stdout;
use log::debug;

pub fn debug_arg() -> Arg {
  Arg::new("DEBUG")
    .help("Show debug logging")
    .short('d')
    .long("debug")
    .action(ArgAction::SetTrue)
}

pub fn verbose_arg() -> Arg {
  Arg::new("VERBOSE")
    .help("Show verbose output")
    .short('v')
    .long("verbose")
    .action(ArgAction::SetTrue)
}

pub fn configure_logging(args: &ArgMatches) {
  let filter = if args.get_flag("DEBUG") {
    "debug"
  } else {
    "info"
  };
  env_logger::Builder::from_env(Env::default().default_filter_or(filter))
    .target(Stdout)
    .init();
  debug!("Debug logging");
}
