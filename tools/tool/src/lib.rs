use sysdc_parser::structure::SysDCSystem;

pub trait Tool {
    fn run(&self, args: Vec<String>, system: &SysDCSystem);
}
