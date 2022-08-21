use std::error::Error;

use super::name::Name;
use super::types::Type;
use super::token::{ TokenKind, Tokenizer };
use super::error::{ CompileError, CompileErrorKind };
use super::structure::unchecked::{ SysDCUnit, SysDCData, SysDCModule, SysDCFunction, SysDCSpawn, SysDCSpawnChild };

// 複数要素を一気にパースするためのマクロ
// - 返り値: Vec<T>
// - 第一引数: Option<T>を返す関数呼び出し
// - 第二引数: TokenKindで表されるデリミタ(省略可)
macro_rules! parse_list {
    ($self:ident$(.$generator:ident)*($args:expr)) => {{
        let mut var_list = vec!();
        while let Some(elem) = $self$(.$generator)*($args)? {
            var_list.push(elem);
        }
        var_list
    }};

    ($self:ident$(.$generator:ident)*($args:expr), $delimiter:expr) => {{
        let mut var_list = vec!();
        while let Some(elem) = $self$(.$generator)*($args)? {
            var_list.push(elem);
            if $self.tokenizer.expect($delimiter)?.is_none() {
                break;
            }
        }
        var_list
    }};
}

enum Annotation {
    Return(Name),
    Spawn(SysDCSpawn)
}

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>
}

impl<'a> Parser<'a> {
    pub fn parse(tokenizer: Tokenizer<'a>, namespace: Name) -> Result<SysDCUnit, Box<dyn Error>> {
        let mut parser = Parser { tokenizer };
        parser.parse_root(namespace)
    }

    /**
     * <root> ::= { <sentence> }
     * <sentence> ::= { <data> | <module> }
     */
    fn parse_root(&mut self, namespace: Name) -> Result<SysDCUnit, Box<dyn Error>> {
        let mut data = vec!();
        let mut modules = vec!();
        while self.tokenizer.has_token() {
            match (self.parse_data(&namespace)?, self.parse_module(&namespace)?) {
                (None, None) => return CompileError::new(CompileErrorKind::UnexpectedEOF),
                (d, m) => {
                    if d.is_some() { data.push(d.unwrap()); }
                    if m.is_some() { modules.push(m.unwrap()); }
                }
            }
        }
        Ok(SysDCUnit::new(namespace, data, modules))
    }

    /**
     * <data> ::= data <id> \{ <id_type_mapping_list, delimiter=,> \}
     */
    fn parse_data(&mut self, namespace: &Name) -> Result<Option<SysDCData>, Box<dyn Error>> {
        // data
        if self.tokenizer.expect(TokenKind::Data)?.is_none() {
            return Ok(None)
        }

        // <id>
        let name = Name::from(namespace, self.tokenizer.request(TokenKind::Identifier)?.get_id()?);

        // \{ <id_type_mapping_list, delimiter=,> \}
        self.tokenizer.request(TokenKind::BracketBegin)?;
        let member = parse_list!(self.parse_id_type_mapping(&name), TokenKind::Separater);
        self.tokenizer.request(TokenKind::BracketEnd)?;

        Ok(Some(SysDCData::new(name, member)))
    }

    /**
     * <module> ::= module <id> \{ <function_list, delimiter=None> \}
     */
    fn parse_module(&mut self, namespace: &Name) -> Result<Option<SysDCModule>, Box<dyn Error>> {
        // module
        if self.tokenizer.expect(TokenKind::Module)?.is_none() {
            return Ok(None);
        }

        // <id>
        let name = Name::from(namespace, self.tokenizer.request(TokenKind::Identifier)?.get_id()?);

        // \{ <function_list, delimiter=None> \}
        self.tokenizer.request(TokenKind::BracketBegin)?;
        let functions = parse_list!(self.parse_function(&name));
        self.tokenizer.request(TokenKind::BracketEnd)?;

        Ok(Some(SysDCModule::new(name, functions)))
    }

