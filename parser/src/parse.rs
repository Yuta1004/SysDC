use super::name::Name;
use super::types::Type;
use super::token::{ TokenKind, Tokenizer };
use super::error::{ PResult, PErrorKind };
use super::structure::unchecked;

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

pub struct UnitParser<'a> {
    tokenizer: Tokenizer<'a>
}

impl<'a> UnitParser<'a> {
    pub fn parse(tokenizer: Tokenizer<'a>) -> PResult<unchecked::SysDCUnit> {
        let mut parser = UnitParser { tokenizer };
        parser.parse_root(Name::new_root())
    }

    /**
     * <root> ::= { <sentence> }
     * <sentence> ::= unit <id_chain>; { <import> | <data> | <module> }
     */
    fn parse_root(&mut self, namespace: Name) -> PResult<unchecked::SysDCUnit> {
        // unit <id_chain> ;
        self.tokenizer.request(TokenKind::Unit)?;
        let namespace = match self.parse_id_chain(&namespace)? {
            Some((found_name, _)) => Name::new(&namespace, found_name.name),
            None => return PErrorKind::UnitNameNotSpecified.to_err_with_loc(self.tokenizer.get_now_ref_loc())
        };
        self.tokenizer.request(TokenKind::Semicolon)?;

        // { <import> | <data> | <module> }
        let (mut imports, mut data, mut modules) = (vec!(), vec!(), vec!());
        while self.tokenizer.exists_next() {
            match (self.parse_import()?, self.parse_data(&namespace)?, self.parse_module(&namespace)?) {
                (None, None, None) => return PErrorKind::DataOrModuleNotFound.to_err_with_loc(self.tokenizer.get_now_ref_loc()),
                (i, d, m) => {
                    if i.is_some() { imports.extend(i.unwrap()); }
                    if d.is_some() { data.push(d.unwrap()); }
                    if m.is_some() { modules.push(m.unwrap()); }
                }
            }
        }

        Ok(unchecked::SysDCUnit::new(namespace, data, modules, imports))
    }

    /**
     * <import> ::= from <id_chain> import <id_list, delimiter=','> ;
     */
    fn parse_import(&mut self) -> PResult<Option<Vec<Name>>> {
        // from
        if self.tokenizer.expect(TokenKind::From)?.is_none() {
            return Ok(None)
        }

        // <id_chain>
        let from_namespace = match self.parse_id_chain(&Name::new_root())? {
            Some((found_name, _)) => Name::new(&Name::new_root(), found_name.name),
            None => return PErrorKind::FromNamespaceNotSpecified.to_err_with_loc(self.tokenizer.get_now_ref_loc())
        };

        // import <id_list, delimiter=','> ;
        self.tokenizer.request(TokenKind::Import)?;
        let mut importes = vec!();
        for import in parse_list!(self.tokenizer.expect(TokenKind::Identifier), TokenKind::Separater) {
            importes.push(Name::new(&from_namespace, import.orig));
        }
        self.tokenizer.request(TokenKind::Semicolon)?;

        Ok(Some(importes))
    }

    /**
     * <data> ::= data <id> \{ <id_type_mapping_list, delimiter=,> \}
     */
    fn parse_data(&mut self, namespace: &Name) -> PResult<Option<unchecked::SysDCData>> {
        // data
        if self.tokenizer.expect(TokenKind::Data)?.is_none() {
            return Ok(None)
        }

        // <id>
        let name = Name::new(namespace, self.tokenizer.request(TokenKind::Identifier)?.orig);

        // \{ <id_type_mapping_list, delimiter=,> \}
        self.tokenizer.request(TokenKind::BracketBegin)?;
        let member = parse_list!(self.parse_id_type_mapping(&name), TokenKind::Separater);
        self.tokenizer.request(TokenKind::BracketEnd)?;

        Ok(Some(unchecked::SysDCData::new(name, member)))
    }

    /**
     * <module> ::= module <id> \{ <function_list, delimiter=None> \}
     */
    fn parse_module(&mut self, namespace: &Name) -> PResult<Option<unchecked::SysDCModule>> {
        // module
        if self.tokenizer.expect(TokenKind::Module)?.is_none() {
            return Ok(None);
        }

        // <id>
        let name = Name::new(namespace, self.tokenizer.request(TokenKind::Identifier)?.orig);

        // \{ <function_list, delimiter=None> \}
        self.tokenizer.request(TokenKind::BracketBegin)?;
        let functions = parse_list!(self.parse_function(&name));
        self.tokenizer.request(TokenKind::BracketEnd)?;

        Ok(Some(unchecked::SysDCModule::new(name, functions)))
    }

