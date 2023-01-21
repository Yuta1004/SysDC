#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod command;

use tauri::{LogicalSize, Manager, Size};

use sysdc_core::structure::SysDCSystem;

pub fn exec(system: SysDCSystem) -> anyhow::Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(system);
            app.get_window("main")
                .unwrap()
                .set_size(Size::Logical(LogicalSize {
                    width: 1024.0,
                    height: 768.0,
                }))?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            command::get_system
        ])
        .run(tauri::generate_context!())?;

    Ok(())
}
