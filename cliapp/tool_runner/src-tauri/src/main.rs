#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub fn main() -> anyhow::Result<()> {
    let system = serde_json::from_str("{ units: [] }")?;
    sysdc_tool_runner::exec(system)
}