    /**
     * <function> ::= <id> <id_type_mapping_list, delimiter=,> -> <id> \{ <function_body> \}
     */
    fn parse_function(&mut self, namespace: &Name) -> Result<Option<SysDCFunction>, Box<dyn Error>> {
        // <id>
        let name = if let Some(name_token) = self.tokenizer.expect(TokenKind::Identifier)? {
            Name::from(namespace, name_token.get_id()?)
        } else {
            return Ok(None);
        };

        // <id_type_mapping_list, delimiter=,>
        self.tokenizer.request(TokenKind::ParenthesisBegin)?;
        let args = parse_list!(self.parse_id_type_mapping(&name), TokenKind::Separater);
        self.tokenizer.request(TokenKind::ParenthesisEnd)?;

        // -> <id>
        self.tokenizer.request(TokenKind::Allow)?;
        let return_type = Type::from(self.tokenizer.request(TokenKind::Identifier)?.get_id()?);   // TODO: Checker

        // \{ <function_body> \}
        self.tokenizer.request(TokenKind::BracketBegin)?;
        let (return_name, spawns) = self.parse_function_body(&name)?;
        self.tokenizer.request(TokenKind::BracketEnd)?;

        Ok(Some(SysDCFunction::new(name, args, (return_name, return_type), spawns)))
    }

    /**
     * <function_body> = <annotation_list, delimiter=''>
     */
    fn parse_function_body(&mut self, namespace: &Name) -> Result<(Name, Vec<SysDCSpawn>), Box<dyn Error>> {
        let mut returns: Option<Name> = None;
        let mut spawns = vec!();
        while let Some(annotation) = self.parse_annotation(namespace)? {
            match annotation {
                Annotation::Return(ret) => {
                    if returns.is_some() {
                        return CompileError::new(CompileErrorKind::ReturnExistsMultiple);
                    }
                    returns = Some(ret)
                }
                Annotation::Spawn(spawn) => spawns.push(spawn),
            }
        }
        if returns.is_none() {
            return CompileError::new(CompileErrorKind::ReturnNotExists);
        }
        Ok((returns.unwrap(), spawns))
    }

    /**
     * <annotation> = @ ( <annotation_spawn> | <annotation_return> )
     */
    fn parse_annotation(&mut self, namespace: &Name) -> Result<Option<Annotation>, Box<dyn Error>> {
        // @
        if self.tokenizer.expect(TokenKind::AtMark)?.is_none() {
            return Ok(None);
        }

        // ( <annotation_return> | <annotation_spawn> )
        if let Some(annotation) = self.parse_annotation_return(namespace)? {
            return Ok(Some(annotation));
        }
        if let Some(annotation) = self.parse_annotation_spawn(namespace)? {
            return Ok(Some(annotation));
        }
        Ok(None)
    }

    /**
     * <annotation_return> ::= return <id>
     */
    fn parse_annotation_return(&mut self, namespace: &Name) -> Result<Option<Annotation>, Box<dyn Error>> {
        if self.tokenizer.expect(TokenKind::Return)?.is_none() {
            return Ok(None);
        }
        let returns = self.tokenizer.request(TokenKind::Identifier)?.get_id()?;
        Ok(Some(Annotation::Return(Name::from(namespace, returns))))
    }

    /**
     * <annotation_spawn> ::= spawn <id_type_mapping> ( \{ { <annotation_spawn_detail> } \} )
     */
    fn parse_annotation_spawn(&mut self, namespace: &Name) -> Result<Option<Annotation>, Box<dyn Error>> {
        // spawn
        if self.tokenizer.expect(TokenKind::Spawn)?.is_none() {
            return Ok(None);
        }

        // <id_type_mapping>
        let spawn_result = self.parse_id_type_mapping(namespace)?;
        if spawn_result.is_none() {
            return CompileError::new(CompileErrorKind::ResultOfSpawnNotSpecified);
        }

        // ( \{ { <annotation_spawn_detail > } \} )
        let mut details = vec!();
        if self.tokenizer.expect(TokenKind::BracketBegin)?.is_some() {
            let mut namespace = namespace.clone();
            while let Some(new_details) = self.parse_annotation_spawn_detail(&namespace)? {
                let for_cmp = new_details[0].clone();
                details.extend(new_details);
                if let SysDCSpawnChild::Return{..} = for_cmp {
                    break;
                }
                namespace = Name::from(&namespace, "_".to_string());
            }
            self.tokenizer.request(TokenKind::BracketEnd)?;
        }

        Ok(Some(Annotation::Spawn(SysDCSpawn::new(spawn_result.unwrap(), details))))
    }

