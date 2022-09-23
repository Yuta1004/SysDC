use sysdc_parser::structure::SysDCSystem;

pub fn exec(system: &SysDCSystem) -> anyhow::Result<()> {
    let serialized_system = serde_json::to_string(system)?;
    println!("{}", serialized_system);
    Ok(())
}
