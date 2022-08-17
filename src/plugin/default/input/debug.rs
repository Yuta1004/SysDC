use std::error::Error;

use crate::plugin::InputPlugin;

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
