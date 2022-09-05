use super::name::Name;
use super::types::{ Type, TypeKind };
use super::error::{ PResult, PError, PErrorKind };
use super::structure::{ SysDCSystem, SysDCUnit, SysDCData, SysDCModule, SysDCFunction, SysDCSpawn, SysDCSpawnChild  };
use super::structure::unchecked;

pub struct Checker {
    def_manager: DefinesManager,
    imports: Vec<Name>
}

impl Checker {
    pub fn check(system: unchecked::SysDCSystem) -> PResult<SysDCSystem> {
        let mut checker = Checker { def_manager: DefinesManager::new(&system)?, imports: vec!() };
        system.convert(|unit| checker.check_unit(unit))
    }

    fn check_unit(&mut self, unit: unchecked::SysDCUnit) -> PResult<SysDCUnit> {
        let mut imports = vec!();
        for import in unit.imports.clone() {
            self.def_manager.check_can_import(import.clone(), &vec!())?;
            imports.push(import);
        }
        self.imports = imports;

        unit.convert(
            |data| self.check_data(data,),
            |module| self.check_module(module),
        )
    }

    fn check_data(&self, data: unchecked::SysDCData) -> PResult<SysDCData>{
        data.convert(|(name, types): (Name, Type)|
            if types.kind.is_primitive() {
                Ok((name, types))
            } else {
                self.def_manager.resolve_from_type((name, types), &self.imports)
            }
        )
    }

    fn check_module(&self, module: unchecked::SysDCModule) -> PResult<SysDCModule> {
        module.convert(|func| self.check_function(func))
    }

    fn check_function(&self, func: unchecked::SysDCFunction) -> PResult<SysDCFunction> {
        let req_ret_type = self.def_manager.resolve_from_type(func.returns.clone().unwrap(), &self.imports)?.1;

        let a_converter = |arg| self.def_manager.resolve_from_type(arg, &self.imports);
        let r_converter = |returns: Option<(Name, Type)>| {
            let (ret_name, _) = returns.unwrap();
            let ret = self.def_manager.resolve_from_name(ret_name.clone(), &self.imports)?;
            Ok(Some(ret))
        };
        let func = func.convert(a_converter, r_converter, |spawn| self.check_spawn(spawn))?;

        let act_ret_type = &func.returns.as_ref().unwrap().1;
        if &req_ret_type != act_ret_type {
            return PError::new(PErrorKind::TypeUnmatch2(req_ret_type, act_ret_type.clone()));
        }
        Ok(func)
    }

    fn check_spawn(&self, spawn: unchecked::SysDCSpawn) -> PResult<SysDCSpawn> {
        let req_ret_type = self.def_manager.resolve_from_type(spawn.result.clone(), &self.imports)?.1;

        let spawn = spawn.convert(
            |(name, _)| self.def_manager.resolve_from_name(name.clone(), &self.imports),
            |spawn_child| self.check_spawn_child(spawn_child)
        )?;

        for spawn_child in &spawn.details {
            match spawn_child {
                SysDCSpawnChild::Return(_, act_ret_type) =>
                    if &req_ret_type != act_ret_type {
                        return PError::new(PErrorKind::TypeUnmatch2(req_ret_type, act_ret_type.clone()));
                    }
                _ => {}
            }
        }
        Ok(spawn)
    }

