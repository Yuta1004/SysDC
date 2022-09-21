use tauri::State;

use sysdc_parser::structure::SysDCSystem;

#[tauri::command]
pub fn get_system(system: State<'_, SysDCSystem>) -> SysDCSystem {
    (*system).clone()
}
