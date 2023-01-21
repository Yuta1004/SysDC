use std::fs;
use std::fs::File;
use std::io::Write;

use clap::Parser;
use rmp_serde::Serializer;
use serde::Serialize;

use sysdc_core::Parser as SParser;
use sysdc_core::structure::SysDCSystem;

#[derive(Parser)]
pub struct ParseCmd {
    #[clap(required = true)]
    input: Vec<String>,

    #[clap(short, long, default_value = "out.sysdc")]
    output: String,
}

impl ParseCmd {
    pub fn run(&self) -> anyhow::Result<()> {
        self.save_system(self.read_files()?)
    }

    fn read_files(&self) -> anyhow::Result<SysDCSystem> {
        let mut load_unit_cnt = 0;
        let mut parser = SParser::default();
        for filename in &self.input {
            for entry in glob::glob(filename)?.flatten() {
                if entry.is_file() {
                    let filename = entry.file_name().unwrap().to_str().unwrap().to_string();
                    let program = fs::read_to_string(&entry)?;
                    println!("Loading: {}", filename);
                    parser.parse(filename, &program)?;
                    load_unit_cnt += 1;
                }
            }
        }
        let system = parser.check()?;
        println!("{} units loaded!", load_unit_cnt);
        Ok(system)
    }

    fn save_system(&self, system: SysDCSystem) -> anyhow::Result<()> {
        let mut serialized_system = vec![];
        system.serialize(&mut Serializer::new(&mut serialized_system))?;

        let mut f = File::create(&self.output)?;
        f.write_all(&serialized_system)?;
        Ok(f.flush()?)
    }
}
