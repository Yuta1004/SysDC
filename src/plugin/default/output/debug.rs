use sysdc::parser::structure::SysDCSystem;

use crate::plugin::OutputPlugin;

pub struct DebugPlugin;

impl DebugPlugin {
    pub fn new() -> Box<DebugPlugin> {
        Box::new(DebugPlugin)
    }
}

impl OutputPlugin for DebugPlugin {
    fn get_name(&self) -> &str {
        "debug"
    }

    fn run(&self, _: Vec<String>, system: &SysDCSystem) -> Result<(), Box<dyn std::error::Error>> {
        println!("{:?}", system);
        Ok(())
    }
}
