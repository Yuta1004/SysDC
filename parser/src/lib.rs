mod util;
mod parse;
mod token;
mod check;
mod error;
pub mod name;
pub mod types;
pub mod structure;

use std::error::Error;

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

    pub fn parse(&mut self, filename: String, program: &String) -> Result<(), Box<dyn Error>> {
        let tokenizer = Tokenizer::new(program);
        match UnitParser::parse(tokenizer) {
            Ok(unit) => {
                self.units.push(unit);
                Ok(())
            },
            Err(mut err) => {
                err.set_filename(filename);
                err.upgrade()
            }
        }
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
            ("A.def", "unit test.A; data A {}"),
            ("B.def", "unit test.B; data B {}"),
            ("C.def", "unit test.C; data C {}"),
            ("D.def", "unit test.D; data D {}"),
            ("E.def", "unit test.E; data E {}")
        ];
        for (filename, program) in programs {
            parser.parse(filename.to_string(), &program.to_string()).unwrap();
        }
        parser.check().unwrap();
    }
}
