#[cfg(test)]
mod tests {
    use crate::Obj;
    use crate::ObjProp;
    use crate::RunResult;
    use crate::Value;
    use crate::Vm;

    #[test]
    fn test_return_number() {
        let mut vm = Vm::new();
        let res = vm.run_code("return 1");
        assert_eq!(res, RunResult::Value { value: Value::Int(1), scope_id: 0 });
    }

    #[test]
    fn simple_plus() {
        let mut vm = Vm::new();
        let res = vm.run_code("return 1 + 1");
        assert_eq!(res, RunResult::Value { value: Value::Int(2), scope_id: 0});
    }

    #[test]
    fn simple_sub() {
        let mut vm = Vm::new();
        let res = vm.run_code("return 1 - 1");
        assert_eq!(res, RunResult::Value { value: Value::Int(0), scope_id: 0});
    }

    #[test]
    fn add_sub() {
        let mut vm = Vm::new();
        let res = vm.run_code("return 1 + 1 - 1");
        assert_eq!(res, RunResult::Value { value: Value::Int(1), scope_id: 0});
    }

    #[test]
    fn simple_comparsion() {
        let mut vm = Vm::new();
        let res = vm.run_code("return 1 == 1");
        assert_eq!(res, RunResult::Value { value: Value::Bool(true), scope_id: 0});
    }

    #[test]
    fn simple_if_true() {
        let mut vm = Vm::new();
        let res = vm.run_code("if true { return 1 }");
        assert_eq!(res, RunResult::Value { value: Value::Int(1), scope_id: 0 });
    }

    #[test]
    fn simple_if_false() {
        let mut vm = Vm::new();
        let res = vm.run_code("if false { return 1 }");
        assert_eq!(res, RunResult::None);
    }

    #[test]
    fn assign_to_var() {
        let mut vm = Vm::new();
        let res = vm.run_code(r#"
        a = 1
        return a
        "#);
        assert_eq!(res, RunResult::Value { value: Value::Int(1), scope_id: 0 });
    }

    #[test]
    fn simple_array() {
        let mut vm = Vm::new();
        let res = vm.run_code("return [1,2,3]");

        match res {
            RunResult::Value { value, .. } => {
                let arr = match value {
                    Value::Ptr(p) => match vm.get_val(0, p) {
                        Some(Value::List(arr)) => arr,
                        _ => panic!("Invalid result")
                    },
                    _ => panic!("Invalid result")
                };
                assert_eq!(arr, &mut vec![
                    Value::Int(1),
                    Value::Int(2),
                    Value::Int(3)
                ]);
            },
            _ => panic!("Invalid result")
        }
    }

    #[test]
    fn function_calling() {
        let mut vm = Vm::new();
        vm.log = 2;
        let res = vm.run_code(r#"
        a = () => return 1
        b = a()
        b = b + 1
        return b
        "#);
        assert_eq!(res, RunResult::Value { value: Value::Int(2), scope_id: 0 });
    }

    #[test]
    fn function_call_with_args() {
        let mut vm = Vm::new();
        let res = vm.run_code(r#"
        a = (a) => return a
        return a(1)
        "#);
        assert_eq!(res, RunResult::Value { value: Value::Int(1), scope_id: 1 });
    }

    #[test]
    fn simple_for() {
        let mut vm = Vm::new();
        let res = vm.run_code(r#"
        state = 0
        for a in [1,2,3] {
            state = state - a
        }
        return state
        "#);
        assert_eq!(res, RunResult::Value { value: Value::Int(-6), scope_id: 0 });
    }

    #[test]
    fn await_fun() {
        let mut vm = Vm::new();
        let res = vm.run_code("await(test())");

        assert_eq!(res, RunResult::Await {
            stack_id: 0,
            value: Value::UndefCall { 
                ident: 30, 
                args: vec![] 
            }
        });
    }

    #[test]
    fn await_fun_return_result() {
        let mut vm = Vm::new();
        let res = vm.run_code(r#"return await(test())"#);

        match res {
            RunResult::Await { stack_id, value } => {
                let res = vm.cont(stack_id, Value::Int(1));
                assert_eq!(res, RunResult::Value { value: Value::Int(1), scope_id: 0 });
            },
            _ => panic!("Invalid result")
        }
    }

    #[test]
    fn return_obj_instance() {
        let mut vm = Vm::new();
        let res = vm.run_code(r#"return H1 { text: "lol" }"#);

        match res {
            RunResult::Value { value, .. } => {
                let obj = match value {
                    Value::Ptr(p) => match vm.get_val(0, p) {
                        Some(Value::Obj(obj)) => obj,
                        _ => panic!("Invalid result")
                    },
                    _ => panic!("Invalid result")
                };
                assert_eq!(obj, &mut Obj {
                    name: Some("H1".to_string()),
                    props: vec![
                        ObjProp {
                            name: "text".to_string(),
                            value: Value::Str("lol".to_string())
                        }
                    ]
                });
            },
            _ => panic!("Invalid result")
        }
    }

    #[test]
    fn push_to_list() {
        let mut vm = Vm::new();
        vm.log = 1;
        let res = vm.run_code(r#"
        a = [1,2,3]
        a.push(4)
        return a
        "#);

        match res {
            RunResult::Value { value, .. } => {
                let arr = match value {
                    Value::Ptr(p) => match vm.get_val(0, p) {
                        Some(Value::List(arr)) => arr,
                        _ => panic!("Invalid result")
                    },
                    _ => panic!("Invalid result")
                };
                assert_eq!(arr, &mut vec![
                    Value::Int(1),
                    Value::Int(2),
                    Value::Int(3),
                    Value::Int(4)
                ]);
            },
            _ => panic!("Invalid result")
        }
    }

    #[test]
    fn map_list() {
        let mut vm = Vm::new();
        let res = vm.run_code(r#"return [1,2].map(p => return p * 2)"#);
        match res {
            RunResult::Value { value, .. } => {
                let arr = match value {
                    Value::Ptr(p) => match vm.get_val(0, p) {
                        Some(Value::List(arr)) => arr,
                        _ => panic!("Invalid result")
                    },
                    _ => panic!("Invalid result")
                };
                assert_eq!(arr, &mut vec![
                    Value::Int(2),
                    Value::Int(4),
                ]);
            },
            _ => panic!("Invalid result")
        }
    }
}