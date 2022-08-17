use std::fs;

use glob;

use crate::plugin::{ InputPlugin, PluginError };

pub struct FilesPlugin;

impl FilesPlugin {
    pub fn new() -> Box<FilesPlugin> {
        Box::new(FilesPlugin)
    }
}

impl InputPlugin for FilesPlugin {
    fn get_name(&self) -> &str {
        "files"
    }

    fn run(&self, args: Vec<String>) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
        if args.len() == 0 {
            return Err(Box::new(
                PluginError::RuntimeError("Argument list is empty".to_string())
            ));
        }

        let mut programs = vec!();
        for arg in args {
            for entries in glob::glob(&arg) {
                for entry in entries {
                    let entry = entry?;
                    if entry.is_file() {
                        let unit_name = entry.file_name().ok_or(PluginError::UnknownError)?
                                             .to_str().ok_or(PluginError::UnknownError)?
                                             .to_string();
                        let program = fs::read_to_string(&entry).unwrap();
                        programs.push((unit_name, program));
                    }
                }
            }
        }
        Ok(programs)
    }
}
