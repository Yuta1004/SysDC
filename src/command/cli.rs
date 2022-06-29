#[derive(clap::Parser)]
pub struct CliCmd;

impl CliCmd {
    pub fn run(&self) {
        println!("Cli Command");
    }
}
