#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::thread;
use std::time::Duration;

use tauri::Manager;

use sysdc_parser::structure::SysDCSystem;

pub fn exec(system: SysDCSystem) -> anyhow::Result<()> {
    tauri::Builder::default()
        .setup(move |app| {
            let app = app.app_handle();
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(1));
                app.emit_all("initialize_system", system)
            });
            Ok(())
        })
        .run(tauri::generate_context!())?;
    Ok(())
}
