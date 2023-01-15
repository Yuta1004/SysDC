#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::{ wasm_bindgen, JsValue };

use crate::{ Advice, AdviceLevel };

#[cfg(feature = "wasm")]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn test() -> JsValue {
    let result = __test_body();
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[cfg(not(feature = "wasm"))]
pub fn test() -> Vec<Advice> {
    __test_body()
}

fn __test_body() -> Vec<Advice> {
    vec![
        Advice::new(
            AdviceLevel::Warning,
            "Warning 1".to_string(),
            vec![
                "AAAAAAAAAA".to_string(),
                "AAAAAAAAAA".to_string(),
                "AAAAAAAAAA".to_string(),
                "AAAAAAAAAA".to_string(),
            ]
        ),
        Advice::new(
            AdviceLevel::Warning,
            "Warning 2".to_string(),
            vec![
                "AAAAAAAAAA".to_string(),
                "AAAAAAAAAA".to_string(),
                "AAAAAAAAAA".to_string(),
                "AAAAAAAAAA".to_string(),
            ]
        ),
        Advice::new(
            AdviceLevel::Info,
            "Info 1".to_string(),
            vec![
                "AAAAAAAAAA".to_string(),
                "AAAAAAAAAA".to_string(),
                "AAAAAAAAAA".to_string(),
                "AAAAAAAAAA".to_string(),
            ]
        )
    ]
}
