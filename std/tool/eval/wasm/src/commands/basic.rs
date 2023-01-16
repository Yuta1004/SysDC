use crate::{ Advice, AdviceLevel };
use sysdc_core::structure::SysDCSystem;

#[cfg_attr(not(feature = "wasm"), allow(dead_code))]
pub fn eval_basic_stat(system: &SysDCSystem) -> Option<Advice> {
    let mut messages = vec![];

    let [unit_cnt, module_cnt, func_cnt] = count_elements(system);
    messages.push(format!("ユニット : {} 定義済", unit_cnt));
    messages.push(format!("モジュール : {} 定義済", module_cnt));
    messages.push(format!("関数 : {} 定義済", func_cnt));

    Some(
        Advice::new(
            AdviceLevel::Info,
            "基本情報".to_string(),
            messages
        )
    )
}

fn count_elements(system: &SysDCSystem) -> [i32; 3] {
    let unit_cnt = system.units.len() as i32;

    let other_cnt = system.units.iter().fold([0, 0], |cnt, unit| {
        let module_cnt = cnt[0] + unit.modules.len() as i32;
        let func_cnt = unit.modules.iter().fold(cnt[1], |cnt, module| {
            cnt + module.functions.len() as i32
        });
        [module_cnt, func_cnt]
    });

    [unit_cnt, other_cnt[0], other_cnt[1]]  // unit, module, function
}
