pub mod name;
pub mod parse;
pub mod structure;
pub mod token;
pub mod types;

use std::collections::HashMap;

use parse::Parser;
use token::Tokenizer;
use structure::{ SysDCSystem, SysDCLayer };

pub struct Compiler {
    system: SysDCSystem,
    layers: HashMap<i32, SysDCLayer>
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            system: SysDCSystem::new(),
            layers: HashMap::new()
        }
    }

    pub fn add_unit(&mut self, unit_name: String, program: &String) {
        let tokenizer = Tokenizer::new(program);
        let mut parser = Parser::new(self.system.name.clone(), unit_name, tokenizer);

        let (layer_num, unit) = parser.parse();
        if !self.layers.contains_key(&layer_num) {
            self.layers.insert(layer_num, SysDCLayer::new(&self.system.name, layer_num));
        }
        self.layers.get_mut(&layer_num).unwrap().push_units(unit);
    }

    pub fn generate_system(self) -> SysDCSystem {
        let mut system = self.system;
        let mut layers = self.layers;
        for layer_num in layers.keys().map(|e| e.clone()).collect::<Vec<i32>>() {
            system.push_layer(layers.remove(&layer_num).unwrap());
        }
        system  // TODO: Connector
    }
}

#[cfg(test)]
mod test {
    use super::Compiler;

    #[test]
    fn compile() {
        let mut compiler = Compiler::new();
        let programs = [
            ("user1", "layer 0; data User1 {}"),
            ("user2", "layer 0; data User2 {}"),
            ("user3", "layer 0; data User3 {}"),
            ("user4", "layer 1; data User4 {}"),
            ("user5", "layer 2; data User5 {}")
        ];
        for (unit_name, program) in programs {
            compiler.add_unit(unit_name.to_string(), &program.to_string());
        }
        let system = compiler.generate_system();
        assert_eq!(system.layers.len(), 3);
    }
}
