mod parse;
mod run;

use std::process::exit;

use clap::{AppSettings, Parser, Subcommand};

/// Programming Language aiming to support Software Design and Development
#[derive(Parser)]
#[clap(author, version, name = "SysDC-Cli")]
#[clap(global_settings(&[AppSettings::DisableHelpSubcommand]))]
pub struct App {
    #[clap(subcommand)]
    sub: AppSub,
}

#[derive(Subcommand)]
#[allow(non_camel_case_types)]
enum AppSub {
    /// Parse files *.def into *.sysdc
    parse(parse::ParseCmd),

    /// Run tools
    run(run::RunCmd),
}

impl App {
    pub fn run() {
        let result = match App::parse().sub {
            AppSub::parse(cmd) => cmd.run(),
            AppSub::run(cmd) => cmd.run(),
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
