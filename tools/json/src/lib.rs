use std::fs::File;
use std::io::Write;

use serde_json;
use sysdc_parser::structure::SysDCSystem;
use sysdc_tool::Tool;

pub struct JSONTool;

impl JSONTool {
    pub fn new() -> Box<JSONTool> {
        Box::new(JSONTool)
    }
}

impl Tool for JSONTool {
    fn run(&self, args: Vec<String>, system: &SysDCSystem) {
        let serialized_system = serde_json::to_string(system).unwrap();
        match args.len() {
            0 => println!("{}", serialized_system),
            _ => {
                let mut f = File::create(&args[0]).unwrap();
                write!(f, "{}", serialized_system).unwrap();
                f.flush().unwrap();
            }
        }
    }
}
