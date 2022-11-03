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
        let system = UnitParser::parse(tokenizer)?;
        self.units.push(system);
        Ok(())
    }

    pub fn check(self) -> anyhow::Result<SysDCSystem> {
        let system = unchecked::SysDCSystem::new(self.units);
        check::check(system)
    }
}
