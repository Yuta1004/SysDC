use clap::Parser;

#[derive(Parser)]
pub struct ListCmd;

impl ListCmd {
    pub fn run(&self) -> anyhow::Result<()> {
        println!("* debug (v1.0.0) : Print a internal structure");
        println!("* json (v1.0.1) : Convert a internal structure into JSON");
        println!("* view (v0.1.0) : Graphically depict software design");
        Ok(())
    }
}
