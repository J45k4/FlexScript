
#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{Expr, parse_raw_ast, parse_file, CodeFile, Const, Assign, StructField, Struct, VarType, NonNullType, If, IfBranch, ConstStmt, ObjExpr, ObjField, TypeStmt, TypeField, MatchCase, MatchExpr, Func, Param, ForExpr, Call, RangeExpr, PropAccess, Array, Xml};
    use crate::BinOP;
    use crate::BodyItem;
    use crate::Operator;

    #[test]
    fn test_parse_string_literal() {
        let code = r#""qwerty""#;

        let ast = parse_file(code).unwrap();

        let expected = CodeFile {
            body: vec![
                BodyItem::Expr(
                    Expr::Const(
                        Const::String(
                            "qwerty".to_string()
                        )
                    )
                )
            ]
        };

        assert_eq!(ast, expected);
    }

    #[test]
    fn test_float_literal() {
        let code = r#"5.5"#;

        let ast = parse_file(code).unwrap();

        let expected = CodeFile {
            body: vec![
                BodyItem::Expr(
                    Expr::Const(
                        Const::Float(
                            5.5
                        )
                    )
                )
            ]
        };

        assert_eq!(ast, expected);
    }

    #[test]
    fn test_int_literal() {
        let code = r#"5"#;

        let ast = parse_file(code).unwrap();

        let expected = CodeFile {
            body: vec![
                BodyItem::Expr(
                    Expr::Const(
                        Const::Int(
                            5
                        )
                    )
                )
            ]
        };

        assert_eq!(ast, expected);
    }

    #[test]
    fn test_parse_simple_equation() {
        let code = r#"5 + 5 + 5 * 2"#;

        let ast = parse_file(code).unwrap();

        let expected = CodeFile {
            body: vec![
                BodyItem::Expr(
                    Expr::BinOP(
                        BinOP {
                            left: Box::new(
                                Expr::Const(
                                    Const::Int(5)
                                ),
                            ),
                            op: Operator::Add,
                            right: Box::new( 
                                Expr::BinOP(
                                    BinOP {
                                        left: Box::new(
                                            Expr::Const(
                                                Const::Int(5)
                                            )
                                        ),
                                        op: Operator::Add,
                                        right: Box::new(
                                            Expr::BinOP(
                                                BinOP {
                                                    left: Box::new( 
                                                        Expr::Const(
                                                            Const::Int(5)
                                                        )
                                                    ),
                                                    op: Operator::Mul,
                                                    right: Box::new(
                                                        Expr::Const(
                                                            Const::Int(2)
                                                        )
                                                    ) 
                                                }
                                            )
                                        )
                                    }
                                )
                            )
                        },
                    )
                )
            ]
        };

        assert_eq!(ast, expected)
    }

    #[test]
    fn test_assign() {
        let code = r#"a = 5"#;

        let ast = parse_file(code).unwrap();

        assert_eq!(ast, CodeFile {
            body: vec![
                BodyItem::Assign(
                    Assign {
                        target: Box::new(
                            Expr::Ident(
                                "a".to_string()
                            )
                        ),
                        value: Box::new(
                            Expr::Const(
                                Const::Int(5)
                            )
                        )
                    }
                )
            ]
        })
    }

    #[test]
    fn test_parse_struct() {
        let code = r#"
            struct Foo {
                a int
                b int
            }
        "#;

        let ast = parse_file(code).unwrap();

        assert_eq!(ast, CodeFile {
            body: vec![
                BodyItem::Struct(
                    Struct {
                        name: "Foo".to_string(),
                        fields: vec![
                            StructField {
                                name: "a".to_string(),
                                typ: VarType {
                                    typ: NonNullType::Int,
                                    array: false,
                                    nullable: false
                                }
                            },
                            StructField {
                                name: "b".to_string(),
                                typ: VarType {
                                    typ: NonNullType::Int,
                                    array: false,
                                    nullable: false
                                }
                            }
                        ]
                    }
                )
            ]
        })
    }

    #[test]
    fn test_parse_if() {
        let code = r#"
        if 10 != 5 {

        } else if 10 == 5 {

        } else {
            
        }
        "#;

        let ast = parse_file(code).unwrap();

        let expected = CodeFile {
            body: vec![
                BodyItem::Expr(
                    Expr::If(
                        If {
                            branches: vec![
                                IfBranch {
                                    condition: Expr::BinOP(
                                        BinOP {
                                            left: Box::new(
                                                Expr::Const(
                                                    Const::Int(10)
                                                )
                                            ),
                                            op: Operator::Neq,
                                            right: Box::new(
                                                Expr::Const(
                                                    Const::Int(5)
                                                )
                                            )
                                        }
                                    ),
                                    body: vec![]
                                },
                                IfBranch {
                                    condition: Expr::BinOP(
                                        BinOP {
                                            left: Box::new(
                                                Expr::Const(
                                                    Const::Int(10)
                                                )
                                            ),
                                            op: Operator::Eq,
                                            right: Box::new(
                                                Expr::Const(
                                                    Const::Int(5)
                                                )
                                            )
                                        }
                                    ),
                                    body: vec![]
                                }
                            ],
                            else_body: vec![]
                        }
                    )
                )
            ],
        };        assert_eq!(ast, expected)
    }

    #[test]
    fn test_const_obj() {
        let code = r#"
            const person = {
                name: "John"
                age: 20
            }
        "#;

        let ast = parse_file(code).unwrap();

        let expected = CodeFile {
            body: vec![
                BodyItem::Const(
                    ConstStmt {
                        ident: "person".to_string(),
                        value: Expr::ObjExpr(
                            ObjExpr {
                                fields: vec![
                                    ObjField {
                                        target: "name".to_string(),
                                        value: Expr::Const(
                                            Const::String("John".to_string())
                                        )
                                    },
                                    ObjField {
                                        target: "age".to_string(),
                                        value: Expr::Const(
                                            Const::Int(20)
                                        )
                                    }
                                ]
                            }
                        )
                    }
                )
            ]
        };

        assert_eq!(ast, expected)
    }

    #[test]
    fn test_parse_type() {
        let code = r#"
            type Person = {
                name string
                age int?
                friends Person[]
            }
        "#;

        let rawast = parse_raw_ast(code).unwrap();

        println!("{:#?}", rawast);

        let ast = parse_file(code).unwrap();

        let expected = CodeFile {
            body: vec![
                BodyItem::Type(
                    TypeStmt {
                        name: "Person".to_string(),
                        fields: vec![
                            TypeField {
                                ident: "name".to_string(),
                                typ: VarType {
                                    typ: NonNullType::String,
                                    array: false,
                                    nullable: false
                                }
                            },
                            TypeField {
                                ident: "age".to_string(),
                                typ: VarType {
                                    typ: NonNullType::Int,
                                    array: false,
                                    nullable: true
                                }
                            },
                            TypeField {
                                ident: "friends".to_string(),
                                typ: VarType {
                                    typ: NonNullType::Identifier("Person".to_string()),
                                    array: true,
                                    nullable: false
                                }
                            }
                        ]
                    }
                )
            ]
        };

        assert_eq!(ast, expected)
    }

    #[test]
    fn test_parse_match() {
        let code = r#"
            match 10 {
                5 => 10
                10 => 20
                _ => 30
            }
        "#;

        let ast = parse_file(code).unwrap();

        let expected = CodeFile {
            body: vec![
                BodyItem::Expr(
                    Expr::Match(
                        MatchExpr {
                            expr: Box::new(
                                Expr::Const(
                                    Const::Int(10)
                                )
                            ),
                            cases: vec![
                                MatchCase {
                                    patterns: vec![Expr::Const(Const::Int(5))],
                                    body: vec![
                                        BodyItem::Expr(    
                                            Expr::Const(
                                                Const::Int(10)
                                            )
                                        )
                                    ]
                                },
                                MatchCase {
                                    patterns: vec![Expr::Const(Const::Int(10))],
                                    body: vec![
                                        BodyItem::Expr (
                                            Expr::Const(
                                                Const::Int(20)
                                            )
                                        )    
                                    ]
                                },
                                MatchCase {
                                    patterns: vec![],
                                    body: vec![
                                        BodyItem::Expr (
                                            Expr::Const(
                                                Const::Int(30)
                                            )
                                        )    
                                    ]                                     
                                }
                            ]
                        }
                    )
                )
            ]
        };

        assert_eq!(ast, expected)
    }

    #[test]
    fn test_parse_arrow_func() {
        let code = r#"
            const add = (a, b) => a + b
        "#;

        let ast = parse_file(code).unwrap();

        let expected = CodeFile {
            body: vec![
                BodyItem::Const(
                    ConstStmt {
                        ident: "add".to_string(),
                        value: Expr::Func(
                            Func {
                                is_async: false,
                                params: vec![
                                    Param {
                                        name: "a".to_string(),
                                        typ: None
                                    },
                                    Param {
                                        name: "b".to_string(),
                                        typ: None
                                    }
                                ],
                                body: vec![
                                    BodyItem::Expr(
                                        Expr::BinOP(
                                            BinOP {
                                                left: Box::new(
                                                    Expr::Ident(
                                                        "a".to_string()
                                                    )
                                                ),
                                                op: Operator::Add,
                                                right: Box::new(
                                                    Expr::Ident(
                                                        "b".to_string()
                                                    )
                                                )
                                            }
                                        )
                                    )
                                ]
                            }
                        )
                    }
                )
            ]
        };

        assert_eq!(ast, expected)
    }

    #[test]
    fn test_parse_for_range() {
        let code = r#"
            for i in 0..10 {
            }
        "#;

        let ast = parse_file(code).unwrap();

        let expected = CodeFile {
            body: vec![
                BodyItem::Expr(
                    Expr::For(
                        ForExpr {
                            vars: vec![
                                Expr::Ident(
                                    "i".to_string()
                                )
                            ],
                            expr: Some(
                                Box::new(
                                    Expr::Range(
                                        RangeExpr {
                                            start: Some(Box::new(
                                                Expr::Const(
                                                    Const::Int(0)
                                                )
                                            )),
                                            end: Some(Box::new(
                                                Expr::Const(
                                                    Const::Int(10)
                                                )
                                            ))
                                        }
                                    )
                                )
                            ),
                            body: vec![]
                        }
                    )
                )
            ]
        };

        assert_eq!(ast, expected)
    }

    #[test]
    fn parse_call_expr() {
        let code = r#"
            add(1, 2)
        "#;

        let ast = parse_file(code).unwrap();

        let expected = CodeFile {
            body: vec![
                BodyItem::Expr(
                    Expr::Call(
                        Call {
                            callee: Box::new(
                                Expr::Ident(
                                    "add".to_string()
                                )
                            ),
                            args: vec![
                                Expr::Const(
                                    Const::Int(1)
                                ),
                                Expr::Const(
                                    Const::Int(2)
                                )
                            ]
                        }
                    )
                )
            ]
        };

        assert_eq!(ast, expected)
    }

    #[test]
    fn parse_assign_to_property() {
        let code = r#"
            person.name = "John"
        "#;

        let ast = parse_file(code).unwrap();

        let expected = CodeFile {
            body: vec![
                BodyItem::Expr(
                    Expr::Assign(
                        Assign {
                            target: Box::new(Expr::PropAccess(
                                PropAccess {
                                    expr: Box::new(
                                        Expr::Ident(
                                            "person".to_string()
                                        )
                                    ),
                                    prop: Box::new(Expr::Ident("name".to_string()))
                                }
                            )),
                            value: Box::new(Expr::Const(
                                Const::String("John".to_string())
                            ))
                        }
                    )
                )
            ]
        };

        assert_eq!(ast, expected)
    }

    #[test]
    fn test_parse_array() {
        let code = r#"
            [1, 2, 3]
        "#;

        let ast = parse_file(code).unwrap();

        let expected = CodeFile {
            body: vec![
                BodyItem::Expr(
                    Expr::Array(
                        Array {
                            items: vec![
                                Expr::Const(
                                    Const::Int(1)
                                ),
                                Expr::Const(
                                    Const::Int(2)
                                ),
                                Expr::Const(
                                    Const::Int(3)
                                )
                            ]
                        }
                    )
                )
            ]
        };

        assert_eq!(ast, expected)
    }

    #[test]
    fn test_parse_xml() {
        let code = r#"<div></div>"#;

        let ast = parse_file(code).unwrap();

        let expected = CodeFile {
            body: vec![
                BodyItem::Expr(
                    Expr::Xml(
                        Xml {
                            name: "div".to_string(),
                            attrs: vec![],
                            children: vec![]
                        }
                    )
                )
            ]
        };

        assert_eq!(ast, expected)
    }

    // #[test]
    // fn test_parse_sql() {
    //     let code = r#"select id, name from people where id == 1"#;

    //     let ast = parse_file(code).unwrap();

    //     let expected = CodeFile {
    //         body: vec![
    //             BodyItem::Expr(
    //                 Expr::Sql(
    //                     Sql {
    //                         select: vec![
    //                             "id".to_string(),
    //                             "name".to_string()
    //                         ],
    //                         from: "people".to_string(),
    //                         where_: Some(
    //                             Box::new(
    //                                 Expr::BinOP(
    //                                     BinOP {
    //                                         left: Box::new(
    //                                             Expr::Ident(
    //                                                 "id".to_string()
    //                                             )
    //                                         ),
    //                                         op: Operator::Eq,
    //                                         right: Box::new(
    //                                             Expr::Const(
    //                                                 Const::Int(1)
    //                                             )
    //                                         )
    //                                     }
    //                                 )
    //                             )
    //                         )
    //                     }
    //                 )
    //             )
    //         ]
    //     };

    //     assert_eq!(ast, expected)
    // }
}

