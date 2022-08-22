use std::error::Error;

use super::name::Name;
use super::types::{ Type, TypeKind };
use super::error::{ CompileError, CompileErrorKind };
use super::structure::{ SysDCSystem, SysDCUnit, SysDCData, SysDCModule, SysDCFunction, SysDCSpawn, SysDCSpawnChild  };
use super::structure::unchecked;

pub struct Checker {
    def_manager: DefinesManager,
    imports: Vec<Name>
}

impl Checker {
    pub fn check(system: unchecked::SysDCSystem) -> Result<SysDCSystem, Box<dyn Error>> {
        let mut checker = Checker { def_manager: DefinesManager::new(&system), imports: vec!() };
        system.convert(|unit| checker.check_unit(unit))
    }

    fn check_unit(&mut self, unit: unchecked::SysDCUnit) -> Result<SysDCUnit, Box<dyn Error>> {
        let mut imports = vec!();
        for import in unit.imports.clone() {
            self.def_manager.is_defined(import.clone(), &vec!())?;
            imports.push(import);
        }
        self.imports = imports;

        unit.convert(
            |data| self.check_data(data,),
            |module| self.check_module(module),
        )
    }

    fn check_data(&self, data: unchecked::SysDCData) -> Result<SysDCData, Box<dyn Error>> {
        data.convert(|(name, types): (Name, Type)|
            match types.kind {
                TypeKind::Int32 => Ok((name, types)),
                _ => self.def_manager.resolve_from_type((name, types), &self.imports)
            }
        )
    }

    fn check_module(&self, module: unchecked::SysDCModule) -> Result<SysDCModule, Box<dyn Error>> {
        module.convert(|func| self.check_function(func))
    }

