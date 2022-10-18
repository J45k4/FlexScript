
#[cfg(test)]
mod tests {
    use crate::{parse_text, Stmt, AST, Expr, Literal, Term, ExprRightSide, ExprOperator, parse_raw_ast, parse_file, CodeFile, Const, Assign, StructField, Struct, VarType, NonNullType, If, IfBranch};
    use crate::BinOP;
    use crate::BodyItem;
    use crate::Operation;

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
                            op: Operation::Add,
                            right: Box::new( 
                                Expr::BinOP(
                                    BinOP {
                                        left: Box::new(
                                            Expr::Const(
                                                Const::Int(5)
                                            )
                                        ),
                                        op: Operation::Add,
                                        right: Box::new(
                                            Expr::BinOP(
                                                BinOP {
                                                    left: Box::new( 
                                                        Expr::Const(
                                                            Const::Int(5)
                                                        )
                                                    ),
                                                    op: Operation::Mul,
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
                                            op: Operation::Neq,
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
                                            op: Operation::Eq,
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
        };

        assert_eq!(ast, expected)
    }
}