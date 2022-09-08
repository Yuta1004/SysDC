mod parse;
mod token;
mod check;
mod error;
mod location;
pub mod name;
pub mod types;
pub mod structure;

use parse::UnitParser;
use structure::unchecked;
use structure::SysDCSystem;
use token::Tokenizer;

#[derive(Default)]
pub struct Parser {
    units: Vec<unchecked::SysDCUnit>,
}

impl Parser {
    pub fn parse(&mut self, filename: String, program: &str) -> anyhow::Result<()> {
        let tokenizer = Tokenizer::new(filename, program);
        match UnitParser::parse(tokenizer) {
            Ok(unit) => {
                self.units.push(unit);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn check(self) -> anyhow::Result<SysDCSystem> {
        match check::check(unchecked::SysDCSystem::new(self.units)) {
            Ok(system) => Ok(system),
            Err(err) => Err(err),
        }
    }
}
