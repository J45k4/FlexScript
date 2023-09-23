#[cfg(test)]
mod tests {
    use crate::Obj;
    use crate::ObjProp;
    use crate::Property;
    use crate::RunResult;
    use crate::Value;
    use crate::Vm;

    #[test]
    fn test_return_number() {
        let mut vm = Vm::new();
        let res = vm.run_code("return 1");
        assert_eq!(res, RunResult::Value(Value::Int(1)));
    }

    #[test]
    fn simple_plus() {
        let mut vm = Vm::new();
        let res = vm.run_code("return 1 + 1");
        assert_eq!(res, RunResult::Value(Value::Int(2)));
    }

    #[test]
    fn simple_sub() {
        let mut vm = Vm::new();
        let res = vm.run_code("return 1 - 1");
        assert_eq!(res, RunResult::Value(Value::Int(0)));
    }

    #[test]
    fn add_sub() {
        let mut vm = Vm::new();
        let res = vm.run_code("return 1 + 1 - 1");
        assert_eq!(res, RunResult::Value(Value::Int(1)));
    }

    #[test]
    fn simple_comparsion() {
        let mut vm = Vm::new();
        let res = vm.run_code("return 1 == 1");
        assert_eq!(res, RunResult::Value(Value::Bool(true)));
    }

    #[test]
    fn simple_if_true() {
        let mut vm = Vm::new();
        let res = vm.run_code("if true { return 1 }");
        assert_eq!(res, RunResult::Value(Value::Int(1)));
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
        assert_eq!(res, RunResult::Value(Value::Int(1)));
    }

