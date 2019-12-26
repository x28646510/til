use std::env;

use clap::{App, AppSettings, SubCommand};

mod version;

fn clap_root<'a, 'b>() -> App<'a, 'b> {
  App::new("TIL clap")
    .bin_name("til_clap")
    .global_settings(&[AppSettings::UnifiedHelpMessage])
    .version(crate::version::TIL_CLAP)
    .subcommand(get_subcommand())
    .about("TIL clap")
}

fn get_subcommand<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("get").about("Get something")
}

fn main() {
  let matches = clap_root().get_matches();
}
