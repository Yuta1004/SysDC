mod parse;
mod exec;

use clap::{ AppSettings, Parser, Subcommand };

use parse::ParseCmd;
use exec::ExecCmd;

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
    /// Parse *.def files
    parse(ParseCmd),

    /// Execute tool
    exec(ExecCmd)
}

impl App {
    pub fn run() {
        let result = match App::parse().sub {
            AppSub::parse(cmd) => cmd.run(),
            AppSub::exec(cmd) => cmd.run()
        };
        if let Err(err) = result {
            println!("[ERROR] {}", err);
        }
    }
}
