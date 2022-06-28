use std::rc::Rc;
use std::cell::RefCell;

use super::name::Name;
use super::types::TmpType;
use super::token::{ Token, TokenKind, Tokenizer };
use super::structure::{ SysDCSystem, SysDCLayer, SysDCUnit, SysDCData, SysDCVariable, SysDCModule, SysDCProcedure };

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
                continue;
            }
            if let Some(module) = self.parse_module(&unit.name) {
                unit.push_module(module);
                continue;
            }
            panic!("[ERROR] Data/Module not found, but tokens remain");
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
     * <data> ::= data <id_type_mapping_var_list, begin="{", end="}">
     */
    fn parse_data(&mut self, namespace: &Name) -> Option<Rc<RefCell<SysDCData>>> {
        if self.tokenizer.expect(TokenKind::Data).is_none() {
            return None;
        }

        let data = SysDCData::new(namespace, &self.tokenizer.request(TokenKind::Identifier).get_id());
        let var_list = self.parse_id_type_mapping_var_list(&data.borrow().name, TokenKind::BracketBegin, TokenKind::BracketEnd);
        for var in var_list {
            data.borrow_mut().push_variable(var);
        }
        Some(data)
    }

    /**
     * <module> ::= module <id> \{ <procedures> \}
     */

    fn parse_module(&mut self, namespace: &Name) -> Option<Rc<RefCell<SysDCModule>>> {
        if self.tokenizer.expect(TokenKind::Module).is_none() {
            return None;
        }

        let module = SysDCModule::new(namespace, &self.tokenizer.request(TokenKind::Identifier).get_id());
        self.tokenizer.request(TokenKind::BracketBegin);
        loop {
            let procedure = self.parse_procedure(&module.borrow().name);
            match procedure {
                Some(procedure) => module.borrow_mut().push_procedure(procedure),
                None => break
            }
        }
        self.tokenizer.request(TokenKind::BracketEnd);
        Some(module)
    }

    /**
     * <procedure> ::= <id> <id_type_mapping_var_list, begin="(", end=")"> -> <type> \{ {<annotation>} \}
     */
    fn parse_procedure(&mut self, namespace: &Name) -> Option<Rc<RefCell<SysDCProcedure>>> {
        let name_token = self.tokenizer.expect(TokenKind::Identifier);
        if name_token.is_none() {
            return None;
        }

        let procedure = SysDCProcedure::new(namespace, &name_token.unwrap().get_id());

        let args = self.parse_id_type_mapping_var_list(&procedure.borrow().name, TokenKind::ParenthesisBegin, TokenKind::ParenthesisEnd);
        for arg in args {
            procedure.borrow_mut().push_arg(arg);
        }

        self.tokenizer.request(TokenKind::Allow);
        let types = self.tokenizer.request(TokenKind::Identifier).get_id();
        procedure.borrow_mut().set_return_type(TmpType::new(&types));

        self.tokenizer.request(TokenKind::BracketBegin);
        let (uses_variables, modifies_variables) = self.parse_annotation(&procedure.borrow().name);
        for variable in uses_variables {
            procedure.borrow_mut().push_using_variable(variable);
        }
        for variable in modifies_variables {
            procedure.borrow_mut().push_modifying_variable(variable);
        }
        self.tokenizer.request(TokenKind::BracketEnd);

        Some(procedure)
    }

    /**
     * <annotation> ::= {<use> | <modify>}
     */
    fn parse_annotation(&mut self, namespace: &Name) -> (Vec<Rc<RefCell<SysDCVariable>>>, Vec<Rc<RefCell<SysDCVariable>>>) {
        let (mut uses_variables, mut modifies_variables) = (vec!(), vec!());
        loop {
            if let Some(variables) = self.parse_use(namespace) {
                uses_variables.extend(variables);
                continue;
            }
            if let Some(variables) = self.parse_modify(namespace) {
                modifies_variables.extend(variables);
                continue;
            }
            break;
        }
        (uses_variables, modifies_variables)
    }

    /**
     * <use> ::= use \= <var_list, begin="[", end="]"> ;
     */
    fn parse_use(&mut self, namespace: &Name) -> Option<Vec<Rc<RefCell<SysDCVariable>>>> {
        if self.tokenizer.expect(TokenKind::Use).is_none() {
            return None;
        }

        self.tokenizer.request(TokenKind::Equal);
        let uses_variables = self.parse_var_list(namespace, TokenKind::ListBegin, TokenKind::ListEnd);
        self.tokenizer.request(TokenKind::Semicolon);
        Some(uses_variables)
    }

    /**
     * <modify> ::= modify \= <var_list, begin="[", end="]"> ;
     */
    fn parse_modify(&mut self, namespace: &Name) -> Option<Vec<Rc<RefCell<SysDCVariable>>>> {
        if self.tokenizer.expect(TokenKind::Modify).is_none() {
            return None;
        }

        self.tokenizer.request(TokenKind::Equal);
        let modifies_variables = self.parse_var_list(namespace, TokenKind::ListBegin, TokenKind::ListEnd);
        self.tokenizer.request(TokenKind::Semicolon);
        Some(modifies_variables)
    }

    /**
     * <var_list> ::= <begin> {<var>} <end>
     */
    fn parse_var_list(&mut self, namespace: &Name, begin: TokenKind, end: TokenKind) -> Vec<Rc<RefCell<SysDCVariable>>> {
        self.tokenizer.request(begin);
        let mut var_list = vec!();
        loop {
            var_list.push(self.parse_var(namespace));
            if self.tokenizer.expect(end.clone()).is_some() {
                break;
            } else {
                self.tokenizer.request(TokenKind::Separater);
            }
        }
        var_list
    }

    /**
     * <var> ::= {<id>.} <id>
     */
    fn parse_var(&mut self, namespace: &Name) -> Rc<RefCell<SysDCVariable>> {
        let mut discovered_name_elems = vec!();
        loop {
            discovered_name_elems.push(self.tokenizer.request(TokenKind::Identifier).get_id());
            if self.tokenizer.expect(TokenKind::Accessor).is_none() {
                break;
            }
        }
        SysDCVariable::new(namespace, &discovered_name_elems.join("."), TmpType::new(&"tmp".to_string()))
        // => Connector will resolves "Name", "Type" after parse process
    }

    /**
     * <id_type_mapping_var_list> = <begin> {<id_type_mapping_var>} <end>
     */
    fn parse_id_type_mapping_var_list(&mut self, namespace: &Name, begin: TokenKind, end: TokenKind) -> Vec<Rc<RefCell<SysDCVariable>>> {
        self.tokenizer.request(begin);
        let mut var_list = vec!();
        loop {
            var_list.push(self.parse_id_type_mapping_var(namespace));
            if self.tokenizer.expect(end.clone()).is_some() {
                break;
            } else {
                self.tokenizer.request(TokenKind::Separater);
            }
        }
        var_list
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
    use super::{ SysDCSystem, SysDCLayer, SysDCUnit, SysDCData, SysDCVariable, SysDCModule, SysDCProcedure };

    #[test]
    fn parse_simple_unit() {
        compare_unit("layer 0;", generate_test_unit(0));
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

        compare_unit(program, unit);
    }

    #[test]
    fn parse_module() {
        let program = "
            layer 0;
            module UserModule {
                greet(name: string, message: string) -> none {
                    use = [name, message];
                    modify = [name];
                }
            }
        ";

        let mut unit = generate_test_unit(0);
        let module = SysDCModule::new(&unit.name, &"UserModule".to_string());
        let procedure = SysDCProcedure::new(&module.borrow().name, &"greet".to_string());
        let arg_name = SysDCVariable::new(&procedure.borrow().name, &"name".to_string(), TmpType::new(&"string".to_string()));
        let arg_message = SysDCVariable::new(&procedure.borrow().name, &"message".to_string(), TmpType::new(&"string".to_string()));
        let use_name = SysDCVariable::new(&procedure.borrow().name, &"name".to_string(), TmpType::new(&"tmp".to_string()));
        let use_message = SysDCVariable::new(&procedure.borrow().name, &"message".to_string(), TmpType::new(&"tmp".to_string()));
        let modify_name = SysDCVariable::new(&procedure.borrow().name, &"name".to_string(), TmpType::new(&"tmp".to_string()));
        procedure.borrow_mut().set_return_type(TmpType::new(&"none".to_string()));
        procedure.borrow_mut().push_arg(arg_name);
        procedure.borrow_mut().push_arg(arg_message);
        procedure.borrow_mut().push_using_variable(use_name);
        procedure.borrow_mut().push_using_variable(use_message);
        procedure.borrow_mut().push_modifying_variable(modify_name);
        module.borrow_mut().push_procedure(procedure);
        unit.push_module(module);

        compare_unit(program, unit);
    }

    #[test]
    #[should_panic]
    fn parse_syntax_error_1() {
        parse("aaa");
    }

    #[test]
    #[should_panic]
    fn parse_syntax_error_2() {
        parse("
            layer 0;
            data User {
                id: int32,
                age,
                name: string
            }
        ");
    }

    #[test]
    #[should_panic]
    fn parse_syntax_error_3() {
        parse("
            layer 0;
            module {
                greet() {
                }
            }
        ");
    }

    fn compare_unit(program: &str, unit: SysDCUnit) {
        assert_eq!(format!("{:?}", parse(program)), format!("{:?}", unit));
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
