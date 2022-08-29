use sysdc_parser::structure::SysDCSystem;
use sysdc_tool::Tool;

pub struct DebugTool;

impl DebugTool {
    pub fn new() -> Box<DebugTool> {
        Box::new(DebugTool)
    }
}

impl Tool for DebugTool {
    fn run(&self, _: Vec<String>, system: &SysDCSystem) {
        println!("{:?}", system);
    }
}
