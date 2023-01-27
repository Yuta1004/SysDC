use std::fmt::{ Display, Formatter };

use crate::{ Advice, AdviceLevel };
use sysdc_core::name::Name;
use sysdc_core::structure::{ SysDCSystem, SysDCModule, SysDCFunction, SysDCAnnotation };

pub fn eval_complex_stat(system: &SysDCSystem) -> Option<Advice> {
    let messages = system.units.iter().fold(vec![], |messages, unit| {
        unit.modules.iter().fold(messages, |mut messages, module| {
            let advice = eval_complex_stat_module(module);
            let _messages = advice.iter().map(|adv| adv.to_string()).collect::<Vec<String>>();
            messages.extend(_messages.into_iter());
            messages
        })
    });

    match messages.len() {
        0 => None,
        _ => Some(
            Advice::new(
                AdviceLevel::Warning,
                "分割可能な処理".to_string(),
                messages
            )
        )

    }
}

enum ComplexAdviceKind {
    TooManySpawning,   // spawn
    TooManyModifying,  // modify
    TooManyArguments,
    TooManyFunctions,
}

struct ComplexAdvice<'a> {
    kind: ComplexAdviceKind,
    name: &'a Name,
}

impl<'a> ComplexAdviceKind {
    pub fn new(self, name: &'a Name) -> ComplexAdvice<'a> {
        ComplexAdvice { kind: self, name }
    }
}

impl<'a> Display for ComplexAdvice<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} : ", self.name.get_full_name())?;
        match self.kind {
            ComplexAdviceKind::TooManySpawning =>
                write!(f, "1つの関数内で作成する変数が多すぎる可能性があります"),
            ComplexAdviceKind::TooManyModifying =>
                write!(f, "1つの関数が更新対象としている変数が多すぎる可能性があります"),
            ComplexAdviceKind::TooManyArguments =>
                write!(f, "引数が多すぎる可能性があります"),
            ComplexAdviceKind::TooManyFunctions =>
                write!(f, "1つのモジュールに属する関数が多すぎる可能性があります")
        }
    }
}

fn eval_complex_stat_module(module: &SysDCModule) -> Vec<ComplexAdvice> {    // TODO: 固定値を無くす
    let mut advice = vec![];

    // TooManyFunctions
    let func_cnt = module.functions.len() as i32;
    if func_cnt > 12 {
        advice.push(ComplexAdviceKind::TooManyFunctions.new(&module.name));
    }

    // TooMany(Spawning|Modifying|Arguments)
    module.functions.iter().fold(advice, |mut advice, func| {
        let mut fadvice = eval_complex_stat_func(func);
        if fadvice.len() > 0 {
            advice.append(&mut fadvice);
        }
        advice
    })
}

fn eval_complex_stat_func(func: &SysDCFunction) -> Vec<ComplexAdvice> {      // TODO: 固定値を無くす
    let mut advice = vec![];

    // TooManySpawning
    let spawning_cnt = func.annotations
        .iter()
        .filter(|anno| matches!(anno, SysDCAnnotation::Spawn { .. }))
        .count();
    if spawning_cnt > 6 {
        advice.push(ComplexAdviceKind::TooManySpawning.new(&func.name));
    }

    // TooManyModifying
    let modifying_cnt = func.annotations
        .iter()
        .filter(|anno| matches!(anno, SysDCAnnotation::Modify { .. }))
        .count();
    if modifying_cnt > 6 {
        advice.push(ComplexAdviceKind::TooManyModifying.new(&func.name));
    }

    // TooManyArguments
    let arg_cnt = func.args.len() as i32;
    if arg_cnt > 6 {
        advice.push(ComplexAdviceKind::TooManyArguments.new(&func.name));
    }

    advice
}
