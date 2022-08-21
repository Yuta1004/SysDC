use std::error::Error;

use super::name::Name;
use super::error::CompileError;
use super::types::{ Type, TypeKind };
use super::structure::{ SysDCSystem, SysDCUnit, SysDCData, SysDCModule, SysDCFunction, SysDCSpawn, SysDCSpawnChild  };
use super::structure::unchecked;

pub struct Checker {
    def_manager: DefinesManager
}

impl Checker {
    pub fn check(system: unchecked::SysDCSystem) -> Result<SysDCSystem, Box<dyn Error>> {
        let checker = Checker { def_manager: DefinesManager::new(&system) };
        system.convert(|unit| checker.check_unit(unit))
    }

    fn check_unit(&self, unit: unchecked::SysDCUnit) -> Result<SysDCUnit, Box<dyn Error>> {
        unit.convert(
            |data| self.check_data(data),
            |module| self.check_module(module)
        )
    }

    fn check_data(&self, data: unchecked::SysDCData) -> Result<SysDCData, Box<dyn Error>> {
        data.convert(|(name, types): (Name, Type)|
            match types.kind {
                TypeKind::Int32 => Ok((name, types)),
                _ => self.def_manager.resolve_from_type(name, types)
            }
        )
    }

    fn check_module(&self, module: unchecked::SysDCModule) -> Result<SysDCModule, Box<dyn Error>> {
        module.convert(|func| self.check_function(func))
    }

    fn check_function(&self, func: unchecked::SysDCFunction) -> Result<SysDCFunction, Box<dyn Error>> {
        let a_converter = |(name, types)| self.def_manager.resolve_from_type(name, types);
        let r_converter = |returns: Option<(Name, Type)>| {
            let (ret_name, ret_type) = returns.unwrap();
            let require_ret = self.def_manager.resolve_from_type(ret_name.clone(), ret_type)?;
            let ret = self.def_manager.resolve_from_name(ret_name.clone(), ret_name.name)?;
            // if require_ret.1 != ret.1 {
            //     return Err(Box::new(CompileError::TypeUnmatch2(require_ret.1, ret.1)));
            // }
            Ok(Some(ret))
        };
        func.convert(a_converter, r_converter, |spawn| self.check_spawn(spawn))
    }
    
    fn check_spawn(&self, spawn: unchecked::SysDCSpawn) -> Result<SysDCSpawn, Box<dyn Error>> {
        spawn.convert(
            |(name, _)| self.def_manager.resolve_from_name(name.clone(), name.name),
            |spawn_child| self.check_spawn_child(spawn_child)
        )
    }

