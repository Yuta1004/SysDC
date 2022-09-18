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
            app.manage(SysDCSystemManager::new(system));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            command::get_system
        ])
        .run(tauri::generate_context!())?;

    Ok(())
}

pub struct SysDCSystemManager {
    system: Arc<SysDCSystem>
}

impl SysDCSystemManager {
    pub fn new(system: SysDCSystem) -> SysDCSystemManager {
        SysDCSystemManager { system: Arc::new(system) }
    }

    pub fn get(&self) -> Arc<SysDCSystem> {
        Arc::clone(&self.system)
    }
}
