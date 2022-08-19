use super::name::Name;
use super::types::Type;
use super::token::{ TokenKind, Tokenizer };
use super::structure::{ SysDCUnit, SysDCData, SysDCModule, SysDCFunction, SysDCAnnotation, SysDCSpawn, SysDCSpawnChild };

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
    pub fn parse(&mut self, namespace: Name) -> SysDCUnit {
        let mut data = vec!();
        let mut modules = vec!();
        while self.tokenizer.has_token() {
            match (self.parse_data(&namespace), self.parse_module(&namespace)) {
                (None, None) => panic!("[ERROR] Data/Module not found, but tokens remain"),
                (d, m) => {
                    if d.is_some() { data.push(d.unwrap()); }
                    if m.is_some() { modules.push(m.unwrap()); }
                }
            }
        }
        SysDCUnit::new(namespace, data, modules)
    }

    /**
     * <data> ::= data <id> \{ <id_type_mapping_list, delimiter=,> \}
     */
    fn parse_data(&mut self, namespace: &Name) -> Option<SysDCData> {
        // data
        self.tokenizer.expect(TokenKind::Data)?;

        // <id>
        let name = Name::from(namespace, self.tokenizer.request(TokenKind::Identifier).get_id());

        // \{ <id_type_mapping_list, delimiter=,> \}
        self.tokenizer.request(TokenKind::BracketBegin);
        let member = parse_list!(self.parse_id_type_mapping(&name), TokenKind::Separater);
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
        let name = Name::from(namespace, self.tokenizer.request(TokenKind::Identifier).get_id());

        // \{ <function_list, delimiter=None> \}
        self.tokenizer.request(TokenKind::BracketBegin);
        let functions = parse_list!(self.parse_function(&name));
        self.tokenizer.request(TokenKind::BracketEnd);

        Some(SysDCModule::new(name, functions))
    }

    /**
     * <function> ::= <id> <id_type_mapping_list, delimiter=,> -> <id> \{ <function_body> \}
     */
    fn parse_function(&mut self, namespace: &Name) -> Option<SysDCFunction> {
        // <id>
        let name_token = self.tokenizer.expect(TokenKind::Identifier)?;
        let name = Name::from(namespace, name_token.get_id());

        // <id_type_mapping_list, delimiter=,>
        self.tokenizer.request(TokenKind::ParenthesisBegin);
        let args = parse_list!(self.parse_id_type_mapping(&name), TokenKind::Separater);
        self.tokenizer.request(TokenKind::ParenthesisEnd);

        // -> <id>
        self.tokenizer.request(TokenKind::Allow);
        let return_type = Type::from(self.tokenizer.request(TokenKind::Identifier).get_id());   // TODO: Checker

        // \{ <function_body> \}
        self.tokenizer.request(TokenKind::BracketBegin);
        let (return_name, spawns) = self.parse_function_body(&name);
        self.tokenizer.request(TokenKind::BracketEnd);

        Some(SysDCFunction::new(name, args, (return_name, return_type), spawns))
    }

    /**
     * <function_body> = <annotation_list, delimiter=''>
     */
    fn parse_function_body(&mut self, namespace: &Name) -> (Name, Vec<SysDCSpawn>) {
        let mut returns: Option<Name> = None;
        let mut spawns = vec!();
        while let Some(annotation) = self.parse_annotation(namespace) {
            match annotation {
                SysDCAnnotation::Return(ret) => {
                    if returns.is_some() {
                        panic!("[ERROR] Annotation \"return\" is multiple defined")
                    }
                    returns = Some(ret)
                }
                SysDCAnnotation::Spawn(spawn) => spawns.push(spawn),
            }
        }
        if returns.is_none() {
            panic!("[ERROR] Annotation \"return\" is not defined");
        }
        (returns.unwrap(), spawns)
    }

    /**
     * <annotation> = @ ( <annotation_spawn> | <annotation_return> )
     */
    fn parse_annotation(&mut self, namespace: &Name) -> Option<SysDCAnnotation> {
        // @
        self.tokenizer.expect(TokenKind::AtMark)?;

        // ( <annotation_return> | <annotation_spawn> )
        if let Some(annotation) = self.parse_annotation_return(namespace) {
            return Some(annotation);
        }
        if let Some(annotation) = self.parse_annotation_spawn(namespace) {
            return Some(annotation);
        }
        None
    }

    /**
     * <annotation_return> ::= return <id>
     */
    fn parse_annotation_return(&mut self, namespace: &Name) -> Option<SysDCAnnotation> {
        self.tokenizer.expect(TokenKind::Return)?;
        let returns = self.tokenizer.request(TokenKind::Identifier).get_id();
        Some(SysDCAnnotation::new_return(Name::from(namespace, returns)))   
    }

    /**
     * <annotation_spawn> ::= spawn <id_type_mapping> ( \{ { <annotation_spawn_detail> } \} )
     */
    fn parse_annotation_spawn(&mut self, namespace: &Name) -> Option<SysDCAnnotation> {
        // spawn
        self.tokenizer.expect(TokenKind::Spawn)?;

        // <id_type_mapping>
        let spawn_result = self.parse_id_type_mapping(namespace);
        if spawn_result.is_none() {
            panic!("[ERROR] Missing to specify the result of spawn");
        }

        // ( \{ { <annotation_spawn_detail > } \} )
        let mut details = vec!();
        if self.tokenizer.expect(TokenKind::BracketBegin).is_some() {
            let mut namespace = namespace.clone();
            loop {
                match self.parse_annotation_spawn_detail(&namespace) {
                    Some(new_details) => {
                        let for_cmp = new_details[0].clone();
                        details.extend(new_details);
                        if let SysDCSpawnChild::Return{..} = for_cmp {
                            break;
                        }
                    }
                    None => panic!("[ERROR] Spawn hasn't return")
                }
                namespace = Name::from(&namespace, "_".to_string());
            }
            self.tokenizer.request(TokenKind::BracketEnd);
        }

        Some(SysDCAnnotation::new_spawn(spawn_result.unwrap(), details))
    }

    /** 
     * <annotation_spawn_detail> ::= (
     *      let <id> = <id_chain> \( <id_chain_list, delimiter=','> \) ; |
     *      use <id_chain> ; |
     *      return <id> ;
     * )
     */
    fn parse_annotation_spawn_detail(&mut self, namespace: &Name) -> Option<Vec<SysDCSpawnChild>> {
        // let
        if self.tokenizer.expect(TokenKind::Let).is_some() {
            // <id>
            let let_to = Name::from(namespace, self.tokenizer.request(TokenKind::Identifier).get_id());

            // =
            self.tokenizer.request(TokenKind::Equal);

            // <id_chain> 
            let func = match self.parse_id_chain(namespace) {
                Some((func, _)) => func.name,
                None => panic!("[ERROR] Function Name is requested, but not found")
            };
        
            // \( <id_chain_list, delimiter=',') \)
            self.tokenizer.request(TokenKind::ParenthesisBegin);
            let args = parse_list!(self.parse_id_chain(namespace), TokenKind::Separater);
            self.tokenizer.request(TokenKind::ParenthesisEnd);

            // ;
            self.tokenizer.request(TokenKind::Semicolon);

            return Some(vec!(SysDCSpawnChild::new_let_to(let_to, Type::from(func), args)));
        }

        // use
        if self.tokenizer.expect(TokenKind::Use).is_some() {
            let var_list = parse_list!(self.parse_id_chain(namespace), TokenKind::Separater)
                .into_iter()
                .map(|(name, _)| SysDCSpawnChild::new_use(name, Type::new_unsovled_nohint()))
                .collect();
            self.tokenizer.request(TokenKind::Semicolon);
            return Some(var_list);
        }

        // return
        if self.tokenizer.expect(TokenKind::Return).is_some() {
            match self.parse_id_chain(namespace) {
                Some((name, _)) => {
                    self.tokenizer.request(TokenKind::Semicolon);
                    return Some(vec!(SysDCSpawnChild::new_return(name, Type::new_unsovled_nohint())));
                },
                None => panic!("[ERROR] \"return\" needs variable name")
            }
        }

        None
    }

    /**
     * <id_chain> ::= <id_list, delimiter=.>
     */
    fn parse_id_chain(&mut self, namespace: &Name) -> Option<(Name, Type)> {
        // <id_list, delimiter=,>
        let name_elems = parse_list!(self.tokenizer.expect(TokenKind::Identifier), TokenKind::Accessor);
        let var = name_elems.iter().map(|x| x.get_id()).collect::<Vec<String>>().join(".");
        match var.len() {
            0 => None,
            _ => Some((Name::from(namespace, var), Type::new_unsovled_nohint()))
        }
    }

    /**
     * <id_type_mapping> ::= <id> : <id> 
     */
    fn parse_id_type_mapping(&mut self, namespace: &Name) -> Option<(Name, Type)> {
        // <id> : <id>
        let id1 = self.tokenizer.expect(TokenKind::Identifier)?.get_id();
        self.tokenizer.request(TokenKind::Mapping);
        let id2 = self.tokenizer.request(TokenKind::Identifier).get_id();
        Some((Name::from(namespace, id1), Type::from(id2)))
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use super::super::name::Name;
    use super::super::types::Type;
    use super::super::token::Tokenizer;
    use super::super::structure::{ SysDCUnit, SysDCData, SysDCModule, SysDCFunction, SysDCSpawn, SysDCSpawnChild };
    
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
            SysDCData::new(Name::from(&name, "A".to_string()), vec!()),
            SysDCData::new(Name::from(&name, "B".to_string()), vec!()),
            SysDCData::new(Name::from(&name, "C".to_string()), vec!()),
            SysDCData::new(Name::from(&name, "D".to_string()), vec!()),
            SysDCData::new(Name::from(&name, "E".to_string()), vec!())
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
        let name_box = Name::from(&name, "Box".to_string());

        let member = vec!(
            (Name::from(&name_box, "x".to_string()), Type::from("i32".to_string())),
            (Name::from(&name_box, "y".to_string()), Type::from("UserDefinedData".to_string()))
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
            SysDCModule::new(Name::from(&name, "A".to_string()), vec!()),
            SysDCModule::new(Name::from(&name, "B".to_string()), vec!()),
            SysDCModule::new(Name::from(&name, "C".to_string()), vec!()),
            SysDCModule::new(Name::from(&name, "D".to_string()), vec!()),
            SysDCModule::new(Name::from(&name, "E".to_string()), vec!())
        );
        let unit = SysDCUnit::new(name, vec!(), module);

        compare_unit(program, unit);
    }

    #[test]
    fn function_only_has_return() {
        let program = "
            module BoxModule {
                new() -> Box {
                    @return box
                }
            }
        ";

        let name = generate_name_for_test();
        let name_module = Name::from(&name, "BoxModule".to_string());
        let name_func = Name::from(&name_module, "new".to_string());
        let name_func_ret = Name::from(&name_func, "box".to_string());

        let func_returns = (name_func_ret, Type::from("Box".to_string()));
        let func = SysDCFunction::new(name_func, vec!(), func_returns, vec!());
        let module = SysDCModule::new(name_module, vec!(func));

        let unit = SysDCUnit::new(name, vec!(), vec!(module));

        compare_unit(program, unit);
    }

    #[test]
    fn function_has_return_and_spawn() {
        let program = "
            module BoxModule {
                new() -> Box {
                    @return box

                    @spawn box: Box
                }
            }
        ";

        let name = generate_name_for_test();
        let name_module = Name::from(&name, "BoxModule".to_string());
        let name_func = Name::from(&name_module, "new".to_string());
        let name_func_spawn_box = Name::from(&name_func, "box".to_string());
        let name_func_ret = Name::from(&name_func, "box".to_string());

        let func_spawns = vec!(
            SysDCSpawn::new((name_func_spawn_box, Type::from("Box".to_string())), vec!())
        );
        let func_returns = (name_func_ret, Type::from("Box".to_string()));
        let func = SysDCFunction::new(name_func, vec!(), func_returns, func_spawns);
        let module = SysDCModule::new(name_module, vec!(func));

        let unit = SysDCUnit::new(name, vec!(), vec!(module));

        compare_unit(program, unit);
    }

    #[test]
    fn function_has_full() {
        let program = "
            module BoxModule {
                move(box: Box, dx: i32, dy: i32) -> Box {
                    @return movedBox

                    @spawn movedBox: Box {
                        use box.x, box.y;
                        use dx, dy;

                        let movedBox = UnknownModule.func(dx);

                        return movedBox;
                    }
                }
            }
        ";

        let name = generate_name_for_test();
        let name_module = Name::from(&name, "BoxModule".to_string());
        let name_func = Name::from(&name_module, "move".to_string());
        let name_func_arg_box = Name::from(&name_func, "box".to_string());
        let name_func_arg_dx = Name::from(&name_func, "dx".to_string());
        let name_func_arg_dy = Name::from(&name_func, "dy".to_string());
        let name_func_spawn_box = Name::from(&name_func, "movedBox".to_string());
        let name_func_spawn_use_box_x = Name::from(&name_func, "box.x".to_string());
        let name_func_spawn_use_box_y = Name::from(&name_func, "box.y".to_string());
        let name_func_spawn_use_dx = Name::from(&name_func, "_.dx".to_string());
        let name_func_spawn_use_dy = Name::from(&name_func, "_.dy".to_string());
        let name_func_spawn_let_name = Name::from(&name_func, "_._.movedBox".to_string());
        let name_func_spawn_let_arg_dx = Name::from(&name_func, "_._.dx".to_string());
        let name_func_spawn_ret = Name::from(&name_func, "_._._.movedBox".to_string());
        let name_func_ret = Name::from(&name_func, "movedBox".to_string());

        let func_args = vec!(
            (name_func_arg_box, Type::from("Box".to_string())),
            (name_func_arg_dx, Type::from("i32".to_string())),
            (name_func_arg_dy, Type::from("i32".to_string()))
        );
        let func_spawns = vec!(
            SysDCSpawn::new((name_func_spawn_box, Type::from("Box".to_string())), vec!(
                SysDCSpawnChild::new_use(name_func_spawn_use_box_x, Type::new_unsovled_nohint()),
                SysDCSpawnChild::new_use(name_func_spawn_use_box_y, Type::new_unsovled_nohint()),
                SysDCSpawnChild::new_use(name_func_spawn_use_dx, Type::new_unsovled_nohint()),
                SysDCSpawnChild::new_use(name_func_spawn_use_dy, Type::new_unsovled_nohint()),
                SysDCSpawnChild::new_let_to(name_func_spawn_let_name, Type::from("UnknownModule.func".to_string()), vec!((name_func_spawn_let_arg_dx, Type::new_unsovled_nohint()))),
                SysDCSpawnChild::new_return(name_func_spawn_ret, Type::new_unsovled_nohint())
            ))
        );
        let func_returns = (name_func_ret, Type::from("Box".to_string()));
        let func = SysDCFunction::new(name_func, func_args, func_returns, func_spawns);
        let module = SysDCModule::new(name_module, vec!(func));

        let unit = SysDCUnit::new(name, vec!(), vec!(module));

        compare_unit(program, unit);
    }

    #[test]
    #[should_panic]
    fn illegal_function_1() {
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
    fn illegal_function_2() {
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
    fn illegal_function_3() {
        let program = "
            module BoxModule {
                move() {

                }
            }
        ";
        parse(program);
    }

    #[test]
    fn full() {
        let program = "
            data Box {
                x: i32,
                y: i32
            }

            module BoxModule {
                move(box: Box, dx: i32, dy: i32) -> Box {
                    @return movedBox

                    @spawn movedBox: Box {
                        use box.x, box.y, dx, dy;

                        let movedBox = UnknownModule.func(dx);

                        return movedBox;
                    }
                }
            }
        ";

        let name = generate_name_for_test();
        let name_data = Name::from(&name, "Box".to_string());
        let name_data_x = Name::from(&name_data, "x".to_string());
        let name_data_y = Name::from(&name_data, "y".to_string());
        let name_module = Name::from(&name, "BoxModule".to_string());
        let name_func = Name::from(&name_module, "move".to_string());
        let name_func_arg_box = Name::from(&name_func, "box".to_string());
        let name_func_arg_dx = Name::from(&name_func, "dx".to_string());
        let name_func_arg_dy = Name::from(&name_func, "dy".to_string());
        let name_func_spawn_box = Name::from(&name_func, "movedBox".to_string());
        let name_func_spawn_use_box_x = Name::from(&name_func, "box.x".to_string());
        let name_func_spawn_use_box_y = Name::from(&name_func, "box.y".to_string());
        let name_func_spawn_use_dx = Name::from(&name_func, "dx".to_string());
        let name_func_spawn_use_dy = Name::from(&name_func, "dy".to_string());
        let name_func_spawn_let_name = Name::from(&name_func, "_.movedBox".to_string());
        let name_func_spawn_let_arg_dx = Name::from(&name_func, "_.dx".to_string());
        let name_func_spawn_ret = Name::from(&name_func, "_._.movedBox".to_string());
        let name_func_ret = Name::from(&name_func, "movedBox".to_string());

        let func_args = vec!(
            (name_func_arg_box, Type::from("Box".to_string())),
            (name_func_arg_dx, Type::from("i32".to_string())),
            (name_func_arg_dy, Type::from("i32".to_string()))
        );
        let func_spawns = vec!(
            SysDCSpawn::new((name_func_spawn_box, Type::from("Box".to_string())), vec!(
                SysDCSpawnChild::new_use(name_func_spawn_use_box_x, Type::new_unsovled_nohint()),
                SysDCSpawnChild::new_use(name_func_spawn_use_box_y, Type::new_unsovled_nohint()),
                SysDCSpawnChild::new_use(name_func_spawn_use_dx, Type::new_unsovled_nohint()),
                SysDCSpawnChild::new_use(name_func_spawn_use_dy, Type::new_unsovled_nohint()),
                SysDCSpawnChild::new_let_to(name_func_spawn_let_name, Type::from("UnknownModule.func".to_string()), vec!((name_func_spawn_let_arg_dx, Type::new_unsovled_nohint()))),
                SysDCSpawnChild::new_return(name_func_spawn_ret, Type::new_unsovled_nohint())
            ))
        );
        let func_returns = (name_func_ret, Type::from("Box".to_string()));
        let func = SysDCFunction::new(name_func, func_args, func_returns, func_spawns);
        let module = SysDCModule::new(name_module, vec!(func));

        let data_members = vec!(
            (name_data_x, Type::from("i32".to_string())),
            (name_data_y, Type::from("i32".to_string()))
        );
        let data = SysDCData::new(name_data, data_members);

        let unit = SysDCUnit::new(name, vec!(data), vec!(module));

        compare_unit(program, unit);
    }


    fn generate_name_for_test() -> Name {
        Name::from(&Name::new_root(), "test".to_string())
    }

    fn compare_unit(program: &str, unit: SysDCUnit) {
        assert_eq!(format!("{:?}", parse(program)), format!("{:?}", unit));
    }

    fn parse(program: &str) -> SysDCUnit {
        let program = program.to_string();
        let tokenizer = Tokenizer::new(&program);
        let mut parser = Parser::new(tokenizer);
        parser.parse(generate_name_for_test())
    }
}
