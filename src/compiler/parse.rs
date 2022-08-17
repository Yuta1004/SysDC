use super::name::Name;
use super::types::SysDCType;
use super::token::{ TokenKind, Tokenizer };
use super::structure::{ SysDCSystem, SysDCUnit, SysDCData, SysDCModule, SysDCFunction, SysDCSpawn, SysDCSpawnChild };

// 複数要素を一気にパースするためのマクロ
// - 返り値: Vec<T>
// - 第一引数: Option<T>を返す関数呼び出し
// - 第二引数: TokenKindで表されるデリミタ(省略可)
macro_rules! parse_list {
    ($self:ident$(.$generator:ident)*($args:expr)) => {{
        let mut var_list = vec!();
        while let Some(elem) = $self$(.$generator)*($args) {
            var_list.push(elem);
        }
        var_list
    }};

    ($self:ident$(.$generator:ident)*($args:expr), $delimiter:expr) => {{
        let mut var_list = vec!();
        while let Some(elem) = $self$(.$generator)*($args) {
            var_list.push(elem);
            if $self.tokenizer.expect($delimiter).is_none() {
                break;
            }
        }
        var_list
    }};
}

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: Tokenizer<'a>) -> Parser<'a> {
        Parser { tokenizer }
    }

    /**
     * <root> ::= { <sentence> }
     * <sentence> ::= { <data> | <module> }
     */
    pub fn parse(&mut self, namespace: &Name) -> SysDCUnit {
        let mut data = vec!();
        let mut modules = vec!();
        while self.tokenizer.has_token() {
            match (self.parse_data(namespace), self.parse_module(namespace)) {
                (None, None) => panic!("[ERROR] Data/Module not found, but tokens remain"),
                (d, m) => {
                    if d.is_some() { data.push(d.unwrap()); }
                    if m.is_some() { modules.push(m.unwrap()); }
                }
            }
        }
        SysDCUnit::new(namespace.clone(), data, modules)
    }

    /**
     * <data> ::= data <id> \{ <id_type_mapping_var_list, delimiter=,> \}
     */
    fn parse_data(&mut self, namespace: &Name) -> Option<SysDCData> {
        // data
        self.tokenizer.expect(TokenKind::Data)?;

        // <id>
        let name = Name::new(namespace, self.tokenizer.request(TokenKind::Identifier).get_id());

        // \{ <id_type_mapping_var_list, delimiter=,> \}
        self.tokenizer.request(TokenKind::BracketBegin);
        let member = parse_list!(self.parse_id_type_mapping_var(&name), TokenKind::Separater);
        self.tokenizer.request(TokenKind::BracketEnd);

        Some(SysDCData::new(name, member))
    }

    /**
     * <module> ::= module <id> \{ <function_list, delimiter=None> \}
     */
    fn parse_module(&mut self, namespace: &Name) -> Option<SysDCModule> {
        // module
        self.tokenizer.expect(TokenKind::Module)?;

        // <id>
        let name = Name::new(namespace, self.tokenizer.request(TokenKind::Identifier).get_id());

        // \{ <function_list, delimiter=None> \}
        self.tokenizer.request(TokenKind::BracketBegin);
        let functions = parse_list!(self.parse_function(&name));
        self.tokenizer.request(TokenKind::BracketEnd);

        Some(SysDCModule::new(name, functions))
    }

    /**
     * <function> ::= <id> <id_type_mapping_var_list, delimiter=,> -> <id> ( \{ \} )
     */
    fn parse_function(&mut self, namespace: &Name) -> Option<SysDCFunction> {
        // <id>
        let name_token = self.tokenizer.expect(TokenKind::Identifier)?;
        let name = Name::new(namespace, name_token.get_id());

        // <id_type_mapping_var_list, delimiter=,>
        self.tokenizer.request(TokenKind::ParenthesisBegin);
        let args = parse_list!(self.parse_id_type_mapping_var(&name), TokenKind::Separater);
        self.tokenizer.request(TokenKind::ParenthesisEnd);

        // -> <id>
        self.tokenizer.request(TokenKind::Allow);
        let return_type = SysDCType::from(&name, self.tokenizer.request(TokenKind::Identifier).get_id());   // TODO: Checker

        self.tokenizer.request(TokenKind::BracketBegin);
        self.tokenizer.request(TokenKind::BracketEnd);

        Some(SysDCFunction::new(name, args, (Name::new_root(), return_type), vec!()))
    }

    /**
     * <var> ::= <id_list, delimiter=.>
     */
    fn parse_var(&mut self, namespace: &Name) -> Option<(Name, SysDCType)> {
        // <id_list, delimiter=,>
        let name_elems = parse_list!(self.tokenizer.expect(TokenKind::Identifier), TokenKind::Accessor);
        let var = name_elems.iter().map(|x| x.get_id()).collect::<Vec<String>>().join(".");
        match var.len() {
            0 => None,
            _ => Some((Name::new(namespace, var.clone()), SysDCType::from(namespace, var)))
        }
    }

    /**
     * <id_type_mapping_var> ::= <id> : <id> 
     */
    fn parse_id_type_mapping_var(&mut self, namespace: &Name) -> Option<(Name, SysDCType)> {
        // <id> : <id>
        let id1 = self.tokenizer.expect(TokenKind::Identifier)?.get_id();
        self.tokenizer.request(TokenKind::Mapping);
        let id2 = self.tokenizer.request(TokenKind::Identifier).get_id();
        Some((Name::new(namespace, id1), SysDCType::from(namespace, id2)))
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use super::super::name::Name;
    use super::super::types::SysDCType;
    use super::super::token::Tokenizer;
    use super::super::structure::{ SysDCUnit, SysDCData, SysDCModule, SysDCFunction };
    
    #[test]
    fn data_empty_ok() {
        let program = "
            data A {}
            data B{}
            data C{   

            }
            data D
            {}
            data
            E
            {

            }
        ";

        let name = generate_name_for_test();

        let data = vec!(
            SysDCData::new(Name::new(&name, "A".to_string()), vec!()),
            SysDCData::new(Name::new(&name, "B".to_string()), vec!()),
            SysDCData::new(Name::new(&name, "C".to_string()), vec!()),
            SysDCData::new(Name::new(&name, "D".to_string()), vec!()),
            SysDCData::new(Name::new(&name, "E".to_string()), vec!())
        );
        let unit = SysDCUnit::new(name, data, vec!());

        compare_unit(program, unit);
    }

    #[test]
    fn data_has_member_ok() {
        let program = "
            data Box {
                x: i32,
                y: UserDefinedData,
            }
        ";

        let name = generate_name_for_test();
        let name_box = Name::new(&name, "Box".to_string());

        let member = vec!(
            (Name::new(&name_box, "x".to_string()), SysDCType::Int32),
            (Name::new(&name_box, "y".to_string()), SysDCType::from(&name_box, "UserDefinedData".to_string()))
        );
        let data = SysDCData::new(name_box, member);
        let unit = SysDCUnit::new(name, vec!(data), vec!());

        compare_unit(program, unit);
    }

    #[test]
    #[should_panic]
    fn data_has_illegal_member_def_1() {
        let program = "
            data Box {
                x: i32
                y: i32
            }
        ";
        parse(program);
    }

    #[test]
    #[should_panic]
    fn data_has_illegal_member_def_2() {
        let program = "
            data Box {
                x: i32,
                y:
            }
        ";
        parse(program);
    }

    #[test]
    #[should_panic]
    fn data_has_illegal_member_def_3() {
        let program = "
            data Box
                x: i32,
                y: i32
        ";
        parse(program);
    }

    #[test]
    fn module_empty_ok() {
        let program = "
            module A {}
            module B{}
            module C{   

            }
            module D
            {}
            module
            E
            {

            }
        ";

        let name = generate_name_for_test();

        let module = vec!(
            SysDCModule::new(Name::new(&name, "A".to_string()), vec!()),
            SysDCModule::new(Name::new(&name, "B".to_string()), vec!()),
            SysDCModule::new(Name::new(&name, "C".to_string()), vec!()),
            SysDCModule::new(Name::new(&name, "D".to_string()), vec!()),
            SysDCModule::new(Name::new(&name, "E".to_string()), vec!())
        );
        let unit = SysDCUnit::new(name, vec!(), module);

        compare_unit(program, unit);
    }

    #[test]
    #[should_panic]
    fn module_has_illegal_function_1() {
        let program = "
            module BoxModule {
                move() -> {

                }
            }
        ";
        parse(program);
    }

    #[test]
    #[should_panic]
    fn module_has_illegal_function_2() {
        let program = "
            module BoxModule {
                move(box: Box, dx: i32, dy: ) -> i32 {

                }
            }
        ";
        parse(program);
    }

    #[test]
    #[should_panic]
    fn module_has_illegal_function_3() {
        let program = "
            module BoxModule {
                move() {

                }
            }
        ";
        parse(program);
    }

    #[test]
    fn data_and_module_simple_ok() {
        let program = "
            data Box {
                x: i32,
                y: i32
            }

            module BoxModule {
                move(box: Box, dx: i32, dy: i32) -> Box {
                }
            }
        ";

        let name = generate_name_for_test();
        let name_data = Name::new(&name, "Box".to_string());
        let name_data_x = Name::new(&name_data, "x".to_string());
        let name_data_y = Name::new(&name_data, "y".to_string());
        let name_module = Name::new(&name, "BoxModule".to_string());
        let name_func = Name::new(&name_module, "move".to_string());
        let name_func_arg_box = Name::new(&name_func, "box".to_string());
        let name_func_arg_dx = Name::new(&name_func, "dx".to_string());
        let name_func_arg_dy = Name::new(&name_func, "dy".to_string());

        let func_args = vec!(
            (name_func_arg_box, SysDCType::from(&name_func, "Box".to_string())),
            (name_func_arg_dx, SysDCType::Int32),
            (name_func_arg_dy, SysDCType::Int32)
        );
        let func_returns = (Name::new_root(), SysDCType::from(&name_func, "Box".to_string()));
        let func = SysDCFunction::new(name_func, func_args, func_returns, vec!());

        let module = SysDCModule::new(name_module, vec!(func));

        let data_members = vec!(
            (name_data_x, SysDCType::Int32),
            (name_data_y, SysDCType::Int32)
        );
        let data = SysDCData::new(name_data, data_members);

        let unit = SysDCUnit::new(name, vec!(data), vec!(module));

        compare_unit(program, unit);
    }


    fn generate_name_for_test() -> Name {
        Name::new(&Name::new_root(), "test".to_string())
    }

    fn compare_unit(program: &str, unit: SysDCUnit) {
        assert_eq!(format!("{:?}", parse(program)), format!("{:?}", unit));
    }

    fn parse(program: &str) -> SysDCUnit {
        let program = program.to_string();
        let tokenizer = Tokenizer::new(&program);
        let mut parser = Parser::new(tokenizer);
        parser.parse(&generate_name_for_test())
    }
}

