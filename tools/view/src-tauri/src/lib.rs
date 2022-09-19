#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;
mod react_flow;

use tauri::Manager;

use sysdc_parser::structure::SysDCSystem;

pub fn exec(system: SysDCSystem) -> anyhow::Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(system);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![command::simple::get_flow])
        .run(tauri::generate_context!())?;

    Ok(())
}
