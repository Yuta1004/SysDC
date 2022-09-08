use std::fs::File;
use std::io::Write;

use sysdc_parser::structure::SysDCSystem;

pub fn exec(system: &SysDCSystem, args: &Vec<String>) -> anyhow::Result<()> {
    let serialized_system = serde_json::to_string(system)?;
    match args.len() {
        0 => println!("{}", serialized_system),
        _ => {
            let mut f = File::create(&args[0])?;
            write!(f, "{}", serialized_system)?;
            f.flush()?;
        }
    }
    Ok(())
}
