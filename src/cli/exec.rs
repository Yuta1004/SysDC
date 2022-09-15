use std::fs;

use clap::Parser;
use thiserror::Error;

use sysdc_parser::structure::SysDCSystem;

#[derive(Debug, Error)]
enum ExecError {
    #[error("Tool \"{0}\" not found")]
    ToolNotFound(String),
}

#[derive(Parser)]
#[clap(name = "subcommand")]
pub struct ExecCmd {
    #[clap(required = true)]
    tool: String,

    #[clap(short, long)]
    args: Vec<String>,

    #[clap(short, long, default_value = "out.sysdc")]
    input: String,
}

impl ExecCmd {
    pub fn run(&self) -> anyhow::Result<()> {
        let system = self.load_system()?;
        match self.tool.as_str() {
            "debug" => sysdc_tool_debug::exec(&system)?,
            "json" => sysdc_tool_json::exec(&system, &self.args)?,
            "view" => sysdc_tool_view::exec()?,
            t => return Err(ExecError::ToolNotFound(t.to_string()).into()),
        }
        Ok(())
    }

    fn load_system(&self) -> anyhow::Result<SysDCSystem> {
        let serialized_system = fs::read(&self.input)?;
        Ok(rmp_serde::from_slice::<SysDCSystem>(
            &serialized_system[..],
        )?)
    }
}
