pub mod cli;
pub mod plugin;

use clap::{ AppSettings, Parser, Subcommand };

use cli::CliCmd;
use plugin::PluginCmd;

/// SysDC: System Definition Language and Tools
#[derive(Parser)]
#[clap(author, version, name="subcommand")]
#[clap(global_settings(&[AppSettings::DisableHelpSubcommand]))]
pub struct App {
    #[clap(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
#[allow(non_camel_case_types)]
enum Commands {
    /// Run program
    cli(CliCmd),

    /// Setup plugins (ex. add, remove)
    plugin(PluginCmd),
}

impl App {
    pub fn run() {
        match App::parse().command {
            Commands::cli(cmd) => cmd.run(),
            Commands::plugin(cmd) => cmd.run()
        }
    }
}
