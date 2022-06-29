use super::super::OutputPlugin;
use crate::compiler::structure::SysDCSystem;

pub struct DebugPlugin;

impl DebugPlugin {
    pub fn new() -> Box<DebugPlugin> {
        Box::new(DebugPlugin)
    }
}

impl OutputPlugin for DebugPlugin {
    fn get_name(&self) -> String {
        "debug".to_string()
    }

    fn run(&self, system: &SysDCSystem) {
        println!("{:?}", system);
    }
}