// #[cfg(test)]
// mod test {
//     use super::Name;
//     use super::SysDCType;
//     use super::{ Tokenizer, Parser };
//     use super::{ SysDCUnit, SysDCData, SysDCVariable, SysDCModule, SysDCFunction, SysDCLink };

//     #[test]
//     fn parse_simple_unit() {
//         let program = "
//             layer 0;
//             ref printer Printer;
//         ";
//         compare_unit(program, generate_test_unit(0));
//     }

//     #[test]
//     fn parse_data() {
//         let program = "
//             layer 0;
//             data User {
//                 id: i32,
//                 age: i32
//             }
//         ";

//         let mut unit = generate_test_unit(0);
//         let data = SysDCData::new(&unit.name, "User".to_string());
//         let id = SysDCVariable::new(&data.borrow().name, "id".to_string(), SysDCType::Int32);
//         let age = SysDCVariable::new(&data.borrow().name, "age".to_string(), SysDCType::Int32);
//         data.borrow_mut().push_variable(id);
//         data.borrow_mut().push_variable(age);
//         unit.push_data(data);

//         compare_unit(program, unit);
//     }

//     #[test]
//     fn parse_module_function_has_not_args() {
//         let program = "
//             layer 0;
//             module UserModule {
//                 greet() -> none {
//                     link = Printer::print()
//                 }
//             }
//         ";