    /**
     * <function> ::= func <id> <id_type_mapping_list, delimiter=,> -> <id> \{ <function_body> \}
     * <procedure> ::= proc <id> <id_type_mapping_list, delimiter=,> \{ <procedure_body > \}
     */
    fn parse_function(&mut self, namespace: &Name) -> PResult<Option<unchecked::SysDCFunction>> {
        // func | proc
        let is_func = self.tokenizer.expect(TokenKind::Func)?.is_some();
        if !is_func && self.tokenizer.expect(TokenKind::Proc)?.is_none() {
            return Ok(None);
        }

        // <id>
        let name = Name::new(namespace, self.tokenizer.request(TokenKind::Identifier)?.orig);

        // <id_type_mapping_list, delimiter=,>
        self.tokenizer.request(TokenKind::ParenthesisBegin)?;
        let args = parse_list!(self.parse_id_type_mapping(&name), TokenKind::Separater);
        self.tokenizer.request(TokenKind::ParenthesisEnd)?;

        // ( -> <id> )
        let mut returns_type = None;
        if is_func {
            self.tokenizer.request(TokenKind::Allow)?;
            returns_type = Some(Type::from(self.tokenizer.request(TokenKind::Identifier)?.orig));
        }

        // \{ <function_body> | <procedure_body> \}
        self.tokenizer.request(TokenKind::BracketBegin)?;
        let (returns, annotations) = if is_func {
            let (return_name, annotations) = self.parse_function_body(&name)?;
            (Some((return_name, returns_type.unwrap())), annotations)
        } else {
            (None, self.parse_procedure_body(&name)?)
        };
        self.tokenizer.request(TokenKind::BracketEnd)?;

        Ok(Some(unchecked::SysDCFunction::new(name, args, returns, annotations)))
    }

    /**
     * <function_body> = <annotation_list, delimiter=''>
     */
    fn parse_function_body(&mut self, namespace: &Name) -> PResult<(Name, Vec<unchecked::SysDCAnnotation>)> {
        let mut returns: Option<Name> = None;
        let mut annotations = vec!();
        while let Some(annotation) = self.parse_annotation(namespace)? {
            match annotation {
                unchecked::SysDCAnnotation::Return(ret) => {
                    if returns.is_some() {
                        return PErrorKind::ReturnExistsMultiple.to_err_with_loc(self.tokenizer.get_now_ref_loc());
                    }
                    returns = Some(ret)
                },
                _ => annotations.push(annotation)
            }
        }
        if returns.is_none() {
            return PErrorKind::ReturnNotExists.to_err_with_loc(self.tokenizer.get_now_ref_loc());
        }
        Ok((returns.unwrap(), annotations))
    }

    /**
     * <procedure_body> = <annotation_list, delimiter=''>
     */
    fn parse_procedure_body(&mut self, namespace: &Name) -> PResult<Vec<unchecked::SysDCAnnotation>> {
        let mut annotations = vec!();
        while let Some(annotation) = self.parse_annotation(namespace)? {
            match annotation {
                unchecked::SysDCAnnotation::Return(_) => {
                    return PErrorKind::ReturnExistsOnProcedure.to_err_with_loc(self.tokenizer.get_now_ref_loc());
                },
                _ => annotations.push(annotation)
            }
        }
        Ok(annotations)
    }

    /**
     * <annotation> = @ ( <annotation_return> | <annotation_spawn> | <annotation_modify>)
     */
    fn parse_annotation(&mut self, namespace: &Name) -> PResult<Option<unchecked::SysDCAnnotation>> {
        // @
        if self.tokenizer.expect(TokenKind::AtMark)?.is_none() {
            return Ok(None);
        }

        // ( <annotation_return> | <annotation_spawn> | <annotation_modify> )
        if let Some(annotation) = self.parse_annotation_return(namespace)? {
            return Ok(Some(annotation));
        }
        if let Some(annotation) = self.parse_annotation_spawn(namespace)? {
            return Ok(Some(annotation));
        }
        if let Some(annotation) = self.parse_annotation_modify(namespace)? {
            return Ok(Some(annotation));
        }
        Ok(None)
    }

