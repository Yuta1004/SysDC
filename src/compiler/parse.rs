use super::structure::{ SysDCSystem };
use super::token::Tokenizer;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: Tokenizer<'a>) -> Parser<'a> {
        Parser { tokenizer }
    }

    pub fn parse(&mut self) -> SysDCSystem {
        let system = SysDCSystem::new();
        system
    }
}

#[cfg(test)]
mod test {
    use super::{ Tokenizer, Parser };
    use super::{ SysDCSystem };

    #[test]
    fn parse_system() {
        let program = "".to_string();
        let tokenizer = Tokenizer::new(&program);
        let mut parser = Parser::new(tokenizer);
        compare_system(parser.parse(), SysDCSystem::new());
    }

    fn compare_system(a: SysDCSystem, b: SysDCSystem) {
        assert_eq!(format!("{:?}", a), format!("{:?}", b));
    }
}
