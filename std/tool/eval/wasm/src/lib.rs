pub mod commands;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

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
    message: Vec<String>
}


impl Advice {
    pub fn new(level: AdviceLevel, title: String, message: Vec<String>) -> Advice {
        Advice { level, title, message }
    }
}
