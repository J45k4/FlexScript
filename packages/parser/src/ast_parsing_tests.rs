
#[cfg(test)]
mod tests {
    use crate::{parse_text, Stmt, AST, Expr, Literal, Term, ExprRightSide, ExprOperator, parse_raw_ast, parse_file, CodeFile, Const, Assign, StructField, Struct, VarType, NonNullType, If, IfBranch, ConstStmt, ObjExpr, ObjField, TypeStmt, TypeField};
    use crate::BinOP;
    use crate::BodyItem;
    use crate::Operator;

    use super::*;

    // #[test]
    // fn test_string_literal() {
    //     let code = r#""qwerty""#;

    //     let ast = parse_text(code).unwrap();



    //     assert_eq!(ast, AST {
    //         stmts: vec![
    //             Stmt::Expr(Expr {
    //                 left: Term {
    //                     left: crate::Factor::String("qwerty".to_string()),
    //                     right: vec![]
    //                 },
    //                 right: vec![],
    //                 terminal: None
    //             })
    //         ]
    //     });
    // }

    // #[test]
    // fn test_float_literal() {
    //     let code = r#"5.5"#;

    //     let ast = parse_text(code).unwrap();

    //     assert_eq!(ast, AST {
    //         stmts: vec![
    //             Stmt::Expr(Expr {
    //                 left: Term {
    //                     left: crate::Factor::Float(5.5),
    //                     right: vec![]
    //                 },
    //                 right: vec![],
    //                 terminal: None
    //             })
    //         ]
    //     });
    // }

    // #[test]
    // fn test_int_literal() {
    //     let code = r#"5"#;

    //     let ast = parse_text(code).unwrap();

    //     assert_eq!(ast, AST {
    //         stmts: vec![
    //             Stmt::Expr(Expr {
    //                 left: Term {
    //                     left: crate::Factor::Int(5),
    //                     right: vec![]
    //                 },
    //                 right: vec![],
    //                 terminal: None
    //             })
    //         ]
    //     });
    // }

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
                        target: Expr::Identifier("a".to_string()),
                        value: Expr::Const(
                            Const::Int(5)
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
                                    non_null: false
                                }
                            },
                            StructField {
                                name: "b".to_string(),
                                typ: VarType {
                                    typ: NonNullType::Int,
                                    array: false,
                                    non_null: false
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
                                    non_null: false
                                }
                            },
                            TypeField {
                                ident: "age".to_string(),
                                typ: VarType {
                                    typ: NonNullType::Int,
                                    array: false,
                                    non_null: false
                                }
                            }
                        ]
                    }
                )
            ]
        };

        assert_eq!(ast, expected)
    }
}

