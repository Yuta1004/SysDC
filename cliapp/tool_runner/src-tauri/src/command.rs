use tauri::State;

use sysdc_core::structure::SysDCSystem;

#[tauri::command]
pub fn get_system(system: State<SysDCSystem>) -> SysDCSystem {
    (*system).clone()
}
