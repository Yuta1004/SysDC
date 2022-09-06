mod utils;

use super::name::Name;
use super::types::{ Type, TypeKind };
use super::error::{ PResult, PErrorKind };
use super::structure::{ SysDCSystem, SysDCUnit, SysDCData, SysDCModule, SysDCFunction, SysDCSpawn, SysDCSpawnChild  };
use super::structure::unchecked;
use utils::define::DefinesManager;

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
            return PErrorKind::TypeUnmatch2(req_ret_type, act_ret_type.clone()).to_err();
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
                        return PErrorKind::TypeUnmatch2(req_ret_type, act_ret_type.clone()).to_err();
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
                        return PErrorKind::TypeUnmatch2(req_arg_type.clone(), act_arg_type.clone()).to_err();
                    }
                }
            }
            _ => {}
        }
        Ok(spawn_child)
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
