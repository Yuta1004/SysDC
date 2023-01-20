#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{LogicalSize, Manager, Size};

pub fn exec() -> anyhow::Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            app.get_window("main")
                .unwrap()
                .set_size(Size::Logical(LogicalSize {
                    width: 1024.0,
                    height: 768.0,
                }))?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())?;

    Ok(())
}
