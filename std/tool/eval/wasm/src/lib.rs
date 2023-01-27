mod commands;

use serde::{ Serialize, Deserialize };
use wasm_bindgen::prelude::{ wasm_bindgen, JsValue };


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

#[wasm_bindgen]
pub fn gen_advice(system: JsValue) -> JsValue {
    let advice = match serde_wasm_bindgen::from_value(system) {
        Ok(system) => {
            vec![
                commands::complex::eval_complex_stat(&system),
                commands::duplication::eval_duplication_stat(&system),
                commands::basic::eval_basic_stat(&system)
            ].into_iter().flat_map(|adv| adv).collect::<Vec<Advice>>()
        }
        Err(_e) => vec![]
    };
    serde_wasm_bindgen::to_value(&advice).unwrap()
}
