use crate::{ Advice, AdviceLevel };
use sysdc_core::structure::SysDCSystem;

#[cfg_attr(not(feature = "wasm"), allow(dead_code))]
pub fn eval_duplication_stat(system: &SysDCSystem) -> Option<Advice> {
    let messages = vec![
        "aaa".to_string(),
        "bbb".to_string(),
        "ccc".to_string()
    ];

    Some(
        Advice::new(
            AdviceLevel::Warning,
            "統合可能な処理".to_string(),
            messages
        )
    )
}
