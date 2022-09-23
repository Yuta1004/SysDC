#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;
mod react_flow;

use tauri::{LogicalSize, Manager, Size};

use sysdc_parser::structure::SysDCSystem;

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
            command::get_system::get_system,
            command::gen_flow::gen_flow,
        ])
        .run(tauri::generate_context!())?;

    Ok(())
}
