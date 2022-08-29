use clap::Parser;

#[derive(Parser)]
#[clap(name="subcommand")]
pub struct ParseCmd ;

impl ParseCmd {
    pub fn run(&self) {
        println!("parse");
    }
}
