mod parse;
mod exec;

use std::process::exit;

use clap::{AppSettings, Parser, Subcommand};

use exec::ExecCmd;
use parse::ParseCmd;

/// SysDC: System Definition Language and Tools
#[derive(Parser)]
#[clap(author, version, name = "subcommand")]
#[clap(global_settings(&[AppSettings::DisableHelpSubcommand]))]
pub struct App {
    #[clap(subcommand)]
    sub: AppSub,
}

#[derive(Subcommand)]
#[allow(non_camel_case_types)]
enum AppSub {
    /// Parse *.def files
    parse(ParseCmd),

    /// Execute tool
    exec(ExecCmd),
}

impl App {
    pub fn run() {
        let result = match App::parse().sub {
            AppSub::parse(cmd) => cmd.run(),
            AppSub::exec(cmd) => cmd.run(),
        };
        match result {
            Ok(_) => exit(0),
            Err(err) => {
                println!("[ERROR] {}", err);
                exit(1);
            }
        }
    }
}