//         let mut unit = generate_test_unit(0);
//         let module = SysDCModule::new(&unit.name, "UserModule".to_string());
//         let func = SysDCFunction::new(&module.borrow().name, "greet".to_string());
//         let iop_printer = SysDCLink::new_instance_of_function(&func.borrow().name, "link".to_string());
//         func.borrow_mut().set_link(iop_printer);
//         func.borrow_mut().set_return_type(SysDCType::NoneType);
//         module.borrow_mut().push_function(func);
//         unit.push_module(module);

//         compare_unit(program, unit);
//     }

//     #[test]
//     fn parse_module_func_has_args() {
//         let program = "
//             layer 0;
//             module UserModule binds User {
//                 greet(name: i32, message: i32) -> none {
//                     use = [name, message];
//                     modify = [name];
//                 }
//             }
//         ";

//         let mut unit = generate_test_unit(0);
//         let module = SysDCModule::new(&unit.name, "UserModule".to_string());
//         let func = SysDCFunction::new(&module.borrow().name, "greet".to_string());
//         let arg_name = SysDCVariable::new(&func.borrow().name, "name".to_string(), SysDCType::Int32);
//         let arg_message = SysDCVariable::new(&func.borrow().name, "message".to_string(), SysDCType::Int32);
//         let use_name = SysDCVariable::new(&func.borrow().name, "name".to_string(), SysDCType::from(&func.borrow().name, "name".to_string()));
//         let use_message = SysDCVariable::new(&func.borrow().name, "message".to_string(), SysDCType::from(&func.borrow().name, "message".to_string()));
//         let modify_name = SysDCVariable::new(&func.borrow().name, "name".to_string(), SysDCType::from(&func.borrow().name, "name".to_string()));
//         func.borrow_mut().set_return_type(SysDCType::NoneType);
//         func.borrow_mut().push_arg(arg_name);
//         func.borrow_mut().push_arg(arg_message);
//         func.borrow_mut().push_using_variable(use_name);
//         func.borrow_mut().push_using_variable(use_message);
//         func.borrow_mut().push_modifying_variable(modify_name);
//         module.borrow_mut().push_function(func);
//         unit.push_module(module);

