mod default;

use std::fmt;
use std::fmt::{ Display, Formatter };
use std::error::Error;

use default::{ input, output };
use crate::parser::structure::SysDCSystem;

pub trait InputPlugin {
    fn get_name(&self) -> &str;
    fn run(&self, args: Vec<String>) -> Result<Vec<(String, String)>, Box<dyn Error>>;
}

pub trait OutputPlugin {
    fn get_name(&self) -> &str;
    fn run(&self, args: Vec<String>, system: &SysDCSystem) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug)]
pub enum PluginError {
    NotFound(String),
    Runtime(String),
    Unknown
}

impl Error for PluginError {}

impl Display for PluginError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            PluginError::NotFound(name) => write!(f, "Plugin \"{}\" not found", name),
            PluginError::Runtime(msg) => write!(f, "{}", msg),
            PluginError::Unknown => write!(f, "Occured unknown error at running plugin")
        }
    }
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

    pub fn get_type_in(&self, name: &String) -> Result<&Box<dyn InputPlugin>, Box<dyn Error>> {
        for plugin in &self.in_plugins {
            if plugin.get_name() == name {
                return Ok(plugin);
            }
        }
        Err(Box::new(PluginError::NotFound(name.to_string())))
    }

    pub fn get_type_out(&self, name: &String) -> Result<&Box<dyn OutputPlugin>, Box<dyn Error>> {
        for plugin in &self.out_plugins {
            if plugin.get_name() == name {
                return Ok(plugin);
            }
        }
        Err(Box::new(PluginError::NotFound(name.to_string())))
    }

    fn load_default_plugins() -> (Vec<Box<dyn InputPlugin>>, Vec<Box<dyn OutputPlugin>>) {
        let in_plugins: Vec<Box<dyn InputPlugin>> = vec!(
            input::debug::DebugPlugin::new(),
            input::files::FilesPlugin::new()
        );
        let out_plugins: Vec<Box<dyn OutputPlugin>> = vec!(
            output::debug::DebugPlugin::new(),
            output::json::JSONPlugin::new()
        );
        (in_plugins, out_plugins)
    }
}

#[cfg(test)]
mod test {
    use super::SysDCSystem;
    use super::PluginManager;

    #[test]
    fn test_in_debug() {
        let plugin_manager = PluginManager::new();
        let plugin = plugin_manager.get_type_in(&"debug".to_string());
        assert!(plugin.is_ok());
        plugin.unwrap().run(vec!()).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_in_debug_panic() {
        PluginManager::new().get_type_in(&"test".to_string()).unwrap();
    }

    #[test]
    fn test_out_debug() {
        let plugin_manager = PluginManager::new();
        let plugin = plugin_manager.get_type_out(&"debug".to_string());
        assert!(plugin.is_ok());
        plugin.unwrap().run(vec!(), &SysDCSystem { units: vec!() }).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_out_debug_panic() {
        PluginManager::new().get_type_out(&"test".to_string()).unwrap();
    }
}
