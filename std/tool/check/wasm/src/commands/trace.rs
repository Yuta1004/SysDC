#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::{ wasm_bindgen, JsValue };

use sysdc_core::structure::{ SysDCSystem, SysDCFunction };

#[cfg(feature = "wasm")]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn trace(system: JsValue, fname: String) -> JsValue {
    let system = match serde_wasm_bindgen::from_value::<SysDCSystem>(system) {
        Ok(system) => system,
        Err(_) => return JsValue::default()
    };

    let func = match pick_funcion(&system, fname) {
        Some(func) => func,
        None => return JsValue::default()
    };

    serde_wasm_bindgen::to_value(&func).unwrap()
}

fn pick_funcion(system: &SysDCSystem, fname: String) -> Option<&SysDCFunction> {
    let unit = system.units.iter().find(|unit| {
        fname.starts_with(&unit.name.get_full_name())
    })?;

    let module  = unit.modules.iter().find(|module| {
        fname.starts_with(&module.name.get_full_name())
    })?;

    module.functions.iter().find(|func| {
        fname == func.name.get_full_name()
    })
}