//         compare_unit(program, unit);
//     }

//     #[test]
//     fn parse_module_with_link_chain_first() {
//         let program = "
//             layer 0;
//             module UserModule binds User as this {
//                 greet(message: i32) -> none {
//                     link = chain {
//                         branch {
//                             Printer::print(this.age),
//                             Printer::print(this.name)
//                         },
//                         chain {
//                             Printer::print(this.id),
//                             Printer::print(message)
//                         }
//                     }
//                 }
//             }
//         ";

//         let mut unit = generate_test_unit(0);
//         let module = SysDCModule::new(&unit.name, "UserModule".to_string());
//         let func = SysDCFunction::new(&module.borrow().name, "greet".to_string());
//         let arg_message = SysDCVariable::new(&func.borrow().name, "message".to_string(), SysDCType::Int32);
//         let chain_link = SysDCLink::new_chain(&func.borrow().name, "link".to_string());
//         let branch_link2 = SysDCLink::new_branch(&chain_link.borrow().name, "link".to_string());
//         let iop_age_link3 = SysDCLink::new_instance_of_function(&branch_link2.borrow().name, "link".to_string());
//         let arg_age_link3 = SysDCVariable::new(&iop_age_link3.borrow().name, "this.age".to_string(), SysDCType::from(&iop_age_link3.borrow().name, "this.age".to_string()));
//         let iop_name_link3 = SysDCLink::new_instance_of_function(&branch_link2.borrow().name, "link".to_string());
//         let arg_name_link3 = SysDCVariable::new(&iop_age_link3.borrow().name, "this.name".to_string(), SysDCType::from(&iop_age_link3.borrow().name, "this.name".to_string()));
//         let chain_link2 = SysDCLink::new_chain(&chain_link.borrow().name, "link".to_string());
//         let iop_id_link3 = SysDCLink::new_instance_of_function(&chain_link2.borrow().name, "link".to_string());
//         let arg_id_link3 = SysDCVariable::new(&iop_id_link3.borrow().name, "this.id".to_string(), SysDCType::from(&iop_id_link3.borrow().name, "this.id".to_string()));
//         let iop_message_link3 = SysDCLink::new_instance_of_function(&chain_link2.borrow().name, "link".to_string());
//         let arg_message_link3 = SysDCVariable::new(&iop_message_link3.borrow().name, "message".to_string(), SysDCType::from(&iop_message_link3.borrow().name, "message".to_string()));
//         iop_age_link3.borrow_mut().push_arg(arg_age_link3);
//         iop_name_link3.borrow_mut().push_arg(arg_name_link3);
//         branch_link2.borrow_mut().push_link(iop_age_link3);
//         branch_link2.borrow_mut().push_link(iop_name_link3);
//         iop_id_link3.borrow_mut().push_arg(arg_id_link3);
//         iop_message_link3.borrow_mut().push_arg(arg_message_link3);
//         chain_link2.borrow_mut().push_link(iop_id_link3);
//         chain_link2.borrow_mut().push_link(iop_message_link3);
//         chain_link.borrow_mut().push_link(branch_link2);
//         chain_link.borrow_mut().push_link(chain_link2);
//         func.borrow_mut().set_link(chain_link);
//         func.borrow_mut().set_return_type(SysDCType::NoneType);
//         func.borrow_mut().push_arg(arg_message);
//         module.borrow_mut().push_function(func);
//         unit.push_module(module);

