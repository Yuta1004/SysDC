mod parse;
mod token;
mod check;
mod error;
mod location;
mod name;
mod types;
mod structure;

#[cfg(wasm)]
use {
    wasm_bindgen::prelude::wasm_bindgen,
    wasm_bindgen::JsValue,
};

use parse::UnitParser;
use structure::unchecked;
use token::Tokenizer;

#[cfg(wasm)]
macro_rules! q {
    ($target:expr) => {
        match $target {
            Ok(target) => target,
            Err(err) => return Err(err.to_string()),
        }
    };
}

#[cfg_attr(wasm, wasm_bindgen)]
#[derive(Default)]
pub struct Parser {
    units: Vec<unchecked::SysDCUnit>,
}

#[cfg(wasm)]
impl Parser {
    pub fn parse(&mut self, filename: String, program: &str) -> Result<(), String> {
        let tokenizer = Tokenizer::new(filename, program);
        let unit = q!(UnitParser::parse(tokenizer));
        self.units.push(unit);
        Ok(())
    }

    pub fn check(self) -> Result<JsValue, String> {
        let system = unchecked::SysDCSystem::new(self.units);
        let system = q!(check::check(system));
        Ok(serde_wasm_bindgen::to_value(&system).unwrap())
    }
}

#[cfg(not(wasm))]
impl Parser {
    pub fn parse(&mut self, filename: String, program: &str) -> anyhow::Result<()> {
        let tokenizer = Tokenizer::new(filename, program);
        let unit = UnitParser::parse(tokenizer)?;
        self.units.push(unit);
        Ok(())
    }

    pub fn check(self) -> anyhow::Result<structure::SysDCSystem> {
        let system = unchecked::SysDCSystem::new(self.units);
        check::check(system)
    }
}
