use super::name::Name;
use super::token::{ Token, TokenKind, Tokenizer };
use super::structure::{ SysDCSystem, SysDCLayer };

pub struct Parser<'a> {
    name: String,
    tokenizer: Tokenizer<'a>
}

impl<'a> Parser<'a> {
    pub fn new(name: String, tokenizer: Tokenizer<'a>) -> Parser<'a> {
        Parser { name, tokenizer }
    }

    pub fn parse(&mut self) -> SysDCSystem {
        let mut system = SysDCSystem::new();
        system.push_layer(self.parse_layer(&system.name));
        system
    }

    fn parse_layer(&mut self, namespace: &Name) -> SysDCLayer {
        self.expect(TokenKind::Layer);
        let num_token = self.expect(TokenKind::Number);
        self.expect(TokenKind::Semicolon);
        SysDCLayer::new(namespace, num_token.get_number())
    }

    fn expect(&mut self, kind: TokenKind) -> Token {
        match self.tokenizer.expect_kind(kind.clone()) {
            Some(token) => token,
            None => panic!("[ERROR] Token {:?} is expected, but not found.", kind)
        }
    }
}

#[cfg(test)]
mod test {
    use super::{ Tokenizer, Parser };
    use super::{ SysDCSystem, SysDCLayer };

    #[test]
    fn parse_system() {
        let program = "layer 0;".to_string();

        let mut system = SysDCSystem::new();
        system.push_layer(SysDCLayer::new(&system.name, 0));

        let tokenizer = Tokenizer::new(&program);
        let mut parser = Parser::new("test".to_string(), tokenizer);
        compare_system(parser.parse(), system);
    }

    fn compare_system(a: SysDCSystem, b: SysDCSystem) {
        assert_eq!(format!("{:?}", a), format!("{:?}", b));
    }
}
