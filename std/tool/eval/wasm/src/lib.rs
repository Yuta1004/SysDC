pub mod commands;

use serde::{ Serialize, Deserialize };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdviceLevel {
    Info = 0,
    Warning = 1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Advice {
    level: AdviceLevel,
    title: String,
    messages: Vec<String>
}


impl Advice {
    pub fn new(level: AdviceLevel, title: String, messages: Vec<String>) -> Advice {
        Advice { level, title, messages }
    }
}
