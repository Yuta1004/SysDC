use super::name::Name;
use super::types::{ Type, TypeKind };
use super::structure::{ SysDCSystem, SysDCUnit, SysDCData, SysDCModule, SysDCFunction, SysDCSpawn, SysDCSpawnChild  };

pub struct Checker {
    def_manager: DefinesManager
}

impl Checker {
    pub fn check(system: SysDCSystem) -> SysDCSystem {
        let checker = Checker { def_manager: DefinesManager::new(&system) };
        SysDCSystem::new(
            system.units
                .into_iter()
                .map(|unit| checker.check_unit(unit))
                .collect()
        )
    }

    fn check_unit(&self, unit: SysDCUnit) -> SysDCUnit {
        SysDCUnit::new(
            unit.name,
            unit.data
                .into_iter()
                .map(|data| self.check_data(data))
                .collect(),
            unit.modules
                .into_iter()
                .map(|module| self.check_module(module))
                .collect()
        )
    }

    fn check_data(&self, data: SysDCData) -> SysDCData {
        SysDCData::new(
            data.name,
            data.member
                .into_iter()
                .map(|(name, types)| match types.kind {
                    TypeKind::Int32 => (name, types),
                    _ => self.def_manager.resolve_from_type(&name, types)
                })
                .collect()
        )
    }

    fn check_module(&self, module: SysDCModule) -> SysDCModule {
        SysDCModule::new(
            module.name,
            module.functions
                .into_iter()
                .map(|func| self.check_function(func))
                .collect()
        )
    }

