use super::name::Name;
use super::token::{ Token, TokenKind, Tokenizer };
use super::structure::{ SysDCSystem, SysDCLayer, SysDCUnit };

pub struct Parser<'a> {
    pub namespace: Name,
    pub unit_name: String,
    pub layer_num: i32,
    tokenizer: Tokenizer<'a>
}

impl<'a> Parser<'a> {
    pub fn new(namespace: &Name, unit_name: &String, tokenizer: Tokenizer<'a>) -> Parser<'a> {
        Parser {
            namespace: namespace.clone(),
            unit_name: unit_name.clone(),
            layer_num: 0,
            tokenizer
        }
    }

    pub fn parse(&mut self) -> SysDCUnit {
        let layer = self.parse_layer(&self.namespace.clone());
        let mut unit = SysDCUnit::new(&layer.name, &self.unit_name);
        unit
    }

    fn parse_layer(&mut self, namespace: &Name) -> SysDCLayer {
        self.expect(TokenKind::Layer);
        let num_token = self.expect(TokenKind::Number);
        self.expect(TokenKind::Semicolon);
        SysDCLayer::new(&namespace, num_token.get_number())
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
    use super::Name;
    use super::{ Tokenizer, Parser };
    use super::{ SysDCSystem, SysDCLayer, SysDCUnit };

    #[test]
    fn parse_simple_unit() {
        let program = "layer 0;";

        let unit = generate_test_unit(0);

        compare_unit(parse(program), unit);
    }

    fn compare_unit(a: SysDCUnit, b: SysDCUnit) {
        assert_eq!(format!("{:?}", a), format!("{:?}", b));
    }

    fn generate_test_unit(layer_num: i32) -> SysDCUnit {
        let root_namespace = Name::new_root();
        let layer_namespace = Name::new(&root_namespace, &format!("layer{}", layer_num));
        SysDCUnit::new(&layer_namespace, &"test".to_string()) 
    }

    fn parse(program: &str) -> SysDCUnit {
        let program = program.to_string();
        let tokenizer = Tokenizer::new(&program);
        let mut parser = Parser::new(&Name::new_root(), &"test".to_string(), tokenizer);
        parser.parse()
    }
}
