use tauri::State;

use sysdc_parser::structure::SysDCSystem;

use crate::SysDCSystemManager;

#[tauri::command]
pub fn get_system(manager: State<'_, SysDCSystemManager>) -> SysDCSystem {
    (*manager.get()).clone()
}