    fn check_function(&self, func: SysDCFunction) -> SysDCFunction {
        let args = func.args
            .into_iter()
            .map(|(name, types)| self.def_manager.resolve_from_type(&name, types))
            .collect::<Vec<(Name, Type)>>();

        let mut spawns = vec!();
        for SysDCSpawn { result: (name, types), detail } in func.spawns {
            let mut details = vec!();
            for uses in detail {
                match uses {
                    SysDCSpawnChild::Use{ name, .. } => {
                        let (name, types) = self.def_manager.resolve_from_name(&name, &name.name);
                        details.push(SysDCSpawnChild::new_use(name, types));
                    }
                    SysDCSpawnChild::Return { name, .. } => {
                        let (name, types) = self.def_manager.resolve_from_name(&name, &name.name);
                        details.push(SysDCSpawnChild::new_return(name, types));
                    }
                    SysDCSpawnChild::LetTo { name, func: (_, Type { kind: TypeKind::Unsolved(func), .. }), args } => {
                        let mut let_to_args = vec!();
                        for ((arg_name, _), defined_type) in args.iter().zip(self.def_manager.get_args_type(&name, &func).iter()) {
                            let (arg_name, arg_type) = self.def_manager.resolve_from_name(arg_name, &arg_name.name);
                            if &arg_type != defined_type {
                                panic!("[ERROR] Argument \"{:?}\"'s type is expected \"{:?}\", but \"{:?}\"", arg_name, defined_type, arg_type);
                            }
                            let_to_args.push((arg_name.clone(), arg_type));
                        }
                        let resolved_func = self.def_manager.resolve_from_type(&name, Type::from(func));
                        details.push(SysDCSpawnChild::new_let_to(name, resolved_func, let_to_args))
                    },
                    _ => panic!("[ERROR] Occur unknown error at Checker::check_function")
                }
            }
            spawns.push(SysDCSpawn::new(self.def_manager.resolve_from_type(&name, types), details))
        }

        let (ret_name, ret_type) = func.returns.unwrap();
        let resolved_ret = self.def_manager.resolve_from_type(&ret_name, ret_type);

        SysDCFunction::new(func.name, args, resolved_ret, spawns)
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
    pub fn new(system: &SysDCSystem) -> DefinesManager {
        DefinesManager { defines: DefinesManager::listup_defines(system) }
    }

    pub fn get_args_type(&self, namespace: &Name, name: &String) -> Vec<Type> {
        let (head, tails) = DefinesManager::split_name(name);
        let found_def = self.find(namespace, &head);
        let func = match found_def.kind {
            DefineKind::Module =>
                match tails {
                    Some(tails) => Name::from(&found_def.refs, tails),
                    None => panic!("[ERROR] Missing function name")
                }
            DefineKind::Function(_) =>
                match tails {
                    Some(_) => panic!("[ERROR] Cannot nested access to Function"),
                    None => Name::from(&namespace.get_par_name(true).get_par_name(true), head)
                }
            _ => panic!("[ERROR] Function \"{}\" is not defined", name)
        };

        let mut args = vec!();
        for Define { kind, refs } in &self.defines {
            if let DefineKind::Argument(types) = kind {
                if refs.namespace == func.get_global_name() {
                    args.push(self.resolve_from_type(&func, types.clone()).1);
                }
            }
        }
        args
    }

    pub fn resolve_from_type(&self, name: &Name, types: Type) -> (Name, Type) {
        match &types.kind {
            TypeKind::Int32 => (name.clone(), types),
            TypeKind::Unsolved(hint) => {
                let (head, tails) = DefinesManager::split_name(hint);
                let found_def = self.find(name, &head);
                match found_def.kind {
                    DefineKind::Data =>
                        match tails {
                            Some(_) => panic!("[ERROR] Cannot nested access to Data"),
                            None => (name.clone(), Type::new(TypeKind::Data, Some(found_def.refs)))
                        }
                    DefineKind::Module =>
                        match tails {
                            Some(tails) => self.resolve_from_module_func(name, &found_def.refs.name, &tails),
                            None => panic!("[ERROR] Missing function name")
                        }
                    DefineKind::Function(_) => {
                        self.resolve_from_module_func(name, &name.get_par_name(true).get_par_name(true).name, hint)
                    }
                    _ => panic!("[ERROR] \"{:?}\" is defined but type is unmatched", types)
                }
            },
            _ => panic!("[ERROR] Called unmatch resolve function (from_type)")
        }
    }

    pub fn resolve_from_name(&self, name: &Name, nname: &String) -> (Name, Type) {
        let (head, tails) = DefinesManager::split_name(nname);
        let found_def = self.find(name, &head);
        match found_def.kind {
            DefineKind::Variable(types) => {
                let types = self.resolve_from_type(name, types).1;
                match tails {
                    Some(tails) => self.resolve_from_data_member(name, &types, &tails),
                    None => (found_def.refs.clone(), types)
                }
            }
            _ => panic!("[ERROR] Variable \"{}\" is not defined", nname)
        }
    }

    fn resolve_from_data_member(&self, name: &Name, data: &Type, member: &String) -> (Name, Type) {
        let (head, tails) = DefinesManager::split_name(member);
        for Define { kind, refs } in &self.defines {
            if let DefineKind::DataMember(types) = kind {
                if data.refs.as_ref().unwrap().name == refs.get_par_name(true).name && head == refs.name {
                    return match self.resolve_from_type(name, types.clone()).1 {
                        types@Type { kind: TypeKind::Int32, .. } =>
                            match tails {
                                Some(_) => panic!("[ERROR] Cannot access Int32"),
                                None => (refs.clone(), types)
                            }
                        types@Type { kind: TypeKind::Data, .. } =>
                            match tails {
                                Some(tails) => self.resolve_from_data_member(name, &types, &tails),
                                None => (refs.clone(), types)
                            },
                        _ => panic!("[ERROR] Occur unknown error at DefinesManager::resolve_from_data_member")
                    }
                }
            }
        }
        panic!("[ERROR] Member \"{}\" is not defined in Data \"{}\"", member, data.refs.as_ref().unwrap().name);
    }

    fn resolve_from_module_func(&self, namespace: &Name, module: &String, func: &String) -> (Name, Type) {
        let (head, tails) = DefinesManager::split_name(func);
        if tails.is_some() {
            panic!("[ERROR] Cannot access Function \"{}\" to Function \"{}\"", head, tails.unwrap());
        }

        for Define { kind, refs } in &self.defines {
            if let DefineKind::Function(types) = kind {
                if module == &refs.get_par_name(true).name && head == refs.name {
                    return (refs.clone(), self.resolve_from_type(namespace, types.clone()).1);
                }
            }
        }
        panic!("[ERROR] Function \"{}\" is not defined in Module \"{}\"", func, module);
    }

    fn find(&self, namespace: &Name, name: &String) -> Define {
        if namespace.namespace.len() == 0 {
            panic!("[ERROR] Cannot find the name \"{}\"", name);
        }
        for Define{ kind, refs } in &self.defines {
            if refs.namespace == namespace.namespace && &refs.name == name {
                return Define::new(kind.clone(), refs.clone())
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

    fn listup_defines(system: &SysDCSystem) -> Vec<Define> {
        system.units
            .iter()
            .flat_map(|unit| DefinesManager::listup_defines_unit(unit))
            .collect()
    }

    fn listup_defines_unit(unit: &SysDCUnit) -> Vec<Define> {
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

    fn listup_defines_data(data: &SysDCData) -> Vec<Define> {
        data.member
            .iter()
            .map(|(name, types)| Define::new(DefineKind::DataMember(types.clone()), name.clone()))
            .collect::<Vec<Define>>()
    }

    fn listup_defines_module(module: &SysDCModule) -> Vec<Define> {
        module.functions
            .iter()
            .flat_map(|func| { 
                let mut d = vec!(Define::new(DefineKind::Function(func.returns.as_ref().unwrap().1.clone()), func.name.clone()));
                d.extend(DefinesManager::listup_defines_function(func));
                d
            })
            .collect::<Vec<Define>>()
    }

    fn listup_defines_function(func: &SysDCFunction) -> Vec<Define> {
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
                .flat_map(|spawn@SysDCSpawn { result: (name, types), .. }| {
                    let mut d = vec!(Define::new(DefineKind::Variable(types.clone()), name.clone()));
                    d.extend(DefinesManager::listup_defines_function_spawn_details(spawn));
                    d
                })
                .collect::<Vec<Define>>()
        );
        defined
    }

    fn listup_defines_function_spawn_details(spawn: &SysDCSpawn) -> Vec<Define> {
        let SysDCSpawn { detail: details, .. } = spawn;
        details
            .iter()
            .flat_map(|detail| match detail {
                SysDCSpawnChild::LetTo { name, func: (_, func), ..} => vec!(Define::new(DefineKind::Variable(func.clone()), name.clone())),
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

    fn check(program: &str) {
        let mut compiler = Compiler::new();
        compiler.add_unit("test".to_string(), &program.to_string());
        compiler.generate_system();
    }
}