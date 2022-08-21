mod parse;
mod token;
mod check;
mod error;
pub mod name;
pub mod types;
pub mod structure;

use std::error::Error;

use name::Name;
use parse::Parser;
use token::Tokenizer;
use check::Checker;
use structure::SysDCSystem;
use structure::unchecked;

pub struct Compiler {
    units: Vec<unchecked::SysDCUnit>
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler { units: vec!() }
    }

    pub fn add_unit(&mut self, unit_name: String, program: String) -> Result<(), Box<dyn Error>> {
        self.units.push(Parser::parse(Tokenizer::new(&program), Name::new_root())?);
        Ok(())
    }

    pub fn generate_system(self) -> Result<SysDCSystem, Box<dyn Error>> {
        Checker::check(unchecked::SysDCSystem::new(self.units))
    }
}

#[cfg(test)]
mod test {
    use super::Compiler;

    #[test]
    fn compile() {
        let mut compiler = Compiler::new();
        let programs = [
            ("A", "unit test.A; data A {}"),
            ("B", "unit test.B; data B {}"),
            ("C", "unit test.C; data C {}"),
            ("D", "unit test.D; data D {}"),
            ("E", "unit test.E; data E {}")
        ];
        for (unit_name, program) in programs {
            compiler.add_unit(unit_name.to_string(), program.to_string()).unwrap();
        }
        compiler.generate_system().unwrap();
    }
}
