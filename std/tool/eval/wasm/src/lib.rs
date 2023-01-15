mod commands;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::{ wasm_bindgen, JsValue };

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

#[cfg(feature = "wasm")]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn gen_advice() -> JsValue {
    serde_wasm_bindgen::to_value(
        &vec![
            commands::test()
        ].into_iter().flatten().collect::<Vec<Advice>>()
    ).unwrap()
}
