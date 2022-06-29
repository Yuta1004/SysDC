use super::super::OutputPlugin;
use crate::compiler::structure::SysDCSystem;

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

    fn run(&self, _: Vec<String>, system: &SysDCSystem) {
        println!("{:?}", system);
    }
}
