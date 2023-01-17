#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::{ wasm_bindgen, JsValue };

#[cfg(feature = "wasm")]
use sysdc_core::structure::SysDCSystem;

use sysdc_core::types::TypeKind;
use sysdc_core::structure::SysDCModule;

#[cfg(feature = "wasm")]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn flistup(system: JsValue) -> JsValue {
    let system: SysDCSystem = match serde_wasm_bindgen::from_value(system) {
        Ok(system) => system,
        Err(_) => return serde_wasm_bindgen::to_value::<Vec<()>>(&vec![]).unwrap()
    };

    let found_fs = system.units.iter().fold(vec![], |found_fs, unit| {
        unit.modules.iter().fold(found_fs, |mut found_fs, module| {
            found_fs.extend(get_functions(module).into_iter());
            found_fs
        })
    });

    serde_wasm_bindgen::to_value(&found_fs).unwrap()
}

#[cfg_attr(not(feature = "wasm"), allow(dead_code))]
fn get_functions(module: &SysDCModule) -> Vec<(&str, String)> {
    module.functions.iter().fold(vec![], |mut found_fs, func| {
        let f = match func.returns.1.kind {
            TypeKind::Void => ("Proc", func.name.get_full_name()),
            _ => ("Func", func.name.get_full_name())
        };
        found_fs.push(f);
        found_fs
    })
}
