use anyhow;

use sysdc_parser::structure::SysDCSystem;

pub fn exec(system: &SysDCSystem) -> anyhow::Result<()> {
    println!("{:?}", system);
    Ok(())
}
