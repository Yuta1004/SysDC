use std::fs;

use clap::Parser;

use sysdc_core::structure::SysDCSystem;

#[derive(Parser)]
pub struct RunCmd {
    #[clap(short, long, default_value = "out.sysdc")]
    input: String,
}

impl RunCmd {
    pub fn run(&self) -> anyhow::Result<()> {
        let system = self.load_system()?;
        sysdc_tool_runner::exec(system)
    }

    fn load_system(&self) -> anyhow::Result<SysDCSystem> {
        let serialized_system = fs::read(&self.input)?;
        Ok(rmp_serde::from_slice::<SysDCSystem>(
            &serialized_system[..],
        )?)
    }
}
