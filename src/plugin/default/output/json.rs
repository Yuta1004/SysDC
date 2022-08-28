use std::fs::File;
use std::io::Write;

use serde_json;

use crate::plugin::{ OutputPlugin, PluginError };
use crate::parser::structure::SysDCSystem;

pub struct JSONPlugin;

impl JSONPlugin {
    pub fn new() -> Box<JSONPlugin> {
        Box::new(JSONPlugin)
    }
}

impl OutputPlugin for JSONPlugin {
    fn get_name(&self) -> &str {
        "json"
    }

    fn run(&self, args: Vec<String>, system: &SysDCSystem) -> Result<(), Box<dyn std::error::Error>> {
        let serialized_system = serde_json::to_string(system).unwrap();
        match args.len() {
            0 => println!("{}", serialized_system),
            1 => {
                let mut f = File::create(&args[0])?;
                write!(f, "{}", serialized_system)?;
                f.flush()?;
            }
            _ => Err(Box::new(
                PluginError::Runtime("Usage: out json <filepath>".to_string())
            ))?
        }
        Ok(())
    }
}