    /** 
     * <annotation_spawn_detail> ::= (
     *      let <id> = <id_chain> \( <id_chain_list, delimiter=','> \) ; |
     *      use <id_chain> ; |
     *      return <id> ;
     * )
     */
    fn parse_annotation_spawn_detail(&mut self, namespace: &Name) -> Result<Option<Vec<SysDCSpawnChild>>, Box<dyn Error>> {
        // let
        if self.tokenizer.expect(TokenKind::Let)?.is_some() {
            // <id>
            let let_to = Name::from(namespace, self.tokenizer.request(TokenKind::Identifier)?.get_id()?);

            // =
            self.tokenizer.request(TokenKind::Equal)?;

            // <id_chain> 
            let func = match self.parse_id_chain(namespace)? {
                Some((func, _)) => func.name,
                None => return CompileError::new(CompileErrorKind::FunctionNameNotFound)
            };
        
            // \( <id_chain_list, delimiter=',') \)
            self.tokenizer.request(TokenKind::ParenthesisBegin)?;
            let args = parse_list!(self.parse_id_chain(namespace), TokenKind::Separater);
            self.tokenizer.request(TokenKind::ParenthesisEnd)?;

            // ;
            self.tokenizer.request(TokenKind::Semicolon)?;

            return Ok(Some(vec!(SysDCSpawnChild::new_let_to(let_to, (Name::new_root(), Type::from(func)), args))));
        }

        // use
        if self.tokenizer.expect(TokenKind::Use)?.is_some() {
            let var_list = parse_list!(self.parse_id_chain(namespace), TokenKind::Separater)
                .into_iter()
                .map(|(name, _)| SysDCSpawnChild::new_use(name, Type::new_unsovled_nohint()))
                .collect();
            self.tokenizer.request(TokenKind::Semicolon)?;
            return Ok(Some(var_list));
        }

        // return
        if self.tokenizer.expect(TokenKind::Return)?.is_some() {
            match self.parse_id_chain(namespace)? {
                Some((name, _)) => {
                    self.tokenizer.request(TokenKind::Semicolon)?;
                    return Ok(Some(vec!(SysDCSpawnChild::new_return(name, Type::new_unsovled_nohint()))));
                },
                None => return CompileError::new(CompileErrorKind::ResultOfSpawnNotSpecified)
            }
        }

        Ok(None)
    }

    /**
     * <id_chain> ::= <id_list, delimiter=.>
     */
    fn parse_id_chain(&mut self, namespace: &Name) -> Result<Option<(Name, Type)>, Box<dyn Error>> {
        // <id_list, delimiter=,>
        let name_elems = parse_list!(self.tokenizer.expect(TokenKind::Identifier), TokenKind::Accessor);
        let var = name_elems.iter().map(|x| x.get_id().unwrap()).collect::<Vec<String>>().join(".");
        match var.len() {
            0 => Ok(None),
            _ => Ok(Some((Name::from(namespace, var), Type::new_unsovled_nohint())))
        }
    }

    /**
     * <id_type_mapping> ::= <id> : <id> 
     */
    fn parse_id_type_mapping(&mut self, namespace: &Name) -> Result<Option<(Name, Type)>, Box<dyn Error>> {
        // <id> : <id>
        let id1 = if let Some(id1_token) = self.tokenizer.expect(TokenKind::Identifier)? {
            id1_token.get_id()?
        } else {
            return Ok(None);
        };
        self.tokenizer.request(TokenKind::Mapping)?;
        let id2 = self.tokenizer.request(TokenKind::Identifier)?.get_id()?;
        Ok(Some((Name::from(namespace, id1), Type::from(id2))))
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use super::super::name::Name;
    use super::super::types::Type;
    use super::super::token::Tokenizer;
    use super::super::structure::unchecked::{ SysDCUnit, SysDCData, SysDCModule, SysDCFunction, SysDCSpawn, SysDCSpawnChild };
    
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
                SysDCSpawnChild::new_let_to(
                    name_func_spawn_let_name,
                    (Name::new_root(), Type::from("UnknownModule.func".to_string())),
                    vec!((name_func_spawn_let_arg_dx, Type::new_unsovled_nohint()))
                ),
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
                SysDCSpawnChild::new_let_to(
                    name_func_spawn_let_name,
                    (Name::new_root(), Type::from("UnknownModule.func".to_string())),
                    vec!((name_func_spawn_let_arg_dx, Type::new_unsovled_nohint()))
                ),
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
        Parser::parse(tokenizer, generate_name_for_test()).unwrap()
    }
}
