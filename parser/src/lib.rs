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
        self.units.push(UnitParser::parse(tokenizer)?);
        Ok(())
    }

    pub fn check(self) -> anyhow::Result<SysDCSystem> {
        check::check(unchecked::SysDCSystem::new(self.units))
    }
}
