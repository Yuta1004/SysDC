use tauri::State;

use sysdc_parser::structure::SysDCSystem;

use crate::SysDCSystemWrapper;

#[tauri::command]
pub fn get_system(manager: State<'_, SysDCSystemWrapper>) -> SysDCSystem {
    (*manager.get()).clone()
}
