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
    pub fn run(&self) {
        match self.read_files() {
            Ok(system) => self.save_system(system),
            Err(err) => println!("[ERROR] {}", err)
        }
    }

    fn read_files(&self) -> Result<SysDCSystem, Box<dyn Error>> {
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
                        parser.parse(program)?;
                        println!("Load: {}", filename);
                    }
                }
            }
        }
        parser.check()
    }

    fn save_system(&self, system: SysDCSystem) {
        let mut serialized_system = vec!();
        system.serialize(&mut Serializer::new(&mut serialized_system)).unwrap();

        let mut f = File::create(&self.output).unwrap();
        f.write_all(&serialized_system).unwrap();
        f.flush().unwrap();
    }
}