    /**
     * <annotation_return> ::= return <id>
     */
    fn parse_annotation_return(&mut self, namespace: &Name) -> PResult<Option<unchecked::SysDCAnnotation>> {
        if self.tokenizer.expect(TokenKind::Return)?.is_none() {
            return Ok(None);
        }
        let returns = self.tokenizer.request(TokenKind::Identifier)?.orig;
        Ok(Some(unchecked::SysDCAnnotation::new_return(Name::new(namespace, returns))))
    }

    /**
     * <annotation_spawn> ::= spawn <id_type_mapping> ( \{ { <annotation_spawn_detail> } \} )
     */
    fn parse_annotation_spawn(&mut self, namespace: &Name) -> PResult<Option<unchecked::SysDCAnnotation>> {
        // spawn
        if self.tokenizer.expect(TokenKind::Spawn)?.is_none() {
            return Ok(None);
        }

        // <id_type_mapping>
        let spawn_result = self.parse_id_type_mapping(namespace)?;
        if spawn_result.is_none() {
            return PErrorKind::ResultOfSpawnNotSpecified.to_err_with_loc(self.tokenizer.get_now_ref_loc());
        }

        // ( \{ { <annotation_spawn_detail > } \} )
        let mut details = vec!();
        if self.tokenizer.expect(TokenKind::BracketBegin)?.is_some() {
            let mut namespace = Name::new(&namespace, "_".to_string());
            while let Some(new_details) = self.parse_annotation_spawn_detail(&namespace)? {
                let for_cmp = new_details[0].clone();
                details.extend(new_details);
                if let unchecked::SysDCSpawnDetail::Return{..} = for_cmp {
                    break;
                }
                namespace = Name::new(&namespace, "_".to_string());
            }
            self.tokenizer.request(TokenKind::BracketEnd)?;
        }

        Ok(Some(unchecked::SysDCAnnotation::new_spawn(spawn_result.unwrap(), details)))
    }

    /**
     * <annotation_modify> ::= modify <id> ( \{ { use <id_list, delimiter=,> ; } \} )
     */
    fn parse_annotation_modify(&mut self, namespace: &Name) -> PResult<Option<unchecked::SysDCAnnotation>> {
        // modify
        if self.tokenizer.expect(TokenKind::Modify)?.is_none() {
            return Ok(None)
        }

        // <id>
        let name_token = self.tokenizer.request(TokenKind::Identifier)?;
        let modify_target = (Name::new(namespace, name_token.orig), Type::new_unsovled_nohint());

        // ( \{ { use <id_list, delimiter=,> ; } \} )
        let mut uses = vec!();
        if self.tokenizer.expect(TokenKind::BracketBegin)?.is_some() {
            while self.tokenizer.expect(TokenKind::Use)?.is_some() {
                for name in parse_list!(self.tokenizer.expect(TokenKind::Identifier), TokenKind::Separater) {
                    uses.push((Name::new(namespace, name.orig), Type::new_unsovled_nohint()));
                }
                self.tokenizer.request(TokenKind::Semicolon)?;
            }
            self.tokenizer.request(TokenKind::BracketEnd)?;
        }

        Ok(Some(unchecked::SysDCAnnotation::new_modify(modify_target, uses)))
    }

    /**
     * <annotation_spawn_detail> ::= (
     *      let <id> = <id_chain> \( <id_chain_list, delimiter=','> \) ; |
     *      use <id_list, delimiter=','> ; |
     *      return <id> ;
     * )
     */
    fn parse_annotation_spawn_detail(&mut self, namespace: &Name) -> PResult<Option<Vec<unchecked::SysDCSpawnDetail>>> {
        // let
        if self.tokenizer.expect(TokenKind::Let)?.is_some() {
            // <id>
            let let_to = Name::new(namespace, self.tokenizer.request(TokenKind::Identifier)?.orig);

            // =
            self.tokenizer.request(TokenKind::Equal)?;

            // <id_chain>
            let func = match self.parse_id_chain(namespace)? {
                Some((func, _)) => func.name,
                None => return PErrorKind::FunctionNameNotFound.to_err_with_loc(self.tokenizer.get_now_ref_loc())
            };

            // \( <id_chain_list, delimiter=',') \)
            self.tokenizer.request(TokenKind::ParenthesisBegin)?;
            let args = parse_list!(self.parse_id_chain(namespace), TokenKind::Separater);
            self.tokenizer.request(TokenKind::ParenthesisEnd)?;

            // ;
            self.tokenizer.request(TokenKind::Semicolon)?;

            return Ok(Some(vec!(unchecked::SysDCSpawnDetail::new_let_to(let_to, (Name::new_root(), Type::from(func)), args))));
        }

        // use
        if self.tokenizer.expect(TokenKind::Use)?.is_some() {
            let mut var_list = vec!();
            for token in parse_list!(self.tokenizer.expect(TokenKind::Identifier), TokenKind::Separater) {
                var_list.push(unchecked::SysDCSpawnDetail::new_use(Name::new(namespace, token.orig), Type::new_unsovled_nohint()))
            }
            self.tokenizer.request(TokenKind::Semicolon)?;
            return Ok(Some(var_list));
        }

        // return
        if self.tokenizer.expect(TokenKind::Return)?.is_some() {
            match self.parse_id_chain(namespace)? {
                Some((name, _)) => {
                    self.tokenizer.request(TokenKind::Semicolon)?;
                    return Ok(Some(vec!(unchecked::SysDCSpawnDetail::new_return(name, Type::new_unsovled_nohint()))));
                },
                None => return PErrorKind::ResultOfSpawnNotSpecified.to_err_with_loc(self.tokenizer.get_now_ref_loc())
            }
        }

        Ok(None)
    }

