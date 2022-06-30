pub mod default;

use default::{ input, output };
use crate::compiler::structure::SysDCSystem;

pub trait InputPlugin {
    fn get_name(&self) -> &str;
    fn run(&self, args: Vec<String>) -> Vec<(String, String)>;
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

    pub fn get_type_in(&self, name: &String) -> Option<&Box<dyn InputPlugin>> {
        for plugin in &self.in_plugins {
            if plugin.get_name() == name {
                return Some(plugin);
            }
        }
        None
    }

    pub fn get_type_out(&self, name: &String) -> Option<&Box<dyn OutputPlugin>> {
        for plugin in &self.out_plugins {
            if plugin.get_name() == name {
                return Some(plugin);
            }
        }
        None
    }

    fn load_default_plugins() -> (Vec<Box<dyn InputPlugin>>, Vec<Box<dyn OutputPlugin>>) {
        let in_plugins: Vec<Box<dyn InputPlugin>> = vec!(
            input::DebugPlugin::new(),
            input::FilesPlugin::new()
        );
        let out_plugins: Vec<Box<dyn OutputPlugin>> = vec!(
            output::DebugPlugin::new()
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
        assert!(plugin.is_some());
        plugin.unwrap().run(vec!());
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
        assert!(plugin.is_some());
        plugin.unwrap().run(vec!(), &SysDCSystem::new());
    }

    #[test]
    #[should_panic]
    fn test_out_debug_panic() {
        PluginManager::new().get_type_out(&"test".to_string()).unwrap();
    }
}
