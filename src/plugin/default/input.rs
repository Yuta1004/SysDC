use super::super::InputPlugin;

pub struct DebugPlugin {
    iter_cnt: i32
}

impl DebugPlugin {
    pub fn new() -> Box<DebugPlugin> {
        Box::new(
            DebugPlugin { iter_cnt: 0 }
        )
    }
}

impl InputPlugin for DebugPlugin {
    fn get_name(&self) -> &str {
        "debug"
    }

    fn init(&mut self, _: Vec<String>) {
        self.iter_cnt = 0;
    }
}

impl Iterator for DebugPlugin {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_cnt == 0 {
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
            self.iter_cnt += 1;
            Some((unit_name, program))
        } else {
            None
        }
    }
}