    /**
     * <id_chain> ::= <id_list, delimiter=.>
     */
    fn parse_id_chain(&mut self, namespace: &Name) -> PResult<Option<(Name, Type)>> {
        let name_elems = parse_list!(self.tokenizer.expect(TokenKind::Identifier), TokenKind::Accessor);
        let var = name_elems.into_iter().map(|x| x.orig).collect::<Vec<String>>().join(".");
        match var.len() {
            0 => Ok(None),
            _ => Ok(Some((Name::new(namespace, var), Type::new_unsovled_nohint())))
        }
    }

    /**
     * <id_type_mapping> ::= <id> : <type>
     */
    fn parse_id_type_mapping(&mut self, namespace: &Name) -> PResult<Option<(Name, Type)>> {
        let id1 = if let Some(id1_token) = self.tokenizer.expect(TokenKind::Identifier)? {
            id1_token.orig
        } else {
            return Ok(None);
        };
        self.tokenizer.request(TokenKind::Mapping)?;
        Ok(Some((Name::new(namespace, id1), self.parse_type()?)))
    }

    /**
     * <type> ::= <id>
     */
    fn parse_type(&mut self) -> PResult<Type> {
        // <id>
        let id = self.tokenizer.request(TokenKind::Identifier)?.orig;
        Ok(Type::from(id))
    }
}

#[cfg(test)]
mod test {
    use super::UnitParser;
    use super::super::name::Name;
    use super::super::types::Type;
    use super::super::token::Tokenizer;
    use super::super::structure::unchecked::{ SysDCUnit, SysDCData, SysDCModule, SysDCFunction, SysDCAnnotation, SysDCSpawnDetail };

    #[test]
    fn empty() {
        let program = "unit test;";
        compare_unit(program, SysDCUnit::new(generate_name_for_test(), vec!(), vec!(), vec!()));
    }

    #[test]
    fn import_simple() {
        let program = "
            unit test;

            from outer import A;
            from outer2.in import B;
        ";

        let name = Name::new_root();
        let name_import_1 = Name::new(&Name::new(&name, "outer".to_string()), "A".to_string());
        let name_import_2 = Name::new(&Name::new(&Name::new(&name, "outer2".to_string()), "in".to_string()), "B".to_string());
        let name_imports = vec!(name_import_1, name_import_2);

        compare_unit(program, SysDCUnit::new(generate_name_for_test(), vec!(), vec!(), name_imports));
    }

    #[test]
    fn import_multiple() {
        let program = "
            unit test;

            from outer import A, B;
            from outer2.in import C, D, E;
        ";

        let name = Name::new_root();
        let name_import_1 = Name::new(&Name::new(&name, "outer".to_string()), "A".to_string());
        let name_import_2 = Name::new(&Name::new(&name, "outer".to_string()), "B".to_string());
        let name_import_3 = Name::new(&Name::new(&Name::new(&name, "outer2".to_string()), "in".to_string()), "C".to_string());
        let name_import_4 = Name::new(&Name::new(&Name::new(&name, "outer2".to_string()), "in".to_string()), "D".to_string());
        let name_import_5 = Name::new(&Name::new(&Name::new(&name, "outer2".to_string()), "in".to_string()), "E".to_string());
        let name_imports = vec!(name_import_1, name_import_2, name_import_3, name_import_4, name_import_5);

        compare_unit(program, SysDCUnit::new(generate_name_for_test(), vec!(), vec!(), name_imports));
    }

