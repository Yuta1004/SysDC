use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum AdviceLevel {
    Info = 0,
    Warning = 1
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
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
