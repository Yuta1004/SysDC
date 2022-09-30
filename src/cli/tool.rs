mod list;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct ToolCmd {
    #[clap(subcommand)]
    sub: ToolCmdSub,
}

#[derive(Subcommand)]
#[allow(non_camel_case_types)]
enum ToolCmdSub {
    /// Print installed tools
    list(list::ListCmd),
}

impl ToolCmd {
    pub fn run(&self) -> anyhow::Result<()> {
        match &self.sub {
            ToolCmdSub::list(cmd) => cmd.run(),
        }
    }
}
