use sysdc_parser::structure::SysDCSystem;

pub struct DebugTool;

impl DebugTool {
    pub fn exec(system: &SysDCSystem) {
        println!("{:?}", system);
    }
}