//         compare_unit(program, unit);
//     }

//     #[test]
//     fn parse_module_with_link_branch_first() {
//         let program = "
//             layer 0;
//             module UserModule {
//                 greet(message: i32) -> none {
//                     link = branch {
//                         branch {
//                             Printer::print(this.age),
//                             Printer::print(this.name)
//                         },
//                         chain {
//                             Printer::print(this.id),
//                             Printer::print(message)
//                         }
//                     }
//                 }
//             }
//         ";

//         let mut unit = generate_test_unit(0);
//         let module = SysDCModule::new(&unit.name, "UserModule".to_string());
//         let func = SysDCFunction::new(&module.borrow().name, "greet".to_string());
//         let arg_message = SysDCVariable::new(&func.borrow().name, "message".to_string(), SysDCType::Int32);
//         let branch_link = SysDCLink::new_branch(&func.borrow().name, "link".to_string());
//         let branch_link2 = SysDCLink::new_branch(&branch_link.borrow().name, "link".to_string());
//         let iop_age_link3 = SysDCLink::new_instance_of_function(&branch_link2.borrow().name, "link".to_string());
//         let arg_age_link3 = SysDCVariable::new(&iop_age_link3.borrow().name, "this.age".to_string(), SysDCType::from(&iop_age_link3.borrow().name, "this.age".to_string()));
//         let iop_name_link3 = SysDCLink::new_instance_of_function(&branch_link2.borrow().name, "link".to_string());
//         let arg_name_link3 = SysDCVariable::new(&iop_age_link3.borrow().name, "this.name".to_string(), SysDCType::from(&iop_age_link3.borrow().name, "this.name".to_string()));
//         let chain_link2 = SysDCLink::new_chain(&branch_link.borrow().name, "link".to_string());
//         let iop_id_link3 = SysDCLink::new_instance_of_function(&chain_link2.borrow().name, "link".to_string());
//         let arg_id_link3 = SysDCVariable::new(&iop_id_link3.borrow().name, "this.id".to_string(), SysDCType::from(&iop_id_link3.borrow().name, "this.id".to_string()));
//         let iop_message_link3 = SysDCLink::new_instance_of_function(&chain_link2.borrow().name, "link".to_string());
//         let arg_message_link3 = SysDCVariable::new(&iop_message_link3.borrow().name, "message".to_string(), SysDCType::from(&iop_message_link3.borrow().name, "message".to_string()));
//         iop_age_link3.borrow_mut().push_arg(arg_age_link3);
//         iop_name_link3.borrow_mut().push_arg(arg_name_link3);
//         branch_link2.borrow_mut().push_link(iop_age_link3);
//         branch_link2.borrow_mut().push_link(iop_name_link3);
//         iop_id_link3.borrow_mut().push_arg(arg_id_link3);
//         iop_message_link3.borrow_mut().push_arg(arg_message_link3);
//         chain_link2.borrow_mut().push_link(iop_id_link3);
//         chain_link2.borrow_mut().push_link(iop_message_link3);
//         branch_link.borrow_mut().push_link(branch_link2);
//         branch_link.borrow_mut().push_link(chain_link2);
//         func.borrow_mut().set_link(branch_link);
//         func.borrow_mut().set_return_type(SysDCType::NoneType);
//         func.borrow_mut().push_arg(arg_message);
//         module.borrow_mut().push_function(func);
//         unit.push_module(module);

