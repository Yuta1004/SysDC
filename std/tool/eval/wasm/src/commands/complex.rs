use crate::{ Advice, AdviceLevel };
use sysdc_core::structure::SysDCSystem;

#[cfg_attr(not(feature = "wasm"), allow(dead_code))]
pub fn eval_complex_stat(system: &SysDCSystem) -> Vec<Advice> {
    let messages = vec![
        "aaa".to_string(),
        "bbb".to_string(),
        "ccc".to_string()
    ];

    vec![
        Advice::new(
            AdviceLevel::Warning,
            "分割可能な処理".to_string(),
            messages
        )
    ]
}
