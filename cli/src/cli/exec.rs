use clap::Parser;

#[derive(Parser)]
#[clap(name="subcommand")]
pub struct ExecCmd ;

impl ExecCmd {
    pub fn run(&self) {
        println!("exec");
    }
}
