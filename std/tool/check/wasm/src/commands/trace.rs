use serde::{ Serialize, Deserialize };

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::{ wasm_bindgen, JsValue };

use sysdc_core::structure::{ SysDCSystem, SysDCFunction, SysDCAnnotation, SysDCSpawnDetail };

#[derive(Debug, Serialize, Deserialize)]
enum TraceResult {
    ReturnVar,                              // 返り値として採用される
    ModifyVarL { vars: Vec<String> },       // 他の変数によって値が更新される
    SpawnVarL { vars: Vec<String> },        // 他の変数によって値が生成される
    Affect { func: String, arg_to: String } // 自身の値を使用して他の関数に影響を与える
}

#[cfg(feature = "wasm")]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn trace(system: JsValue, fname: String) -> JsValue {
    let system = match serde_wasm_bindgen::from_value::<SysDCSystem>(system) {
        Ok(system) => system,
        Err(_) => return JsValue::default()
    };

    let func = match pick_funcion(&system, &fname) {
        Some(func) => func,
        None => return JsValue::default()
    };

    let trace_results = if func.returns.0.name == "0" {
        vec![] 
    } else {
        vec![(
            func.returns.0.get_full_name(),
            __trace_var(&system, func.returns.0.get_full_name())
        )]
    };
    let trace_results = func.args.iter().fold(trace_results, |mut trace_results, (n, _)| {
        trace_results.push((n.get_full_name(), __trace_var(&system, n.get_full_name())));
        trace_results
    });
    serde_wasm_bindgen::to_value(&trace_results).unwrap()
}

#[cfg(feature = "wasm")]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn trace_var(system: JsValue, var_name: String) -> JsValue {
    let system = match serde_wasm_bindgen::from_value::<SysDCSystem>(system) {
        Ok(system) => system,
        Err(_) => return JsValue::default()
    };
    serde_wasm_bindgen::to_value(&__trace_var(&system, var_name)).unwrap()
}

#[cfg_attr(not(feature = "wasm"), allow(dead_code))]
fn __trace_var(system: &SysDCSystem, var_name: String) -> Vec<TraceResult> {
    let func = match pick_funcion(system, &var_name) {
        Some(func) => func,
        None => return vec![]
    };

    // ReturnVar
    let mut trace_results = vec![];
    if func.returns.0.get_full_name() == var_name {
        trace_results.push(TraceResult::ReturnVar)
    }

    // ModifyVarL, SpawnVarL, Affect
    let _trace_results = func.annotations.iter().filter_map(|anno| {
        match anno {
            SysDCAnnotation::Affect { func: (afname, _), args } => {
                let arg_idx = args.iter().enumerate().find_map(|(idx, var)| {
                    if var.0.get_full_name() == var_name {
                        Some(idx)
                    } else {
                        None
                    }
                });
                let afunc = pick_funcion(system, &afname.get_full_name()).unwrap();
                if let Some(arg_idx) = arg_idx {
                    Some(TraceResult::Affect {
                        func: afname.get_full_name(),
                        arg_to: afunc.args.get(arg_idx).unwrap().0.get_full_name()
                    })
                } else {
                    None
                }
            },
            SysDCAnnotation::Modify { target: (mname, _), uses} => {
                if mname.get_full_name() == var_name {
                    let vars = uses.iter().map(|(n, _)| n.get_full_name()).collect();
                    Some(TraceResult::ModifyVarL { vars })
                } else {
                    None
                }
            },
            SysDCAnnotation::Spawn { result: (rname, _), details } => {
                if rname.get_full_name() == var_name {
                    let vars = details.iter().filter_map(|detail| {
                        match detail {
                            SysDCSpawnDetail::Use(n, _) => Some(n.get_full_name()),
                            _ => None
                        }
                    }).collect();
                    Some(TraceResult::ModifyVarL { vars })
                } else {
                    None
                }
            }
        }
    }).collect::<Vec<TraceResult>>();

    trace_results.extend(_trace_results.into_iter());
    trace_results
}

#[cfg_attr(not(feature = "wasm"), allow(dead_code))]
fn pick_funcion<'a>(system: &'a SysDCSystem, fname: &String) -> Option<&'a SysDCFunction> {
    let unit = system.units.iter().find(|unit| {
        fname.starts_with(&unit.name.get_full_name())
    })?;

    let module  = unit.modules.iter().find(|module| {
        fname.starts_with(&module.name.get_full_name())
    })?;

    module.functions.iter().find(|func| {
        fname.starts_with(&func.name.get_full_name())
    })
}
