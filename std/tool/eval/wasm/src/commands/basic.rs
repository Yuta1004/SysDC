use crate::{ Advice, AdviceLevel };
use sysdc_core::structure::SysDCSystem;

#[cfg_attr(not(feature = "wasm"), allow(dead_code))]
pub fn eval_basic_stat(system: &SysDCSystem) -> Vec<Advice> {
    let messages = vec!["aaa".to_string()];

    vec![
        Advice::new(
            AdviceLevel::Info,
            "基本情報".to_string(),
            messages
        )
    ]
}