//         compare_unit(program, unit);
//     }

//     #[test]
//     #[should_panic]
//     fn parse_syntax_error_1() {
//         parse("aaa");
//     }

//     #[test]
//     #[should_panic]
//     fn parse_syntax_error_2() {
//         parse("
//             layer 0;
//             data User {
//                 id: i32,
//                 age,
//                 name: string
//             }
//         ");
//     }

//     #[test]
//     #[should_panic]
//     fn parse_syntax_error_3() {
//         parse("
//             layer 0;
//             module {
//                 greet() {
//                 }
//             }
//         ");
//     }

//     #[test]
//     #[should_panic]
//     fn parse_syntax_error_4() {
//         parse("
//             layer 0;
//             module UserModule {
//                 greet() -> none {
//                     link = chain { }
//                 }
//             }
//         ");
//     }

//     #[test]
//     #[should_panic]
//     fn parse_syntax_error_5() {
//         parse("
//             layer 0;
//             module UserModule {
//                 greet() -> noen {
//                     link = 
//                 }
//             }
//         ");
//     }

//     fn compare_unit(program: &str, unit: SysDCUnit) {
//         assert_eq!(format!("{:?}", parse(program)), format!("{:?}", unit));
//     }

//     fn generate_test_unit(layer_num: i32) -> SysDCUnit {
//         let root_namespace = Name::new_root();
//         let layer_namespace = Name::new(&root_namespace, format!("layer{}", layer_num));
//         SysDCUnit::new(&layer_namespace, "test".to_string()) 
//     }

//     fn parse(program: &str) -> SysDCUnit {
//         let program = program.to_string();
//         let tokenizer = Tokenizer::new(&program);
//         let mut parser = Parser::new(Name::new_root(), "test".to_string(), tokenizer);
//         let (_, unit) = parser.parse();
//         unit
//     }
// }
