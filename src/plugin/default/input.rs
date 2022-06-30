use std::fs;

use glob;

use super::super::InputPlugin;

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

    fn run(&self, _: Vec<String>) -> Vec<(String, String)> {
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
        vec!((unit_name, program))
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

    fn run(&self, args: Vec<String>) -> Vec<(String, String)> {
        if args.len() == 0 {
            return vec!();
        }

        let mut programs = vec!();
        for arg in args {
            for files in glob::glob(&arg) {
                for file in files {
                    match file {
                        Ok(path) => {
                            if path.is_file() {
                                let unit_name = path.file_name().unwrap().to_str().unwrap().to_string();
                                let program = fs::read_to_string(&path).unwrap();
                                programs.push((unit_name, program));
                            }
                        },
                        Err(e) => println!("[ERROR] {}", e)
                    }
                }
            }
        }
        programs
    }
}