    #[test]
    fn data_empty_ok() {
        let program = "
            unit test;

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
        let unit = SysDCUnit::new(name, data, vec!(), vec!());

        compare_unit(program, unit);
    }

    #[test]
    fn data_has_member_ok() {
        let program = "
            unit test;

            data Box {
                x: i32,
                y: UserDefinedData,
            }
        ";

        let name = generate_name_for_test();
        let name_box = Name::new(&name, "Box".to_string());

        let member = vec!(
            (Name::new(&name_box, "x".to_string()), Type::from("i32".to_string())),
            (Name::new(&name_box, "y".to_string()), Type::from("UserDefinedData".to_string()))
        );
        let data = SysDCData::new(name_box, member);
        let unit = SysDCUnit::new(name, vec!(data), vec!(), vec!());

        compare_unit(program, unit);
    }

    #[test]
    #[should_panic]
    fn data_has_illegal_member_def_1() {
        let program = "
            unit test;

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
            unit test;

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
            unit test;

            data Box
                x: i32,
                y: i32
        ";
        parse(program);
    }

    #[test]
    fn module_empty_ok() {
        let program = "
            unit test;

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
        let unit = SysDCUnit::new(name, vec!(), module, vec!());

        compare_unit(program, unit);
    }

    #[test]
    fn function_only_has_return() {
        let program = "
            unit test;

            module BoxModule {
                func new() -> Box {
                    @return box
                }
            }
        ";

        let name = generate_name_for_test();
        let name_module = Name::new(&name, "BoxModule".to_string());
        let name_func = Name::new(&name_module, "new".to_string());
        let name_func_ret = Name::new(&name_func, "box".to_string());

        let func_returns = Some((name_func_ret, Type::from("Box".to_string())));
        let func = SysDCFunction::new(name_func, vec!(), func_returns, vec!());
        let module = SysDCModule::new(name_module, vec!(func));

        let unit = SysDCUnit::new(name, vec!(), vec!(module), vec!());

        compare_unit(program, unit);
    }

    #[test]
    fn function_has_return_and_spawn() {
        let program = "
            unit test;

            module BoxModule {
                func new() -> Box {
                    @return box

                    @spawn box: Box
                }
            }
        ";

        let name = generate_name_for_test();
        let name_module = Name::new(&name, "BoxModule".to_string());
        let name_func = Name::new(&name_module, "new".to_string());
        let name_func_spawn_box = Name::new(&name_func, "box".to_string());
        let name_func_ret = Name::new(&name_func, "box".to_string());

        let func_annotations = vec!(
            SysDCAnnotation::new_spawn((name_func_spawn_box, Type::from("Box".to_string())), vec!())
        );
        let func_returns = Some((name_func_ret, Type::from("Box".to_string())));
        let func = SysDCFunction::new(name_func, vec!(), func_returns, func_annotations);
        let module = SysDCModule::new(name_module, vec!(func));

        let unit = SysDCUnit::new(name, vec!(), vec!(module), vec!());

        compare_unit(program, unit);
    }

