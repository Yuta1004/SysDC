use std::io::Write;
use std::fs;
use std::fs::File;
use std::error::Error;

use clap::Parser;
use serde::Serialize;
use rmp_serde::Serializer;

use sysdc_parser::Parser as SParser;
use sysdc_parser::structure::SysDCSystem;

#[derive(Parser)]
#[clap(name="subcommand")]
pub struct ParseCmd {
    #[clap(required=true)]
    input: Vec<String>,

    #[clap(short, long, default_value="out.sysdc")]
    output: String
}

impl ParseCmd {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        self.save_system(self.read_files()?)
    }

    fn read_files(&self) -> Result<SysDCSystem, Box<dyn Error>> {
        let mut load_unit_cnt = 0;
        let mut parser = SParser::new();
        for filename in &self.input {
            for entries in glob::glob(&filename) {
                for entry in entries {
                    let entry = entry.unwrap();
                    if entry.is_file() {
                        let filename = entry.file_name().unwrap()
                                            .to_str().unwrap()
                                            .to_string();
                        let program = fs::read_to_string(&entry)?;
                        println!("Loading: {}", filename);
                        parser.parse(filename, &program)?;
                        load_unit_cnt += 1;
                    }
                }
            }
        }
        let system = parser.check()?;
        println!("{} units loaded!", load_unit_cnt);
        Ok(system)
    }

    fn save_system(&self, system: SysDCSystem) -> Result<(), Box<dyn Error>> {
        let mut serialized_system = vec!();
        system.serialize(&mut Serializer::new(&mut serialized_system))?;

        let mut f = File::create(&self.output)?;
        f.write_all(&serialized_system)?;
        Ok(f.flush()?)
    }
}
