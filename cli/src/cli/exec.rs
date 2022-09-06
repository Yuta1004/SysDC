use std::fmt;
use std::fmt::{ Display, Formatter };
use std::fs;
use std::error::Error;

use clap::Parser;
use rmp_serde;

use sysdc_parser::structure::SysDCSystem;
use sysdc_tool_debug;
use sysdc_tool_json;

#[derive(Debug)]
enum ExecError {
    ToolNotFound(String)
}

impl Error for ExecError {}

impl Display for ExecError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ExecError::ToolNotFound(name) => write!(f, "Tool \"{}\" not found", name)
        }
    }
}

#[derive(Parser)]
#[clap(name="subcommand")]
pub struct ExecCmd {
    #[clap(required=true)]
    tool: String,

    #[clap(short, long)]
    args: Vec<String>,

    #[clap(short, long, default_value="out.sysdc")]
    input: String
}

impl ExecCmd {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let system = self.load_system()?;
        match self.tool.as_str() {
            "debug" => sysdc_tool_debug::exec(&system),
            "json" => sysdc_tool_json::exec(&system, &self.args),
            t => return Err(Box::new(ExecError::ToolNotFound(t.to_string())))
        }
        Ok(())
    }

    fn load_system(&self) -> Result<SysDCSystem, Box<dyn Error>> {
        let serialized_system = fs::read(&self.input)?;
        Ok(rmp_serde::from_slice::<SysDCSystem>(&serialized_system[..])?)
    }
}