    fn check_spawn_child(&self, spawn_child: unchecked::SysDCSpawnChild) -> PResult<SysDCSpawnChild> {
        let ur_converter = |(name, _): (Name, Type)| self.def_manager.resolve_from_name(name.clone(), &self.imports);
        let l_converter = |name: Name, func: (Name, Type), args: Vec<(Name, Type)>| {
            if let Type { kind: TypeKind::Unsolved(_), .. } = func.1 {
                let mut let_to_args = vec!();
                for (arg_name, _) in args {
                    let (arg_name, arg_type) = self.def_manager.resolve_from_name(arg_name.clone(), &self.imports)?;
                    let_to_args.push((arg_name, arg_type));
                }
                let resolved_func = self.def_manager.resolve_from_type((name.clone(), func.1), &self.imports)?;
                return Ok((name, resolved_func, let_to_args));
            }
            panic!("Internal Error")
        };
        let spawn_child = spawn_child.convert(ur_converter, ur_converter, l_converter)?;

        match &spawn_child {
            SysDCSpawnChild::LetTo { func: (func, _), args, .. } => {
                for ((_, act_arg_type), req_arg_type) in args.iter().zip(self.def_manager.get_args_type(&func, &self.imports)?.iter()) {
                    if act_arg_type != req_arg_type {
                        return PError::new(PErrorKind::TypeUnmatch2(req_arg_type.clone(), act_arg_type.clone()));
                    }
                }
            }
            _ => {}
        }
        Ok(spawn_child)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum DefineKind {
    Data,
    DataMember(Type),
    Module,
    Function(Type),
    Argument(Type),
    Variable(Type),
    Use(Name)
}

#[derive(Debug)]
struct Define {
    kind: DefineKind,
    refs: Name
}

impl Define {
    pub fn new(kind: DefineKind, refs: Name) -> Define {
        Define { kind, refs }
    }
}

struct DefinesManager {
    defines: Vec<Define>
}

impl DefinesManager {
    pub fn new(system: &unchecked::SysDCSystem) -> PResult<DefinesManager> {
        let mut def_manager = DefinesManager { defines: vec!() };
        def_manager.listup_defines(system)?;
        Ok(def_manager)
    }

    // 与えられたnameと同じ名前を持つ定義が存在するかどうかを確認する
    pub fn check_can_import(&self, name: Name, imports: &Vec<Name>) -> PResult<()> {
        match self.find(name.clone(), &name.name, imports)?.kind {
            DefineKind::Data | DefineKind::Module => Ok(()),
            _ => PError::new(PErrorKind::NotDefined(name.name))
        }
    }

    // 与えられたnameから参照可能なすべての範囲またはimports内を対象に，typesと一致する定義を探す (Data, Module, Function)
    // ※name, typesはともに関連している状態を想定
    pub fn resolve_from_type(&self, (name, types): (Name, Type), imports: &Vec<Name>) -> PResult<(Name, Type)> {
        if types.kind.is_primitive() || types.kind == TypeKind::Data {
            return Ok((name, types))
        }

        if let TypeKind::Unsolved(hint) = &types.kind {
            let (head, tails) = DefinesManager::split_name(&hint);
            let found_def = self.find(name.clone(), &head, &imports)?;
            return match found_def.kind {
                DefineKind::Data =>
                    match tails {
                        Some(_) => PError::new(PErrorKind::IllegalAccess),
                        None => Ok((name, Type::new(TypeKind::Data, Some(found_def.refs))))
                    }
                DefineKind::Module =>
                    match tails {
                        Some(tails) => self.get_func_in_module(&found_def.refs, &tails, imports),
                        None => PError::new(PErrorKind::MissingFunctionName)
                    }
                DefineKind::Function(_) => {
                    self.get_func_in_module(&name.get_namespace(true), &hint, imports)
                }
                _ => PError::new(PErrorKind::TypeUnmatch1(types))
            }
        }

        panic!("Internal Error");
    }

    // nameから参照可能なすべての範囲またはimports内を対象に，nameと一致する名前をもつ定義を探す (Variable)
    pub fn resolve_from_name(&self, name: Name, imports: &Vec<Name>) -> PResult<(Name, Type)> {
        let (head, tails) = DefinesManager::split_name(&name.name);
        let found_def = self.find(name.clone(), &head, &vec!())?;
        match found_def.kind {
            DefineKind::Variable(types) => {
                let (_, types) = self.resolve_from_type((name.clone(), types), imports)?;
                match types.kind {
                    TypeKind::Data =>
                        match tails {
                            Some(tails) => {
                                let (_, types) = self.get_member_in_data(types.refs.as_ref().unwrap(), &tails, imports)?;
                                Ok((name, types))
                            }
                            None => Ok((found_def.refs, types))
                        }
                    _ => Ok((found_def.refs, types))
                }
            }
            DefineKind::Use(use_ref) => {
                match tails {
                    Some(_) => {
                        let (dname, _) = self.resolve_from_name(use_ref, imports)?;
                        self.resolve_from_name(Name::from(&dname.get_par_name(false), name.name.clone()), imports)
                    },
                    None => self.resolve_from_name(use_ref, imports)
                }
            }
            _ => PError::new(PErrorKind::NotDefined(name.name))
        }
    }

    // 与えられた関数名に対応する関数を探し，関数に登録されている引数の型の一覧を返す
    pub fn get_args_type(&self, func_name: &Name, imports: &Vec<Name>) -> PResult<Vec<Type>> {
        let func_name = func_name.get_full_name();
        let mut args = vec!();
        for Define { kind, refs } in &self.defines {
            if let DefineKind::Argument(types) = kind {
                if &refs.namespace == &func_name {
                    args.push(self.resolve_from_type((refs.clone(), types.clone()), imports)?.1);
                }
            }
        }
        Ok(args)
    }

    // data(Data)内のmember(Member)の定義を探す
    fn get_member_in_data(&self, data: &Name, member: &String, imports: &Vec<Name>) -> PResult<(Name, Type)> {
        let (head, tails) = DefinesManager::split_name(&member);
        for Define { kind, refs } in &self.defines {
            if let DefineKind::DataMember(types) = kind {
                if data.get_full_name() == refs.namespace && head == refs.name {
                    let (_, types) = self.resolve_from_type((refs.clone(), types.clone()), imports)?;
                    if types.kind.is_primitive() {
                        return match tails {
                            Some(_) => PError::new(PErrorKind::IllegalAccess),
                            None => Ok((refs.clone(), types))
                        };
                    }
                    if types.kind == TypeKind::Data {
                        return match tails {
                            Some(tails) => self.get_member_in_data(types.refs.as_ref().unwrap(), &tails, imports),
                            None => Ok((types.refs.clone().unwrap(), types))
                        };
                    }
                    panic!("Internal Error");
                }
            }
        }
        PError::new(PErrorKind::MemberNotDefinedInData(member.clone(), data.name.clone()))
    }

    // module(Module)内のfunc(Function)の定義を探す
    fn get_func_in_module(&self, module: &Name, func: &String, imports: &Vec<Name>) -> PResult<(Name, Type)> {
        for Define { kind, refs } in &self.defines {
            if let DefineKind::Function(types) = kind {
                if module == &refs.get_par_name(true) && func == &refs.name {
                    return Ok((refs.clone(), self.resolve_from_type((refs.clone(), types.clone()), imports)?.1));
                }
            }
        }
        PError::new(PErrorKind::FuncNotDefinedInModule(func.clone(), module.name.clone()))
    }

    // namespace内に存在する定義を対象に，nameと同じ名前を持つ定義を探して返す
    // namespace内に存在しない場合はimports内の名前を探して返す
    // ※namespaceはルートにたどり着くまで再帰的に更新されながら検索が続く (.a.b.c -> .a.b -> .a -> .)
    fn find(&self, mut namespace: Name, name: &String, imports: &Vec<Name>) -> PResult<Define> {
        let had_underscore = namespace.has_underscore();
        while namespace.name.len() > 0 {
            for Define{ kind, refs } in &self.defines {
                if refs.namespace == namespace.namespace && &refs.name == name {
                    if let DefineKind::Variable(_) = kind {
                        if had_underscore && !refs.has_underscore() {
                            continue;
                        }
                    }
                    return Ok(Define::new(kind.clone(), refs.clone()))
                }
            }
            namespace = namespace.get_par_name(false);
        }

        for import in imports {
            if &import.name == name {
                return self.find(import.clone(), &import.name, &vec!());
            }
        }

        PError::new(PErrorKind::NotFound(name.clone()))
    }

    fn split_name(hint: &String) -> (String, Option<String>) {
        let splitted_hint = hint.split(".").collect::<Vec<&str>>();
        match splitted_hint.len() {
            1 => (splitted_hint[0].to_string(), None),
            _ => (splitted_hint[0].to_string(), Some(splitted_hint[1..].join(".")))
        }
    }

    fn define(&mut self, def: Define) -> PResult<()> {
        match &self.find(def.refs.clone(), &def.refs.name, &vec!()) {
            Ok(Define{ kind, .. }) =>
                match (kind, &def.kind) {
                    (DefineKind::Argument(_), _) => {},
                    (_, DefineKind::Argument(_)) => {},
                    _ => return PError::new(PErrorKind::AlreadyDefined(def.refs.name))
                }
            Err(_) => {}
        }
        self.defines.push(def);
        Ok(())
    }

    fn listup_defines(&mut self, system: &unchecked::SysDCSystem) -> PResult<()> {
        for unit in &system.units {
            self.listup_defines_unit(unit)?;
        }
        Ok(())
    }

    fn listup_defines_unit(&mut self, unit: &unchecked::SysDCUnit) -> PResult<()> {
        for data in &unit.data {
            self.define(Define::new(DefineKind::Data, data.name.clone()))?;
            self.listup_defines_data(data)?;
        }
        for module in &unit.modules {
            self.define(Define::new(DefineKind::Module, module.name.clone()))?;
            self.listup_defines_module(module)?;
        }
        Ok(())
    }

    fn listup_defines_data(&mut self, data: &unchecked::SysDCData) -> PResult<()> {
        for (name, types) in &data.members {
            self.define(Define::new(DefineKind::DataMember(types.clone()), name.clone()))?;
        }
        Ok(())
    }

    fn listup_defines_module(&mut self, module: &unchecked::SysDCModule) -> PResult<()> {
        for func in &module.functions {
            self.define(Define::new(DefineKind::Function(func.returns.as_ref().unwrap().1.clone()), func.name.clone()))?;
            self.listup_defines_function(func)?;
        }
        Ok(())
    }

    fn listup_defines_function(&mut self, func: &unchecked::SysDCFunction) -> PResult<()> {
        for (name, types) in &func.args {
            self.define(Define::new(DefineKind::Variable(types.clone()), name.clone()))?;
            self.define(Define::new(DefineKind::Argument(types.clone()), name.clone()))?;
        }
        for spawn@unchecked::SysDCSpawn { result: (name, types), .. } in &func.spawns {
            self.define(Define::new(DefineKind::Variable(types.clone()), name.clone()))?;
            self.listup_defines_function_spawn_details(spawn)?;
        }
        Ok(())
    }

    fn listup_defines_function_spawn_details(&mut self, spawn: &unchecked::SysDCSpawn) -> PResult<()> {
        for detail in &spawn.details {
            match detail {
                unchecked::SysDCSpawnChild::Use(name, _) => {
                    let outer_spawn_namespace = name.clone().get_par_name(true);
                    let outer_use_name = Name::from(&outer_spawn_namespace, name.clone().name);
                    self.define(Define::new(DefineKind::Use(outer_use_name.clone()), name.clone()))?;
                }
                unchecked::SysDCSpawnChild::LetTo { name, func: (_, func), ..} => {
                    self.define(Define::new(DefineKind::Variable(func.clone()), name.clone()))?;
                }
                _ => {}
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::Parser;

    #[test]
    fn primitive_member_only_data() {
        let program = "
            unit test;

            data Test {
                a: i32,
                b: i32,
                c: i32
            }
        ";
        check(vec!(program));
    }

    #[test]
    fn user_defined_type_mix_data() {
        let program = "
            unit test;

            data A {
                a: i32
            }

            data Test {
                a: A,
                b: i32,
                c: A
            }
        ";
        check(vec!(program));
    }

    #[test]
    fn recursive_type_mix_data() {
        let program = "
            unit test;

            data A {
                a: A
            }
        ";
        check(vec!(program));
    }

    #[test]
    #[should_panic]
    fn undefined_type_mix_data() {
        let program = "
            unit test;

            data Test {
                a: i32,
                b: Unknown
            }
        ";
        check(vec!(program));
    }

    #[test]
    fn user_defind_type_mix_module() {
        let program = "
            unit test;

            data A {
                a: i32,
                b: i32
            }

            module TestModule {
                test(a: A) -> A {
                    @return a
                }
            }
        ";
        check(vec!(program));
    }

    #[test]
    fn user_defined_type_mix_module_with_spawn() {
        let program = "
            unit test;

            data A {
                a: B,
                b: B
            }

            data B {
                a: C,
                b: C
            }

            data C {
                a: i32,
                b: i32
            }

            module TestModule {
                test(a: A) -> A {
                    @return b

                    @spawn b: A {
                        use a;
                        let tmp = receiveA(a);
                        let tmp1 = receiveB(a.a);
                        let tmp2 = receiveB(a.b);
                        let tmp3 = receiveC(a.a.a);
                        let tmp4 = receiveC(a.a.b);
                        let tmp5 = receiveC(a.b.a);
                        let tmp6 = receiveC(a.b.b);
                        let tmp7 = receiveInt32(a.a.a.a);
                        let tmp8 = receiveInt32(a.a.a.b);
                        let tmp9 = receiveInt32(a.a.b.a);
                        let tmp10 = receiveInt32(a.a.b.b);
                        let tmp11 = receiveInt32(a.b.a.a);
                        let tmp12 = receiveInt32(a.b.a.b);
                        let tmp13 = receiveInt32(a.b.b.a);
                        let tmp14 = receiveInt32(a.b.b.b);
                    }
                }

                receiveA(a: A) -> i32 {
                    @return tmp
                    @spawn tmp: i32
                }

                receiveB(b: B) -> i32 {
                    @return tmp
                    @spawn tmp: i32
                }

                receiveC(c: C) -> i32 {
                    @return tmp
                    @spawn tmp: i32
                }

                receiveInt32(i: i32) -> i32 {
                    @return tmp
                    @spawn tmp: i32
                }
            }
        ";
        check(vec!(program))
    }

    #[test]
    #[should_panic]
    fn user_defind_type_mix_module_with_spawn_failure_1() {
        let program = "
            unit test;

            data A {
                a: i32,
                b: i32
            }

            module TestModule {
                test(a: A) -> A {
                    @return b

                    @spawn b: A {
                        use aaa;
                    }
                }
            }
        ";
        check(vec!(program));
    }

    #[test]
    #[should_panic]
    fn user_defind_type_mix_module_with_spawn_failure_2() {
        let program = "
            unit test;

            data A {
                a: i32,
                b: i32
            }

            module TestModule {
                test(a: A) -> A {
                    @return b

                    @spawn b: A {
                        use a;
                        let tmp = receiveInt32(a.c);
                    }
                }

                receiveInt32(i: i32) -> i32 {
                    @return tmp
                    @spawn tmp: i32
                }
            }
        ";
        check(vec!(program));
    }

    #[test]
    #[should_panic]
    fn user_defind_type_mix_module_with_spawn_failure_3() {
        let program = "
            unit test;

            data A {
                a: B,
                b: B
            }

            data B {
                a: C,
                b: C
            }

            data C {
                a: i32,
                b: i32
            }

            module TestModule {
                test(a: A) -> A {
                    @return b

                    @spawn b: A {
                        use a.b.a.c;
                    }
                }
            }
        ";
        check(vec!(program));
    }

    #[test]
    fn let_by_user_defined_function_using_completed_name_1() {
        let program = "
            unit test;

            data A {}

            module AModule {
                new() -> A {
                    @return a

                    @spawn a: A
                }
            }

            module TestModule {
                test() -> A {
                    @return a

                    @spawn a: A {
                        let b = AModule.new();
                        return b;
                    }
                }
            }
        ";
        check(vec!(program));
    }

    #[test]
    fn let_by_user_defined_function_using_completed_name_2() {
        let program = "
            unit test;

            data A {}

            module TestModule {
                new() -> A {
                    @return a

                    @spawn a: A
                }

                test() -> A {
                    @return a

                    @spawn a: A {
                        let b = TestModule.new();
                        return b;
                    }
                }
            }
        ";
        check(vec!(program));
    }

    #[test]
    fn let_by_user_defined_function_using_uncompleted_name() {
        let program = "
            unit test;

            data A {}

            module TestModule {
                new() -> A {
                    @return a

                    @spawn a: A
                }

                test() -> A {
                    @return a

                    @spawn a: A {
                        let b = new();
                        return b;
                    }
                }
            }
        ";
        check(vec!(program));
    }

    #[test]
    #[should_panic]
    fn let_by_user_defined_function_using_completed_name_failure() {
        let program = "
            unit test;

            data A {}

            module TestModule {
                new() -> A {
                    @return a

                    @spawn a: A
                }

                test() -> A {
                    @return a

                    @spawn a: A {
                        let b = TestModule.new2();
                        return b;
                    }
                }
            }
        ";
        check(vec!(program));
    }

    #[test]
    #[should_panic]
    fn let_by_user_defined_function_using_uncompleted_name_failure() {
        let program = "
            unit test;

            data A {}

            module TestModule {
                new() -> A {
                    @return a

                    @spawn a: A
                }

                test() -> A {
                    @return a

                    @spawn a: A {
                        let b = new2();
                        return b;
                    }
                }
            }
        ";
        check(vec!(program));
    }

    #[test]
    fn argument_check_ok() {
        let program = "
            unit test;

            data A {}
            data B {}
            data C {}

            module TestModule {
                test() -> A {
                    @return a

                    @spawn a: A {
                        let tmp1 = genA();
                        let tmp2 = genB(tmp1);
                        let tmp3 = genC(tmp1, tmp2);
                    }
                }

                genA() -> A {
                    @return a

                    @spawn a: A
                }

                genB(a: A) -> B {
                    @return b

                    @spawn b: B {
                        use a;
                    }
                }

                genC(a: A, b: B) -> C {
                    @return c

                    @spawn c: C {
                        use a, b;
                    }
                }
            }
        ";
        check(vec!(program));
    }

    #[test]
    #[should_panic]
    fn argument_check_ng() {
        let program = "
            unit test;

            data A {}
            data B {}
            data C {}

            module TestModule {
                test() -> A {
                    @return a

                    @spawn a: A {
                        let tmp1 = genA();
                        let tmp2 = genB(tmp1);
                        let tmp3 = genC(tmp1, tmp1);
                    }
                }

                genA() -> A {
                    @return a

                    @spawn a: A
                }

                genB(a: A) -> B {
                    @return b

                    @spawn b: B {
                        use a;
                    }
                }

                genC(a: A, b: B) -> C {
                    @return c

                    @spawn c: C {
                        use a, b;
                    }
                }
            }
        ";
        check(vec!(program));
    }

    #[test]
    fn return_check_ok() {
        let program = "
            unit test;

            module TestModule {
                test() -> i32 {
                    @return a

                    @spawn a: i32
                }
            }
        ";
        check(vec!(program));
    }

    #[test]
    #[should_panic]
    fn return_check_ng() {
        let program = "
            unit test;

            data A {}

            module TestModule {
                test() -> i32 {
                    @return a

                    @spawn a: A
                }
            }
        ";
        check(vec!(program));
    }

    #[test]
    fn in_spawn_return_check_ok() {
        let program = "
            unit test;

            module TestModule {
                test() -> i32 {
                    @return a

                    @spawn a: i32 {
                        let b = gen_i32();
                        return b;
                    }
                }

                gen_i32() -> i32 {
                    @return a

                    @spawn a: i32
                }
            }
        ";
        check(vec!(program));
    }

    #[test]
    #[should_panic]
    fn in_spawn_return_check_ng() {
        let program = "
            unit test;

            data A {}

            module TestModule {
                test() -> i32 {
                    @return a

                    @spawn a: i32 {
                        let b = gen_A();
                        return b;
                    }
                }

                gen_A() -> A {
                    @return a

                    @spawn a: A
                }
            }
        ";
        check(vec!(program));
    }

    #[test]
    fn import_data_in_other_unit_simple() {
        let program1 = "
            unit test.A;

            data A {}
        ";
        let program2 = "
            unit test.B;

            from test.A import A;

            data B {
                a: A
            }
        ";
        check(vec!(program1, program2));
    }

    #[test]
    fn import_data_in_other_unit_recursive() {
        let program1 = "
            unit test.A;

            data A {
                b: B
            }

            data B {
                c: C
            }

            data C {
                body: i32
            }
        ";
        let program2 = "
            unit test.B;

            from test.A import A;

            module TestModule {
                new() -> A {
                    @return a
                    @spawn a: A
                }

                test() -> i32 {
                    @return v

                    @spawn v: i32 {
                        let a = new();
                        return a.b.c.body;
                    }
                }
            }
        ";
        check(vec!(program1, program2));
    }

    #[test]
    fn import_module_in_other_unit() {
        let program1 = "
            unit test.A;

            module TestModule {
                test() -> i32 {
                    @return a

                    @spawn a: i32
                }
            }
        ";
        let program2 = "
            unit test.B;

            from test.A import TestModule;

            module TestModule2 {
                test() -> i32 {
                    @return a

                    @spawn a: i32 {
                        let a = TestModule.test();
                        return a;
                    }
                }
            }
        ";
        check(vec!(program1, program2));
    }

    #[test]
    #[should_panic]
    fn import_module_in_other_unit_failure() {
        let program1 = "
            unit test.A;

            data A {}

            module TestModule {
                test() -> A {
                    @return a

                    @spawn a: A
                }
            }
        ";
        let program2 = "
            unit test.B;

            from test.A import A, TestModule;

            module TestModule2 {
                test() -> i32 {
                    @return a

                    @spawn a: i32 {
                        let a = TestModule.test();
                        return a;
                    }
                }
            }
        ";
        check(vec!(program1, program2));
    }

    #[test]
    #[should_panic]
    fn multiple_define_1() {
        let program = "
            unit test;

            data A {}
            data A{}
        ";
        check(vec!(program));
    }

    #[test]
    #[should_panic]
    fn multiple_define_2() {
        let program = "
            unit test;

            data A {
                x: i32,
                x: i32,
                y: i32
            }
        ";
        check(vec!(program));
    }

    #[test]
    #[should_panic]
    fn multiple_define_3() {
        let program = "
            unit test;

            module TestModule {}
            module TestModule {}
        ";
        check(vec!(program));
    }

    #[test]
    #[should_panic]
    fn multiple_define_4() {
        let program = "
            unit test;

            module TestModule {
                a() -> i32 {
                    @return b

                    @spawn b: i32
                }

                a() -> i32 {
                    @return c

                    @spawn c: i32
                }
            }
        ";
        check(vec!(program));
    }

    #[test]
    #[should_panic]
    fn multiple_define_5() {
        let program = "
            unit test;

            module TestModule {
                a() -> i32 {
                    @return a

                    @spawn a: i32
                }
            }
        ";
        check(vec!(program));
    }

    #[test]
    #[should_panic]
    fn multiple_define_6() {
        let program = "
            unit test;

            module TestModule {
                a(arg: i32) -> i32 {
                    @return val

                    @spawn val: i32 {
                        return arg;
                    }
                }
            }
        ";
        check(vec!(program));
    }

    #[test]
    #[should_panic]
    fn multiple_define_7() {
        let program = "
            unit test;

            module TestModule {
                a(arg: i32) -> i32 {
                    @return val

                    @spawn val: i32 {
                        use arg;
                        return arg2;
                    }
                }
            }
        ";
        check(vec!(program));
    }

    #[test]
    #[should_panic]
    fn multiple_define_8() {
        let program = "
            unit test;

            module TestModule {
                a(arg: i32) -> i32 {
                    @return val

                    @spawn val: i32 {
                        use arg;
                        let val = new();
                        return val2;
                    }
                }

                new() -> i32 {
                    @return val

                    @spawn val: i32
                }
            }
        ";
        check(vec!(program));
    }

    fn check(programs: Vec<&str>) {
        let mut parser = Parser::new();
        for program in programs {
            parser.parse("check.def".to_string(), &program.to_string()).unwrap();
        }
        parser.check().unwrap();
    }
}
