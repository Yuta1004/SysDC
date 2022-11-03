mod utils;
mod resolve;
mod matches;

use super::structure::unchecked;
use super::structure::SysDCSystem;
use matches::TypeMatchChecker;
use resolve::TypeResolver;
use utils::define::DefinesManager;

pub fn check(system: unchecked::SysDCSystem) -> anyhow::Result<SysDCSystem> {
    // 0. 準備
    let def_manager = DefinesManager::new(&system)?;
    let mut imports = vec![];
    for unit in &system.units {
        for import in &unit.imports {
            def_manager.check_can_import(import, &vec![])?;
            imports.push((*import).clone());
        }
    }

    // 1. 型解決
    let system = TypeResolver::resolve(system, &def_manager, &imports)?;

    // 2. 型適合チェック
    TypeMatchChecker::check(&system, &def_manager, &imports)?;

    Ok(system)
}

#[cfg(test)]
mod test {
    use crate::Parser;

    #[test]
    fn data_only_has_primitive_member() {
        let program = "
            unit test;

            data Test {
                a: i32,
                b: i32,
                c: i32
            }
        ";
        check(vec![program]);
    }

    #[test]
    fn data_has_user_defined_type_member() {
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
        check(vec![program]);
    }

    #[test]
    fn data_has_recursive_member() {
        let program = "
            unit test;

            data A {
                a: A
            }
        ";
        check(vec![program]);
    }

    #[test]
    #[should_panic]
    fn data_has_undefined_typed_member() {
        let program = "
            unit test;

            data Test {
                a: i32,
                b: Unknown
            }
        ";
        check(vec![program]);
    }

