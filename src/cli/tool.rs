use clap::Parser;

#[derive(Parser)]
pub struct ToolCmd {
}

impl ToolCmd {
    pub fn run(&self) -> anyhow::Result<()> {
        println!("tool");
        Ok(())
    }
}