    fn check_function(&self, func: unchecked::SysDCFunction) -> Result<SysDCFunction, Box<dyn Error>> {
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
            return CompileError::new(CompileErrorKind::TypeUnmatch2(req_ret_type, act_ret_type.clone()));
        }
        Ok(func)
    }

    fn check_spawn(&self, spawn: unchecked::SysDCSpawn) -> Result<SysDCSpawn, Box<dyn Error>> {
        let req_ret_type = self.def_manager.resolve_from_type(spawn.result.clone(), &self.imports)?.1;

        let spawn = spawn.convert(
            |(name, _)| self.def_manager.resolve_from_name(name.clone(), &self.imports),
            |spawn_child| self.check_spawn_child(spawn_child)
        )?;

        for spawn_child in &spawn.details {
            match spawn_child {
                SysDCSpawnChild::Return(_, act_ret_type) =>
                    if &req_ret_type != act_ret_type {
                        return CompileError::new(CompileErrorKind::TypeUnmatch2(req_ret_type, act_ret_type.clone()));
                    }
                _ => {}
            }
        }
        Ok(spawn)
    }

    fn check_spawn_child(&self, spawn_child: unchecked::SysDCSpawnChild) -> Result<SysDCSpawnChild, Box<dyn Error>> {
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
                        return CompileError::new(CompileErrorKind::TypeUnmatch2(req_arg_type.clone(), act_arg_type.clone()));
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
    Variable(Type)
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
    pub fn new(system: &unchecked::SysDCSystem) -> DefinesManager {
        DefinesManager { defines: DefinesManager::listup_defines(system) }
    }

    // 与えられたnameと同じ名前を持つ定義が存在するかどうかを確認する
    pub fn is_defined(&self, name: Name, imports: &Vec<Name>) -> Result<(), Box<dyn Error>> {
        match self.find(name.clone(), &name.name, imports)?.kind {
            DefineKind::Data | DefineKind::Module => Ok(()),
            _ => CompileError::new(CompileErrorKind::NotDefined(name.name))
        }
    }

    // 与えられたnameから参照可能なすべての範囲またはimports内を対象に，typesと一致する定義を探す (Data, Module, Function)
    // ※name, typesはともに関連している状態を想定
    pub fn resolve_from_type(&self, (name, types): (Name, Type), imports: &Vec<Name>) -> Result<(Name, Type), Box<dyn Error>> {
        match &types.kind {
            TypeKind::Int32 | TypeKind::Data => Ok((name, types)),
            TypeKind::Unsolved(hint) => {
                let (head, tails) = DefinesManager::split_name(&hint);
                let found_def = self.find(name.clone(), &head, &imports)?;
                match found_def.kind {
                    DefineKind::Data =>
                        match tails {
                            Some(_) => CompileError::new(CompileErrorKind::IllegalAccess),
                            None => Ok((name, Type::new(TypeKind::Data, Some(found_def.refs))))
                        }
                    DefineKind::Module =>
                        match tails {
                            Some(tails) => self.get_func_in_module(&found_def.refs, &tails, imports),
                            None => CompileError::new(CompileErrorKind::MissingFunctionName)
                        }
                    DefineKind::Function(_) => {
                        self.get_func_in_module(&name.get_namespace(true), &hint, imports)
                    }
                    _ => CompileError::new(CompileErrorKind::TypeUnmatch1(types))
                }
            },
            _ => panic!("Internal Error")
        }
    }

    // nameから参照可能なすべての範囲またはimports内を対象に，nameと一致する名前をもつ定義を探す (Variable)
    pub fn resolve_from_name(&self, name: Name, imports: &Vec<Name>) -> Result<(Name, Type), Box<dyn Error>> {
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
            _ => CompileError::new(CompileErrorKind::NotDefined(name.name))
        }
    }

    // 与えられた関数名に対応する関数を探し，関数に登録されている引数の型の一覧を返す
    pub fn get_args_type(&self, func_name: &Name, imports: &Vec<Name>) -> Result<Vec<Type>, Box<dyn Error>> {
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
    fn get_member_in_data(&self, data: &Name, member: &String, imports: &Vec<Name>) -> Result<(Name, Type), Box<dyn Error>> {
        let (head, tails) = DefinesManager::split_name(&member);
        for Define { kind, refs } in &self.defines {
            if let DefineKind::DataMember(types) = kind {
                if data.get_full_name() == refs.namespace && head == refs.name {
                    return match self.resolve_from_type((refs.clone(), types.clone()), imports)? {
                        (_, types@Type { kind: TypeKind::Int32, .. }) =>
                            match tails {
                                Some(_) => CompileError::new(CompileErrorKind::IllegalAccess),
                                None => Ok((refs.clone(), types))
                            }
                        (_, types@Type { kind: TypeKind::Data, .. }) =>
                            match tails {
                                Some(tails) => self.get_member_in_data(types.refs.as_ref().unwrap(), &tails, imports),
                                None => Ok((types.refs.clone().unwrap(), types))
                            },
                        _ => panic!("Internal Error")
                    }
                }
            }
        }
        CompileError::new(CompileErrorKind::MemberNotDefinedInData(member.clone(), data.name.clone()))
    }

    // module(Module)内のfunc(Function)の定義を探す
    fn get_func_in_module(&self, module: &Name, func: &String, imports: &Vec<Name>) -> Result<(Name, Type), Box<dyn Error>> {
        for Define { kind, refs } in &self.defines {
            if let DefineKind::Function(types) = kind {
                if module == &refs.get_par_name(true) && func == &refs.name {
                    return Ok((refs.clone(), self.resolve_from_type((refs.clone(), types.clone()), imports)?.1));
                }
            }
        }
        CompileError::new(CompileErrorKind::FuncNotDefinedInModule(func.clone(), module.name.clone()))
    }

    // namespace内に存在する定義を対象に，nameと同じ名前を持つ定義を探して返す
    // namespace内に存在しない場合はimports内の名前を探して返す
    // ※namespaceはルートにたどり着くまで再帰的に更新されながら検索が続く (.a.b.c -> .a.b -> .a -> .)
    fn find(&self, mut namespace: Name, name: &String, imports: &Vec<Name>) -> Result<Define, Box<dyn Error>> {
        while namespace.name.len() > 0 {
            for Define{ kind, refs } in &self.defines {
                if refs.namespace == namespace.namespace && &refs.name == name {
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

        CompileError::new(CompileErrorKind::NotFound(name.clone()))
    }

    fn split_name(hint: &String) -> (String, Option<String>) {
        let splitted_hint = hint.split(".").collect::<Vec<&str>>();
        match splitted_hint.len() {
            1 => (splitted_hint[0].to_string(), None),
            _ => (splitted_hint[0].to_string(), Some(splitted_hint[1..].join(".")))
        }
    }

    fn listup_defines(system: &unchecked::SysDCSystem) -> Vec<Define> {
        system.units
            .iter()
            .flat_map(|unit| DefinesManager::listup_defines_unit(unit))
            .collect()
    }

    fn listup_defines_unit(unit: &unchecked::SysDCUnit) -> Vec<Define> {
        let mut defined = vec!();
        defined.extend(
            unit.data
                .iter()
                .flat_map(|data| {
                    let mut d = vec!(Define::new(DefineKind::Data, data.name.clone()));
                    d.extend(DefinesManager::listup_defines_data(data));
                    d
                })
                .collect::<Vec<Define>>()
        );
        defined.extend(
            unit.modules
                .iter()
                .flat_map(|module| {
                    let mut d = vec!(Define::new(DefineKind::Module, module.name.clone()));
                    d.extend(DefinesManager::listup_defines_module(module));
                    d
                })
                .collect::<Vec<Define>>()
        );
        defined
    }

    fn listup_defines_data(data: &unchecked::SysDCData) -> Vec<Define> {
        data.members
            .iter()
            .map(|(name, types)| Define::new(DefineKind::DataMember(types.clone()), name.clone()))
            .collect::<Vec<Define>>()
    }

    fn listup_defines_module(module: &unchecked::SysDCModule) -> Vec<Define> {
        module.functions
            .iter()
            .flat_map(|func| { 
                let mut d = vec!(Define::new(DefineKind::Function(func.returns.as_ref().unwrap().1.clone()), func.name.clone()));
                d.extend(DefinesManager::listup_defines_function(func));
                d
            })
            .collect::<Vec<Define>>()
    }

    fn listup_defines_function(func: &unchecked::SysDCFunction) -> Vec<Define> {
        let mut defined = vec!();
        defined.extend(
            func.args
                .iter()
                .flat_map(|(name, types)| vec!(
                    Define::new(DefineKind::Variable(types.clone()), name.clone()),
                    Define::new(DefineKind::Argument(types.clone()), name.clone())
                ))
                .collect::<Vec<Define>>()
        );
        defined.extend(
            func.spawns
                .iter()
                .flat_map(|spawn@unchecked::SysDCSpawn { result: (name, types), .. }| {
                    let mut d = vec!(Define::new(DefineKind::Variable(types.clone()), name.clone()));
                    d.extend(DefinesManager::listup_defines_function_spawn_details(spawn));
                    d
                })
                .collect::<Vec<Define>>()
        );
        defined
    }

    fn listup_defines_function_spawn_details(spawn: &unchecked::SysDCSpawn) -> Vec<Define> {
        let unchecked::SysDCSpawn { details, .. } = spawn;
        details
            .iter()
            .flat_map(|detail| match detail {
                unchecked::SysDCSpawnChild::LetTo { name, func: (_, func), ..} => vec!(Define::new(DefineKind::Variable(func.clone()), name.clone())),
                _ => vec!()
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::super::super::compiler::Compiler;

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
                        use a.a, a.b;
                        use a.a.a, a.a.b, a.b.a, a.b.b;
                        use a.a.a.a, a.a.a.b, a.a.b.a, a.a.b.b, a.b.a.a, a.b.a.b, a.b.b.a, a.b.b.b;
                    }
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
                        use a.c;
                    }
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
    fn import_module_in_other_unit_simple() {
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

    fn check(programs: Vec<&str>) {
        let mut compiler = Compiler::new();
        for program in programs {
            compiler.add_unit(program.to_string()).unwrap();
        }
        compiler.generate_system().unwrap();
    }
}