    #[test]
    fn module_simple() {
        let program = "
            unit test;

            data A {
                a: i32,
                b: i32
            }

            module TestModule {
                func test(a: A) -> A {
                    @return a
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    fn module_with_affect_1() {
        let program = "
            unit test;

            data A {}

            data B {}

            module TestModuleA {
                proc test(a: A, b: B) {}
            }

            module TestModuleB {
                proc test(a: A, b: B) {
                    @affect TestModuleA.test(a, b)
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    fn module_with_affect_2() {
        let program = "
            unit test;

            data A {}

            data B {}

            module TestModuleA {
                proc test(a: A, b: B) {
                    @affect test2(a, b)
                }

                proc test2(a: A, b: B) {}
            }
        ";
        check(vec![program]);
    }

    #[test]
    #[should_panic]
    fn module_with_affect_failure_1() {
        let program = "
            unit test;

            data A {}

            data B {}

            module TestModuleA {
                proc test(a: A, b: B) {
                    @affect test2(a)
                }

                proc test2(a: A, b: B) {}
            }
        ";
        check(vec![program]);
    }

    #[test]
    fn module_with_modify_1() {
        let program = "
            unit test;

            data A {}

            data B {}

            data C {}

            module TestModule {
                proc test(a: A, b: B, c: C) {
                    @modify a {
                        use b, c;
                    }
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    fn module_with_modify_2() {
        let program = "
            unit test;

            data A {}

            data B {}

            data C {}

            module TestModule {
                proc test(a: A, b: B, c: C) {
                    @modify a
                    @modify b
                    @modify c
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    #[should_panic]
    fn module_with_modify_failure_1() {
        let program = "
            unit test;

            data A {}

            data B {
                a: i32
            }

            module TestModule {
                proc test(a: A, b: B, c: C) {
                    @modify a {
                        use b.a;
                    }
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    #[should_panic]
    fn module_with_modify_failure_2() {
        let program = "
            unit test;

            data A {}

            data B {
                a: i32
            }

            module TestModule {
                proc test(a: A, b: B, c: C) {
                    @modify a {
                        use b, c;
                    }
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    fn module_with_spawn() {
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
                func test(a: A) -> A {
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

                func receiveA(a: A) -> i32 {
                    @return tmp
                    @spawn tmp: i32
                }

                func receiveB(b: B) -> i32 {
                    @return tmp
                    @spawn tmp: i32
                }

                func receiveC(c: C) -> i32 {
                    @return tmp
                    @spawn tmp: i32
                }

                func receiveInt32(i: i32) -> i32 {
                    @return tmp
                    @spawn tmp: i32
                }
            }
        ";
        check(vec![program])
    }

    #[test]
    #[should_panic]
    fn module_with_spawn_failure_1() {
        let program = "
            unit test;

            data A {
                a: i32,
                b: i32
            }

            module TestModule {
                func test(a: A) -> A {
                    @return b

                    @spawn b: A {
                        use aaa;
                    }
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    #[should_panic]
    fn module_with_spawn_failure_2() {
        let program = "
            unit test;

            data A {
                a: i32,
                b: i32
            }

            module TestModule {
                func test(a: A) -> A {
                    @return b

                    @spawn b: A {
                        use a;
                        let tmp = receiveInt32(a.c);
                    }
                }

                func receiveInt32(i: i32) -> i32 {
                    @return tmp
                    @spawn tmp: i32
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    #[should_panic]
    fn module_with_spawn_failure_3() {
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
                func test(a: A) -> A {
                    @return b

                    @spawn b: A {
                        use a.b.a.c;
                    }
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    fn ref_function_using_completed_name_1() {
        let program = "
            unit test;

            data A {}

            module AModule {
                func new() -> A {
                    @return a

                    @spawn a: A
                }
            }

            module TestModule {
                func test() -> A {
                    @return a

                    @spawn a: A {
                        let b = AModule.new();
                        return b;
                    }
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    fn ref_function_using_completed_name_2() {
        let program = "
            unit test;

            data A {}

            module TestModule {
                func new() -> A {
                    @return a

                    @spawn a: A
                }

                func test() -> A {
                    @return a

                    @spawn a: A {
                        let b = TestModule.new();
                        return b;
                    }
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    fn ref_function_using_uncompleted_name() {
        let program = "
            unit test;

            data A {}

            module TestModule {
                func new() -> A {
                    @return a

                    @spawn a: A
                }

                func test() -> A {
                    @return a

                    @spawn a: A {
                        let b = new();
                        return b;
                    }
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    #[should_panic]
    fn ref_function_using_completed_name_failure() {
        let program = "
            unit test;

            data A {}

            module TestModule {
                func new() -> A {
                    @return a

                    @spawn a: A
                }

                func test() -> A {
                    @return a

                    @spawn a: A {
                        let b = TestModule.new2();
                        return b;
                    }
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    #[should_panic]
    fn ref_function_using_uncompleted_name_failure() {
        let program = "
            unit test;

            data A {}

            module TestModule {
                func new() -> A {
                    @return a

                    @spawn a: A
                }

                func test() -> A {
                    @return a

                    @spawn a: A {
                        let b = new2();
                        return b;
                    }
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    fn function_argument_check_ok() {
        let program = "
            unit test;

            data A {}
            data B {}
            data C {}

            module TestModule {
                func test() -> A {
                    @return a

                    @spawn a: A {
                        let tmp1 = genA();
                        let tmp2 = genB(tmp1);
                        let tmp3 = genC(tmp1, tmp2);
                    }
                }

                func genA() -> A {
                    @return a

                    @spawn a: A
                }

                func genB(a: A) -> B {
                    @return b

                    @spawn b: B {
                        use a;
                    }
                }

                func genC(a: A, b: B) -> C {
                    @return c

                    @spawn c: C {
                        use a, b;
                    }
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    #[should_panic]
    fn function_argument_check_ng_1() {
        let program = "
            unit test;

            data A {}
            data B {}
            data C {}

            module TestModule {
                func test() -> A {
                    @return a

                    @spawn a: A {
                        let tmp1 = genA();
                        let tmp2 = genB(tmp1);
                        let tmp3 = genC(tmp1, tmp1);
                    }
                }

                func genA() -> A {
                    @return a

                    @spawn a: A
                }

                func genB(a: A) -> B {
                    @return b

                    @spawn b: B {
                        use a;
                    }
                }

                func genC(a: A, b: B) -> C {
                    @return c

                    @spawn c: C {
                        use a, b;
                    }
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    #[should_panic]
    fn function_argument_check_ng_2() {
        let program = "
            unit test;

            module TestModule {
                func test(a: i32) -> i32 {
                    @return result

                    @spawn result: i32 {
                        use a;
                        let val = test2(a);
                        return val;
                    }
                }

                func test2(a: i32, b: i32) -> i32 {
                    @return result
                    @spawn result: i32
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    fn function_return_check_ok() {
        let program = "
            unit test;

            module TestModule {
                func test() -> i32 {
                    @return a

                    @spawn a: i32
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    #[should_panic]
    fn function_return_check_ng() {
        let program = "
            unit test;

            data A {}

            module TestModule {
                func test() -> i32 {
                    @return a

                    @spawn a: A
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    fn in_spawn_return_check_ok() {
        let program = "
            unit test;

            module TestModule {
                func test() -> i32 {
                    @return a

                    @spawn a: i32 {
                        let b = gen_i32();
                        return b;
                    }
                }

                func gen_i32() -> i32 {
                    @return a

                    @spawn a: i32
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    #[should_panic]
    fn in_spawn_return_check_ng() {
        let program = "
            unit test;

            data A {}

            module TestModule {
                func test() -> i32 {
                    @return a

                    @spawn a: i32 {
                        let b = gen_A();
                        return b;
                    }
                }

                func gen_A() -> A {
                    @return a

                    @spawn a: A
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    fn procedure_simple() {
        let program = "
            unit test;

            module TestModule {
                proc test() { }
            }
        ";
        check(vec![program]);
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
        check(vec![program1, program2]);
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
                func new() -> A {
                    @return a
                    @spawn a: A
                }

                func test() -> i32 {
                    @return v

                    @spawn v: i32 {
                        let a = new();
                        return a.b.c.body;
                    }
                }
            }
        ";
        check(vec![program1, program2]);
    }

    #[test]
    fn import_module_in_other_unit() {
        let program1 = "
            unit test.A;

            module TestModule {
                func test() -> i32 {
                    @return a

                    @spawn a: i32
                }
            }
        ";
        let program2 = "
            unit test.B;

            from test.A import TestModule;

            module TestModule2 {
                func test() -> i32 {
                    @return a

                    @spawn a: i32 {
                        let a = TestModule.test();
                        return a;
                    }
                }
            }
        ";
        check(vec![program1, program2]);
    }

    #[test]
    #[should_panic]
    fn import_module_in_other_unit_failure() {
        let program1 = "
            unit test.A;

            data A {}

            module TestModule {
                func test() -> A {
                    @return a

                    @spawn a: A
                }
            }
        ";
        let program2 = "
            unit test.B;

            from test.A import A, TestModule;

            module TestModule2 {
                func test() -> i32 {
                    @return a

                    @spawn a: i32 {
                        let a = TestModule.test();
                        return a;
                    }
                }
            }
        ";
        check(vec![program1, program2]);
    }

    #[test]
    #[should_panic]
    fn multiple_define_1() {
        let program = "
            unit test;

            data A {}
            data A{}
        ";
        check(vec![program]);
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
        check(vec![program]);
    }

    #[test]
    #[should_panic]
    fn multiple_define_3() {
        let program = "
            unit test;

            module TestModule {}
            module TestModule {}
        ";
        check(vec![program]);
    }

    #[test]
    #[should_panic]
    fn multiple_define_4() {
        let program = "
            unit test;

            module TestModule {
                func a() -> i32 {
                    @return b

                    @spawn b: i32
                }

                func a() -> i32 {
                    @return c

                    @spawn c: i32
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    #[should_panic]
    fn multiple_define_5() {
        let program = "
            unit test;

            module TestModule {
                func a() -> i32 {
                    @return a

                    @spawn a: i32
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    #[should_panic]
    fn multiple_define_6() {
        let program = "
            unit test;

            module TestModule {
                func a(arg: i32) -> i32 {
                    @return val

                    @spawn val: i32 {
                        return arg;
                    }
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    #[should_panic]
    fn multiple_define_7() {
        let program = "
            unit test;

            module TestModule {
                func a(arg: i32) -> i32 {
                    @return val

                    @spawn val: i32 {
                        use arg;
                        return arg2;
                    }
                }
            }
        ";
        check(vec![program]);
    }

    #[test]
    #[should_panic]
    fn multiple_define_8() {
        let program = "
            unit test;

            module TestModule {
                func a(arg: i32) -> i32 {
                    @return val

                    @spawn val: i32 {
                        use arg;
                        let val = new();
                        return val2;
                    }
                }

                func new() -> i32 {
                    @return val

                    @spawn val: i32
                }
            }
        ";
        check(vec![program]);
    }

    fn check(programs: Vec<&str>) {
        let mut parser = Parser::default();
        for program in programs {
            parser.parse("check.def".to_string(), program).unwrap();
        }
        parser.check().unwrap();
    }
}
