#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub enum AdviceLevel {
    Info = 0,
    Warning = 1
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Advice {
    level: AdviceLevel,
    title: String,
    message: String
}

impl Advice {
    pub fn new(level: AdviceLevel, title: String, message: String) -> Advice {
        Advice { level, title, message }
    }
}
