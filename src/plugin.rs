pub mod default;

use default::{ input, output };
use crate::compiler::structure::SysDCSystem;

pub trait InputPlugin: Iterator<Item=(String, String)> {
    fn get_name(&self) -> &str;
    fn init(&mut self, args: Vec<String>);
}

pub trait OutputPlugin {
    fn get_name(&self) -> &str;
    fn run(&self, args: Vec<String>, system: &SysDCSystem);
}

pub struct PluginManager {
    in_plugins: Vec<Box<dyn InputPlugin>>,
    out_plugins: Vec<Box<dyn OutputPlugin>>
}

impl PluginManager {
    pub fn new() -> PluginManager {
        let (in_plugins, out_plugins) = PluginManager::load_default_plugins();
        PluginManager { in_plugins, out_plugins }
    }

    fn load_default_plugins() -> (Vec<Box<dyn InputPlugin>>, Vec<Box<dyn OutputPlugin>>) {
        let in_plugins: Vec<Box<dyn InputPlugin>> = vec!(
            input::DebugPlugin::new()
        );
        let out_plugins: Vec<Box<dyn OutputPlugin>> = vec!(
            output::DebugPlugin::new()
        );
        (in_plugins, out_plugins)
    }
}