    fn check_spawn_child(&self, spawn_child: unchecked::SysDCSpawnChild) -> Result<SysDCSpawnChild, Box<dyn Error>> {
        let ur_converter = |(name, _): (Name, Type)| self.def_manager.resolve_from_name(name.clone(), name.name);
        let l_converter = |name: Name, (_, func): (Name, Type), args: Vec<(Name, Type)>| {
            if let Type { kind: TypeKind::Unsolved(func), .. } = func {
                let mut let_to_args = vec!();
                for ((arg_name, _), required_type) in args.into_iter().zip(self.def_manager.get_args_type(&name, &func).unwrap().into_iter()) {
                    let (arg_name, arg_type) = self.def_manager.resolve_from_name(arg_name.clone(), arg_name.name)?;
                    // if arg_type != required_type {
                    //     return Err(Box::new(CompileError::TypeUnmatch2(required_type, arg_type)));
                    // }
                    let_to_args.push((arg_name, arg_type));
                }
                let resolved_func = self.def_manager.resolve_from_type(name.clone(), Type::from(func))?;
                return Ok((name, resolved_func, let_to_args));
            }
            panic!("")
            // Err(Box::new(CompileError::InternalError))
        };
        spawn_child.convert(ur_converter, ur_converter, l_converter)
        // let (name, types) = self.def_manager.resolve_from_name(name.clone(), name.name)?;
        // if types != required_types.1 {
        //     return Err(Box::new(CompileError::TypeUnmatch2(required_types.1, types)));
        // }
        // SysDCSpawnChild::new_return(name, types)
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

    pub fn get_args_type(&self, namespace: &Name, name: &String) -> Result<Vec<Type>, Box<dyn Error>> {
        let (head, tails) = DefinesManager::split_name(name);
        let found_def = self.find(namespace, &head)?;
        let func = match &found_def.kind {
            DefineKind::Module =>
                match tails {
                    Some(tails) => Name::from(&found_def.refs, tails),
                    None => return Err(Box::new(CompileError::MissingFunctionName))
                }
            DefineKind::Function(_) =>
                match tails {
                    Some(_) => return Err(Box::new(CompileError::IllegalAccess)),
                    None => Name::from(&namespace.get_par_name(true).get_par_name(true), head)
                }
            _ => return Err(Box::new(CompileError::NotDefined(name.to_string())))
        };
        let func_name = func.get_global_name();

        let mut args = vec!();
        for Define { kind, refs } in &self.defines {
            if let DefineKind::Argument(types) = kind {
                if &refs.namespace == &func_name {
                    args.push(self.resolve_from_type(func.clone(), types.clone())?.1);
                }
            }
        }
        Ok(args)
    }

    pub fn resolve_from_type(&self, name: Name, types: Type) -> Result<(Name, Type), Box<dyn Error>> {
        match types.kind.clone() {
            TypeKind::Int32 => Ok((name, types)),
            TypeKind::Unsolved(hint) => {
                let (head, tails) = DefinesManager::split_name(&hint);
                let found_def = self.find(&name, &head)?;
                match found_def.kind {
                    DefineKind::Data =>
                        match tails {
                            Some(_) => Err(Box::new(CompileError::IllegalAccess)),
                            None => Ok((name, Type::new(TypeKind::Data, Some(found_def.refs))))
                        }
                    DefineKind::Module =>
                        match tails {
                            Some(tails) => self.resolve_from_module_func(name, found_def.refs.name, tails),
                            None => Err(Box::new(CompileError::MissingFunctionName))
                        }
                    DefineKind::Function(_) => {
                        self.resolve_from_module_func(name.clone(), name.get_par_name(true).get_par_name(true).name, hint)
                    }
                    _ => Err(Box::new(CompileError::TypeUnmatch1(types)))
                }
            },
            _ => Err(Box::new(CompileError::InternalError))
        }
    }

    pub fn resolve_from_name(&self, name: Name, nname: String) -> Result<(Name, Type), Box<dyn Error>> {
        let (head, tails) = DefinesManager::split_name(&nname);
        let found_def = self.find(&name, &head)?;
        match found_def.kind {
            DefineKind::Variable(types) => {
                let types = self.resolve_from_type(name.clone(), types)?.1;
                match tails {
                    Some(tails) => self.resolve_from_data_member(name, types, tails),
                    None => Ok((found_def.refs, types))
                }
            }
            _ => Err(Box::new(CompileError::NotDefined(nname)))
        }
    }

    fn resolve_from_data_member(&self, name: Name, data: Type, member: String) -> Result<(Name, Type), Box<dyn Error>> {
        let (head, tails) = DefinesManager::split_name(&member);
        for Define { kind, refs } in &self.defines {
            if let DefineKind::DataMember(types) = kind {
                if data.refs.as_ref().unwrap().name == refs.get_par_name(true).name && head == refs.name {
                    return match self.resolve_from_type(name.clone(), types.clone())?.1 {
                        types@Type { kind: TypeKind::Int32, .. } =>
                            match tails {
                                Some(_) => Err(Box::new(CompileError::IllegalAccess)),
                                None => Ok((refs.clone(), types))
                            }
                        types@Type { kind: TypeKind::Data, .. } =>
                            match tails {
                                Some(tails) => self.resolve_from_data_member(name, types, tails),
                                None => Ok((refs.clone(), types))
                            },
                        _ => Err(Box::new(CompileError::InternalError))
                    }
                }
            }
        }
        Err(Box::new(CompileError::MemberNotDefinedInData(member, data.refs.unwrap().name)))
    }

    fn resolve_from_module_func(&self, name: Name, module: String, func: String) -> Result<(Name, Type), Box<dyn Error>> {
        for Define { kind, refs } in &self.defines {
            if let DefineKind::Function(types) = kind {
                if module == refs.get_par_name(true).name && func == refs.name {
                    return Ok((refs.clone(), self.resolve_from_type(name, types.clone())?.1));
                }
            }
        }
        Err(Box::new(CompileError::FuncNotDefinedInModule(func, module)))
    }

    fn find(&self, namespace: &Name, name: &String) -> Result<Define, Box<dyn Error>> {
        if namespace.namespace.len() == 0 {
            return Err(Box::new(CompileError::NotFound(name.to_string())));
        }
        for Define{ kind, refs } in &self.defines {
            if refs.namespace == namespace.namespace && &refs.name == name {
                return Ok(Define::new(kind.clone(), refs.clone()))
            }
        }
        self.find(&namespace.get_par_name(false), name)
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
            data Test {
                a: i32,
                b: i32,
                c: i32
            }
        ";
        check(program);
    }

    #[test]
    fn user_defined_type_mix_data() {
        let program = "
            data A {
                a: i32
            }

            data Test {
                a: A,
                b: i32,
                c: A
            }
        ";
        check(program);
    }

    #[test]
    fn recursive_type_mix_data() {
        let program = "
            data A {
                a: A
            }
        ";
        check(program);
    }

    #[test]
    #[should_panic]
    fn undefined_type_mix_data() {
        let program = "
            data Test {
                a: i32,
                b: Unknown
            }
        ";
        check(program);
    }

    #[test]
    fn user_defind_type_mix_module() {
        let program = "
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
        check(program);
    }

    #[test]
    fn user_defined_type_mix_module_with_spawn() {
        let program = "
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
        check(program)
    }

    #[test]
    #[should_panic]
    fn user_defind_type_mix_module_with_spawn_failure_1() {
        let program = "
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
        check(program);
    }

    #[test]
    #[should_panic]
    fn user_defind_type_mix_module_with_spawn_failure_2() {
        let program = "
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
        check(program);
    }

    #[test]
    #[should_panic]
    fn user_defind_type_mix_module_with_spawn_failure_3() {
        let program = "
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
        check(program);
    }

    #[test]
    fn let_by_user_defined_function_using_completed_name_1() {
        let program = "
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
        check(program);
    }

    #[test]
    fn let_by_user_defined_function_using_completed_name_2() {
        let program = "
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
        check(program);
    }

    #[test]
    fn let_by_user_defined_function_using_uncompleted_name() {
        let program = "
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
        check(program);
    }

    #[test]
    #[should_panic]
    fn let_by_user_defined_function_using_completed_name_failure() {
        let program = "
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
        check(program);
    }

    #[test]
    #[should_panic]
    fn let_by_user_defined_function_using_uncompleted_name_failure() {
        let program = "
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
        check(program);
    }

    #[test]
    fn argument_check_ok() {
        let program = "
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
        check(program);
    }

    #[test]
    #[should_panic]
    fn argument_check_ng() {
        let program = "
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
        check(program);
    }

    #[test]
    fn return_check_ok() {
        let program = "
            module TestModule {
                test() -> i32 {
                    @return a

                    @spawn a: i32
                }
            }
        ";
        check(program);
    }

    #[test]
    #[should_panic]
    fn return_check_ng() {
        let program = "
            data A {}

            module TestModule {
                test() -> i32 {
                    @return a

                    @spawn a: A
                }
            }
        ";
        check(program);
    }

    #[test]
    fn in_spawn_return_check_ok() {
        let program = "
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
        check(program);
    }

    #[test]
    #[should_panic]
    fn in_spawn_return_check_ng() {
        let program = "
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
        check(program);
    }

    fn check(program: &str) {
        let mut compiler = Compiler::new();
        compiler.add_unit("test".to_string(), program.to_string()).unwrap();
        compiler.generate_system().unwrap();
    }
}
