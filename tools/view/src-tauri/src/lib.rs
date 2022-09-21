#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;
mod react_flow;

use tauri::{LogicalPosition, Manager, Position};

use sysdc_parser::structure::SysDCSystem;

pub fn exec(system: SysDCSystem) -> anyhow::Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(system);

            let window = app.get_window("main").unwrap();
            let pwsize = *window.current_monitor()?.unwrap().size();
            window.set_size(pwsize)?;
            window.set_position(Position::Logical(
                LogicalPosition { x: 0.0, y: 0.0 }
            ))?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            command::get_system::get_system,
            command::gen_flow::gen_flow,
        ])
        .run(tauri::generate_context!())?;

    Ok(())
}
