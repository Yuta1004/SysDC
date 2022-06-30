use std::fs;
use std::error::Error;

use glob;

use super::super::{ InputPlugin, PluginError };

pub struct DebugPlugin;

impl DebugPlugin {
    pub fn new() -> Box<DebugPlugin> {
        Box::new(DebugPlugin)
    }
}

impl InputPlugin for DebugPlugin {
    fn get_name(&self) -> &str {
        "debug"
    }

    fn run(&self, _: Vec<String>) -> Result<Vec<(String, String)>, Box<dyn Error>> {
        let unit_name = "debug".to_string();
        let program = "
            layer 0;

            data User {
                id: int32,
                age: int32,
                name: string
            }

            module UserModule binds User as this {
                greet() -> string {
                    use = [this.name];
                }
                
                change_age(age: int32) -> none {
                    modify = [this.age];
                }
            }
        ".to_string();
        Ok(vec!((unit_name, program)))
    }
}

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
