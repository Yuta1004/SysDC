mod parse;
mod token;
mod check;
mod error;
pub mod name;
pub mod types;
pub mod structure;

use std::error::Error;

use name::Name;
use token::Tokenizer;
use parse::UnitParser;
use check::Checker;
use structure::SysDCSystem;
use structure::unchecked;

pub struct Parser {
    units: Vec<unchecked::SysDCUnit>
}

impl Parser {
    pub fn new() -> Parser {
        Parser { units: vec!() }
    }

    pub fn parse(&mut self, program: String) -> Result<(), Box<dyn Error>> {
        let root_name = Name::new_root();
        let tokenizer = Tokenizer::new(&program);
        self.units.push(UnitParser::parse(tokenizer, root_name)?);
        Ok(())
    }

    pub fn check(self) -> Result<SysDCSystem, Box<dyn Error>> {
        match Checker::check(unchecked::SysDCSystem::new(self.units)) {
            Ok(system) => Ok(system),
            Err(err) => err.upgrade()
        }
    }
}

#[cfg(test)]
mod test {
    use super::Parser;

    #[test]
    fn parse() {
        let mut parser = Parser::new();
        let programs = [
            "unit test.A; data A {}",
            "unit test.B; data B {}",
            "unit test.C; data C {}",
            "unit test.D; data D {}",
            "unit test.E; data E {}"
        ];
        for program in programs {
            parser.parse(program.to_string()).unwrap();
        }
        parser.check().unwrap();
    }
}
