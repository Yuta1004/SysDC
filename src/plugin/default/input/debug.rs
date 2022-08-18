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
            data Box {
                x: i32,
                y: i32,
                w: i32,
                h: i32
            }

            module BoxModule {
                new(x: i32, y: i32, w: i32, h: i32) -> Box {
                    @return box

                    +use x, y, w, h
                    @spawn box: Box
                }

                move(box: Box, dx: i32, dy: i32) -> Box {
                    @return movedBox

                    +use box
                    +use dx, dy
                    @spawn movedBox: Box
                }

                changeSize(box: Box, w: i32, h: i32) -> Box {
                    @return changedBox

                    +use box
                    +use w, h
                    @spawn changedBox: Box
                }
            }
        ".to_string();
        Ok(vec!((unit_name, program)))
    }
}