    #[test]
    fn function_has_full() {
        let program = "
            unit test;

            module BoxModule {
                func move(box: Box, dx: i32, dy: i32) -> Box {
                    @return movedBox

                    @spawn movedBox: Box {
                        use box;
                        use dx, dy;

                        let movedBox = UnknownModule.function(dx);

                        return movedBox;
                    }
                }
            }
        ";

        let name = generate_name_for_test();
        let name_module = Name::new(&name, "BoxModule".to_string());
        let name_func = Name::new(&name_module, "move".to_string());
        let name_func_arg_box = Name::new(&name_func, "box".to_string());
        let name_func_arg_dx = Name::new(&name_func, "dx".to_string());
        let name_func_arg_dy = Name::new(&name_func, "dy".to_string());
        let name_func_spawn_box = Name::new(&name_func, "movedBox".to_string());
        let name_func_spawn_use_box = Name::new(&name_func, "_.box".to_string());
        let name_func_spawn_use_dx = Name::new(&name_func, "_._.dx".to_string());
        let name_func_spawn_use_dy = Name::new(&name_func, "_._.dy".to_string());
        let name_func_spawn_let_name = Name::new(&name_func, "_._._.movedBox".to_string());
        let name_func_spawn_let_arg_dx = Name::new(&name_func, "_._._.dx".to_string());
        let name_func_spawn_ret = Name::new(&name_func, "_._._._.movedBox".to_string());
        let name_func_ret = Name::new(&name_func, "movedBox".to_string());

        let func_args = vec!(
            (name_func_arg_box, Type::from("Box".to_string())),
            (name_func_arg_dx, Type::from("i32".to_string())),
            (name_func_arg_dy, Type::from("i32".to_string()))
        );
        let func_annotations = vec!(
            SysDCAnnotation::new_spawn((name_func_spawn_box, Type::from("Box".to_string())), vec!(
                SysDCSpawnDetail::new_use(name_func_spawn_use_box, Type::new_unsovled_nohint()),
                SysDCSpawnDetail::new_use(name_func_spawn_use_dx, Type::new_unsovled_nohint()),
                SysDCSpawnDetail::new_use(name_func_spawn_use_dy, Type::new_unsovled_nohint()),
                SysDCSpawnDetail::new_let_to(
                    name_func_spawn_let_name,
                    (Name::new_root(), Type::from("UnknownModule.function".to_string())),
                    vec!((name_func_spawn_let_arg_dx, Type::new_unsovled_nohint()))
                ),
                SysDCSpawnDetail::new_return(name_func_spawn_ret, Type::new_unsovled_nohint())
            ))
        );
        let func_returns = Some((name_func_ret, Type::from("Box".to_string())));
        let func = SysDCFunction::new(name_func, func_args, func_returns, func_annotations);
        let module = SysDCModule::new(name_module, vec!(func));

        let unit = SysDCUnit::new(name, vec!(), vec!(module), vec!());

        compare_unit(program, unit);
    }

    #[test]
    #[should_panic]
    fn illegal_function_1() {
        let program = "
            unit test;

            module BoxModule {
                func move() -> {

                }
            }
        ";
        parse(program);
    }

    #[test]
    #[should_panic]
    fn illegal_function_2() {
        let program = "
            unit test;

            module BoxModule {
                func move(box: Box, dx: i32, dy: ) -> i32 {

                }
            }
        ";
        parse(program);
    }

    #[test]
    #[should_panic]
    fn illegal_function_3() {
        let program = "
            unit test;

            module BoxModule {
                func move() {

                }
            }
        ";
        parse(program);
    }

    #[test]
    #[should_panic]
    fn illegal_function_4() {
        let program = "
            unit test;

            module BoxModule {
                move() -> i32 {
                    @return a
                }
            }
        ";
        parse(program);
    }

    #[test]
    fn procedure_simple() {
        let program = "
            unit test;

            module BoxModule {
                proc new() {
                }
            }
        ";

        let name = generate_name_for_test();
        let name_module = Name::new(&name, "BoxModule".to_string());
        let name_proc = Name::new(&name_module, "new".to_string());

        let proc = SysDCFunction::new(name_proc, vec!(), None, vec!());
        let module = SysDCModule::new(name_module, vec!(proc));

        let unit = SysDCUnit::new(name, vec!(), vec!(module), vec!());

        compare_unit(program, unit);
    }

    #[test]
    fn proc_has_spawn() {
        let program = "
            unit test;

            module BoxModule {
                proc new() {
                    @spawn box: Box
                }
            }
        ";

        let name = generate_name_for_test();
        let name_module = Name::new(&name, "BoxModule".to_string());
        let name_proc = Name::new(&name_module, "new".to_string());
        let name_proc_spawn_box = Name::new(&name_proc, "box".to_string());

        let proc_annotations = vec!(
            SysDCAnnotation::new_spawn((name_proc_spawn_box, Type::from("Box".to_string())), vec!())
        );
        let proc = SysDCFunction::new(name_proc, vec!(), None, proc_annotations);
        let module = SysDCModule::new(name_module, vec!(proc));

        let unit = SysDCUnit::new(name, vec!(), vec!(module), vec!());

        compare_unit(program, unit);
    }

    #[test]
    #[should_panic]
    fn illegal_procedure_1() {
        let program = "
            unit test;

            module BoxModule {
                proc move() -> i32 {

                }
            }
        ";
        parse(program);
    }

