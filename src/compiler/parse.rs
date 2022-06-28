use std::rc::Rc;
use std::cell::RefCell;

use super::name::Name;
use super::types::SysDCType;
use super::token::{ Token, TokenKind, Tokenizer };
use super::structure::{ SysDCSystem, SysDCLayer, SysDCUnit, SysDCData, SysDCVariable };

struct TmpType {
    name: String
}

impl TmpType {
    pub fn new(name: &String) -> Rc<TmpType> {
        Rc::new(
            TmpType { name: name.to_string() }
        )
    }
}

impl SysDCType for TmpType {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_full_name(&self) -> String {
        self.name.clone()
    }
}

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

    /**
     * <root> ::= {<sentence>}
     * <sentence> ::= <layer> {<data> | <module>}
     */
    pub fn parse(&mut self) -> SysDCUnit {
        let layer = self.parse_layer(&self.namespace.clone());
        let mut unit = SysDCUnit::new(&layer.name, &self.unit_name);
        while self.tokenizer.has_token() {
            if let Some(data) = self.parse_data(&unit.name) {
                unit.push_data(data);
            }
            println!("{}", self.tokenizer.has_token());
        }
        unit
    }

    /**
     * <layer> :: = layer <num> ;
     */
    fn parse_layer(&mut self, namespace: &Name) -> SysDCLayer {
        self.tokenizer.request(TokenKind::Layer);
        let num_token = self.tokenizer.request(TokenKind::Number);
        self.tokenizer.request(TokenKind::Semicolon);
        SysDCLayer::new(&namespace, num_token.get_number())
    }

    /**
     * <data> ::= data \{ {<id_type_mapping_var>} \} 
     */
    fn parse_data(&mut self, namespace: &Name) -> Option<Rc<RefCell<SysDCData>>> {
        if self.tokenizer.expect(TokenKind::Data).is_none() {
            return None;
        }

        let data = SysDCData::new(namespace, &self.tokenizer.request(TokenKind::Identifier).get_id());
        self.tokenizer.request(TokenKind::BracketBegin);
        loop {
            let var = self.parse_id_type_mapping_var(&data.borrow().name);
            data.borrow_mut().push_variable(var);

            if self.tokenizer.expect(TokenKind::BracketEnd).is_some() {
                break;
            } else {
                self.tokenizer.request(TokenKind::Separater);
            }
        }
        Some(data)
    }

    /**
     * <id_type_mapping_var> ::= <id> : <type> 
     */
    fn parse_id_type_mapping_var(&mut self, namespace: &Name) -> Rc<RefCell<SysDCVariable>> {
        let id = self.tokenizer.request(TokenKind::Identifier).get_id();
        self.tokenizer.request(TokenKind::Mapping);
        let types = self.tokenizer.request(TokenKind::Identifier).get_id();
        SysDCVariable::new(namespace, &id, TmpType::new(&types))
    }
}

#[cfg(test)]
mod test {
    use super::Name;
    use super::{ TmpType, Tokenizer, Parser };
    use super::{ SysDCSystem, SysDCLayer, SysDCUnit, SysDCData, SysDCVariable };

    #[test]
    fn parse_simple_unit() {
        let program = "layer 0;";

        let unit = generate_test_unit(0);

        compare_unit(parse(program), unit);
    }

    #[test]
    fn parse_data() {
        let program = "
            layer 0;
            data User {
                id: int32,
                age: int32,
                name: string
            }
        ";

        let mut unit = generate_test_unit(0);
        let data = SysDCData::new(&unit.name, &"User".to_string());
        let id = SysDCVariable::new(&data.borrow().name, &"id".to_string(), TmpType::new(&"int32".to_string()));
        let age = SysDCVariable::new(&data.borrow().name, &"age".to_string(), TmpType::new(&"int32".to_string()));
        let name = SysDCVariable::new(&data.borrow().name, &"name".to_string(), TmpType::new(&"string".to_string()));
        data.borrow_mut().push_variable(id);
        data.borrow_mut().push_variable(age);
        data.borrow_mut().push_variable(name);
        unit.push_data(data);

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
