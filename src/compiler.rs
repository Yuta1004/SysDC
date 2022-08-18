mod parse;
mod token;
mod check;
pub mod name;
pub mod types;
pub mod structure;

use name::Name;
use parse::Parser;
use token::Tokenizer;
use check::Checker;
use structure::{ SysDCSystem, SysDCUnit };

pub struct Compiler {
    units: Vec<SysDCUnit>
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler { units: vec!() }
    }

    pub fn add_unit(&mut self, unit_name: String, program: &String) {
        let name = Name::from(&Name::new_root(), unit_name);
        let tokenizer = Tokenizer::new(program);
        let mut parser = Parser::new(tokenizer);
        let unit = parser.parse(&name);
        self.units.push(unit);
    }

    pub fn generate_system(self) -> SysDCSystem {
        Checker::check(SysDCSystem::new(self.units))
    }
}

#[cfg(test)]
mod test {
    use super::Compiler;

    #[test]
    fn compile() {
        let mut compiler = Compiler::new();
        let programs = [
            ("A", "data A {}"),
            ("B", "data B {}"),
            ("C", "data C {}"),
            ("D", "data D {}"),
            ("E", "data E {}")
        ];
        for (unit_name, program) in programs {
            compiler.add_unit(unit_name.to_string(), &program.to_string());
        }
        compiler.generate_system();
    }
}
