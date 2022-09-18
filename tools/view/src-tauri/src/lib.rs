#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;

use std::sync::Arc;

use tauri::Manager;

use sysdc_parser::structure::SysDCSystem;

pub fn exec(system: SysDCSystem) -> anyhow::Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(SysDCSystemWrapper::new(system));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            command::get_system
        ])
        .run(tauri::generate_context!())?;

    Ok(())
}

pub struct SysDCSystemWrapper {
    system: Arc<SysDCSystem>
}

impl SysDCSystemWrapper {
    pub fn new(system: SysDCSystem) -> SysDCSystemWrapper {
        SysDCSystemWrapper { system: Arc::new(system) }
    }

    pub fn get(&self) -> Arc<SysDCSystem> {
        Arc::clone(&self.system)
    }
}