    #[test]
    #[should_panic]
    fn illegal_procedure_2() {
        let program = "
            unit test;

            module BoxModule {
                proc move() -> {

                }
            }
        ";
        parse(program);
    }

    #[test]
    #[should_panic]
    fn illegal_procedure_3() {
        let program = "
            unit test;

            module BoxModule {
                proc move() {
                    @return a
                }
            }
        ";
        parse(program);
    }

    #[test]
    fn full() {
        let program = "
            unit test;

            from outer import A;
            from outer2.in import C, D, E;

            data Box {
                x: i32,
                y: i32
            }

            module BoxModule {
                func move(box: Box, dx: i32, dy: i32) -> Box {
                    @return movedBox

                    @spawn movedBox: Box {
                        use box, dx, dy;

                        let movedBox = UnknownModule.function(dx);

                        return movedBox;
                    }
                }
            }
        ";

        let name = Name::new_root();
        let name_import_1 = Name::new(&Name::new(&name, "outer".to_string()), "A".to_string());
        let name_import_2 = Name::new(&Name::new(&Name::new(&name, "outer2".to_string()), "in".to_string()), "C".to_string());
        let name_import_3 = Name::new(&Name::new(&Name::new(&name, "outer2".to_string()), "in".to_string()), "D".to_string());
        let name_import_4 = Name::new(&Name::new(&Name::new(&name, "outer2".to_string()), "in".to_string()), "E".to_string());
        let name_imports = vec!(name_import_1, name_import_2, name_import_3, name_import_4);

        let name = generate_name_for_test();
        let name_data = Name::new(&name, "Box".to_string());
        let name_data_x = Name::new(&name_data, "x".to_string());
        let name_data_y = Name::new(&name_data, "y".to_string());
        let name_module = Name::new(&name, "BoxModule".to_string());
        let name_func = Name::new(&name_module, "move".to_string());
        let name_func_arg_box = Name::new(&name_func, "box".to_string());
        let name_func_arg_dx = Name::new(&name_func, "dx".to_string());
        let name_func_arg_dy = Name::new(&name_func, "dy".to_string());
        let name_func_spawn_box = Name::new(&name_func, "movedBox".to_string());
        let name_func_spawn_use_box = Name::new(&name_func, "_.box".to_string());
        let name_func_spawn_use_dx = Name::new(&name_func, "_.dx".to_string());
        let name_func_spawn_use_dy = Name::new(&name_func, "_.dy".to_string());
        let name_func_spawn_let_name = Name::new(&name_func, "_._.movedBox".to_string());
        let name_func_spawn_let_arg_dx = Name::new(&name_func, "_._.dx".to_string());
        let name_func_spawn_ret = Name::new(&name_func, "_._._.movedBox".to_string());
        let name_func_ret = Name::new(&name_func, "movedBox".to_string());

        let func_args = vec!(
            (name_func_arg_box, Type::from("Box".to_string())),
            (name_func_arg_dx, Type::from("i32".to_string())),
            (name_func_arg_dy, Type::from("i32".to_string()))
        );
        let func_annotations = vec!(
            SysDCAnnotation::new_spawn((name_func_spawn_box, Type::from("Box".to_string())), vec!(
                SysDCSpawnDetail::new_use(name_func_spawn_use_box, Type::new_unsovled_nohint()),
                SysDCSpawnDetail::new_use(name_func_spawn_use_dx, Type::new_unsovled_nohint()),
                SysDCSpawnDetail::new_use(name_func_spawn_use_dy, Type::new_unsovled_nohint()),
                SysDCSpawnDetail::new_let_to(
                    name_func_spawn_let_name,
                    (Name::new_root(), Type::from("UnknownModule.function".to_string())),
                    vec!((name_func_spawn_let_arg_dx, Type::new_unsovled_nohint()))
                ),
                SysDCSpawnDetail::new_return(name_func_spawn_ret, Type::new_unsovled_nohint())
            ))
        );
        let func_returns = Some((name_func_ret, Type::from("Box".to_string())));
        let func = SysDCFunction::new(name_func, func_args, func_returns, func_annotations);
        let module = SysDCModule::new(name_module, vec!(func));

        let data_members = vec!(
            (name_data_x, Type::from("i32".to_string())),
            (name_data_y, Type::from("i32".to_string()))
        );
        let data = SysDCData::new(name_data, data_members);

        let unit = SysDCUnit::new(name, vec!(data), vec!(module), name_imports);

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
        UnitParser::parse(tokenizer).unwrap()
    }
}