    #[test]
    fn simple_array() {
        let mut vm = Vm::new();
        let res = vm.run_code("return [1,2,3]");

        match res {
            RunResult::Value(value) => {
                assert_eq!(value, Value::List(vec![
                    Value::Int(1),
                    Value::Int(2),
                    Value::Int(3)
                ]));
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
        assert_eq!(res, RunResult::Value(Value::Int(2)));
    }

    #[test]
    fn function_call_with_args() {
        let mut vm = Vm::new();
        let res = vm.run_code(r#"
        a = (a) => return a
        return a(1)
        "#);
        assert_eq!(res, RunResult::Value(Value::Int(1)));
    }

    #[test]
    fn function_call_move_return_value_to_different_scope() {
        let mut vm = Vm::new();
        let res = vm.run_code(r#"
        a = () => {
            person = Person {
                name: "Seppo"
            }

            return person
        }
        new_person = a()
        return new_person
        "#);
        let expected = RunResult::Value(
            Value::Obj(
                Obj { 
                    name: Some("Person".to_string()), 
                    props: vec![
                        ObjProp {
                            name: "name".to_string(),
                            value: Value::Str("Seppo".to_string())
                        }
                    ]
                }
            )
        );
        assert_eq!(res, expected);
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
        assert_eq!(res, RunResult::Value(Value::Int(-6)));
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
                assert_eq!(res, RunResult::Value(Value::Int(1)));
            },
            _ => panic!("Invalid result")
        }
    }

    #[test]
    fn return_obj_instance() {
        let mut vm = Vm::new();
        let res = vm.run_code(r#"return H1 { text: "lol" }"#);

        match res {
            RunResult::Value(value) => {
                assert_eq!(value, Value::Obj(
                    Obj {
                        name: Some("H1".to_string()),
                        props: vec![
                            ObjProp {
                                name: "text".to_string(),
                                value: Value::Str("lol".to_string())
                            }
                        ]
                    }
                ));
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
            RunResult::Value(value) => {
                assert_eq!(value, Value::List(
                    vec![
                        Value::Int(1),
                        Value::Int(2),
                        Value::Int(3),
                        Value::Int(4)
                    ]
                ));
            },
            _ => panic!("Invalid result")
        }
    }

    #[test]
    fn pop_from_list() {
        let mut vm = Vm::new();
        let res = vm.run_code(r#"
        a = [1,2,3]
        a.pop()
        return a
        "#);

        match res {
            RunResult::Value(value) => {
                assert_eq!(value, Value::List(
                    vec![
                        Value::Int(1),
                        Value::Int(2),
                    ]
                ));
            },
            _ => panic!("Invalid result")
        }
    }

    #[test]
    fn map_list() {
        let mut vm = Vm::new();
        let res = vm.run_code(r#"return [1,2].map(p => return p * 2)"#);
        match res {
            RunResult::Value(value) => {
                assert_eq!(value, Value::List( 
                    vec![
                        Value::Int(2),
                        Value::Int(4),
                    ]
                ));
            },
            _ => panic!("Invalid result")
        }
    }

    #[test]
    fn nested_obj() {
        let mut vm = Vm::new();

        let res = vm.run_code(r#"
        return Html {
            head: Head {
                title: "hello"
            },
            body: []
        }"#);

        
        match res {
            RunResult::Value(value) => {
                let expected = Value::Obj(
                    Obj {
                        name: Some("Html".to_string()),
                        props: vec![
                            ObjProp {
                                name: "body".to_string(),
                                value: Value::List(vec![])
                            },
                            ObjProp {
                                name: "head".to_string(),
                                value: Value::Obj(
                                    Obj {
                                        name: Some("Head".to_string()),
                                        props: vec![
                                            ObjProp {
                                                name: "title".to_string(),
                                                value: Value::Str("hello".to_string())
                                            }
                                        ]
                                    }
                                )
                            }
                        ]
                    }
                );

                assert_eq!(value, expected);
            },
            _ => panic!("Invalid result")
        }
    }

    #[test]
    fn mapping_list_to_obj_prop() {
        let mut vm = Vm::new();
        vm.log = 2;

        let res = vm.run_code(r#"
        return Obj {
            numbers: [1, 2, 3].map((p) => return p * 2)
        }"#);

        match res {
            RunResult::Value(value) => {
                let expected = Value::Obj(
                    Obj {
                        name: Some("Obj".to_string()),
                        props: vec![
                            ObjProp {
                                name: "numbers".to_string(),
                                value: Value::List(vec![
                                    Value::Int(2),
                                    Value::Int(4),
                                    Value::Int(6),
                                ])
                            }
                        ]
                    }
                );

                assert_eq!(value, expected);
            },
            _ => panic!("Invalid result")
        }
    }

    #[test]
    fn cloning_obj_with_prop_with_array_of_objects() {
        let mut vm = Vm::new();

        let res = vm.run_code(r#"
        return Obj {
            persons: [Person { name: "test" }]
        }"#);

        match res {
            RunResult::Value(value) => {
                let expected = Value::Obj(
                    Obj {
                        name: Some("Obj".to_string()),
                        props: vec![
                            ObjProp {
                                name: "persons".to_string(),
                                value: Value::List(vec![
                                    Value::Obj(
                                        Obj {
                                            name: Some("Person".to_string()),
                                            props: vec![
                                                ObjProp {
                                                    name: "name".to_string(),
                                                    value: Value::Str("test".to_string())
                                                }
                                            ]
                                        }
                                    )
                                ])
                            }
                        ]
                    }
                );

                assert_eq!(value, expected);
            },
            _ => panic!("Invalid result")
        }
    }

    #[test]
    fn does_not_work() {
        let mut vm = Vm::new();
        vm.log = 2;

        let res = vm.run_code(r#"
        return Html {
            head: Head {
                title: "hello"
            },
            body: [1, 2, 3].map((p) => {
                return H1 {
                    text: p
                }
            })
        }"#);

        println!("{:?}", res);
        // println!("{:?}", vm.scope);

        match res {
            RunResult::Value(value) => {
                println!("{:#?}", value);
            },
            _ => todo!()
        }
    }
}