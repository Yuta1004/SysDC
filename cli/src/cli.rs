mod interactive;
mod plugin;

use clap::{ AppSettings, Parser, Subcommand };

use interactive::InteractiveCmd;
use plugin::PluginCmd;

/// SysDC: System Definition Language and Tools
#[derive(Parser)]
#[clap(author, version, name="subcommand")]
#[clap(global_settings(&[AppSettings::DisableHelpSubcommand]))]
pub struct App {
    #[clap(subcommand)]
    sub: AppSub
}

#[derive(Subcommand)]
#[allow(non_camel_case_types)]
enum AppSub {
    /// Run interactive mode
    interactive(InteractiveCmd),

    /// Setup plugins (ex. add, remove)
    plugin(PluginCmd),
}

impl App {
    pub fn run() {
        match App::parse().sub {
            AppSub::interactive(mut cmd) => cmd.run(),
            AppSub::plugin(cmd) => cmd.run()
        }
    }
}
