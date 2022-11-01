use std::{any, vec};

use anyhow::{Ok, bail};
use pest::{iterators::{Pair, Pairs}, Parser};

use crate::{parser_gen::Rule, Stmts, Stmt, Expr, Term, Factor, ExprRightSide, ExprOperator, ExprTerminalOperator, ExprTerminal, TermOperator, TermRightSide, Struct, TypeStmt, EnumStmt, Func, Item, Operator, BinOP, Const, Assign, If, IfBranch, ObjExpr, ObjField, ConstStmt, TypeField, VarType, NonNullType, MatchCase, MatchExpr, Body, Param, RangeExpr, ForExpr, Call, PropAccess, Array, Xml, XmlChild, Export, Return, FlexscriptParser, Ast};

fn parse_break_stmt(pair: Pair<Rule>) -> anyhow::Result<Stmt> {


    Ok(Stmt::BreakStmt(None))
}

fn parse_return_stmt(pair: Pair<Rule>) -> anyhow::Result<Stmt> {
    Ok(Stmt::ReturnStmt(None))
}

// pub fn parse_stmts(pair: Pair<Rule>) -> anyhow::Result<Stmts> {
//     let mut stmts = vec![];

//     let inner = pair.into_inner();

//     for pair in inner {
//         let rule = pair.as_rule();

//         match rule {
//             Rule::stmt => {
//                 let stmt = parse_stmt(pair)?;
//                 stmts.push(stmt);
//             }
//             Rule::stmts => {
//                 let stmts = parse_stmts(pair)?;
//                 stmts.extend(stmts);
//             }
//             Rule::EOI => (),
//             _ => {}
//         }
//     }

//     Ok(stmts)
// }

// fn parse_assignment_stmt(pair: Pair<Rule>) -> anyhow::Result<Assign> {
//     let mut inner = pair.into_inner();

//     let next = inner.next().unwrap();

//     let target = parse_expr(next)?;

//     let next = inner.next().unwrap();

//     let value = parse_expr(next)?;

//     let assign = Assign {
//         target: Box::new(target),
//         value: Box::new(value)
//     };

//     Ok(assign)
// }

fn parse_if_branch(pair: Pair<Rule>) -> anyhow::Result<IfBranch> {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();
    let expr = parse_expr(next)?;
    let next = inner.next().unwrap();
    let body = parse_stmts(next)?;

    let branch = IfBranch {
        condition: expr,
        body: body
    };

    Ok(branch)
}

fn parse_if_stmt(pair: Pair<Rule>) -> anyhow::Result<Expr> {
    let mut inner = pair.into_inner();

    let mut if_expr = If {
        branches: vec![],
        else_body:  vec![]
    };

    while let Some(pair) = inner.next() {
        let rule = pair.as_rule();

        match rule {
            Rule::if_branch => {
                let branch = parse_if_branch(pair)?;

                if_expr.branches.push(branch);
            },
            Rule::else_if_branch => {
                let branch = parse_if_branch(pair)?;

                if_expr.branches.push(branch);
            },
            Rule::else_branch => {
                let mut inner = pair.into_inner();
                let next = inner.next().unwrap();
                let body = parse_stmts(next)?;

                if_expr.else_body = body;
            },
            _ => {}
        }
    }

    Ok(Expr::If(if_expr))
}

fn parse_arg(pair: Pair<Rule>) -> anyhow::Result<Param> {
    let mut inner = pair.into_inner();

    let next = inner.next().unwrap();

    let name = match next.as_rule() {
        Rule::ident => next.as_str().to_string(),
        _ => bail!("Expected identifier")
    };

    let next = inner.next();

    let typ = match next {
        Some(pair) => {
            let typ = parse_type(pair)?;
            Some(typ)
        },
        None => None
    };

    let param = Param {
        name,
        typ
    };

    Ok(param)
}

fn parse_args(pair: Pair<Rule>) -> anyhow::Result<Vec<Param>> {
    let mut args = vec![];

    for pair in pair.into_inner() {
        let arg = parse_arg(pair)?;

        args.push(arg);
    }

    Ok(args)
}

fn parse_block(pair: Pair<Rule>) -> anyhow::Result<Body> {
    println!("parse_block");

    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();

    let body = parse_stmts(next)?;
    
    Ok(body)
}

fn parse_normal_func(pair: Pair<Rule>) -> anyhow::Result<Func> {
    let mut inner = pair.into_inner();

    let next = inner.next().unwrap();

    let params = parse_args(next)?;

    let next = inner.next().unwrap();
    let rule = next.as_rule();

    let body = match rule {
        Rule::block_stmt => {
            parse_block(next)?
        },
        Rule::stmt => {
            vec![parse_stmt(next)?]
        },
        _ => {
            panic!("Parse normal func error {:?}", rule);
        }  
    };
    let func = Func {
        params: params,
        body: body,
        is_async: false
    };

    Ok(func)
}

fn parse_async_func(pair: Pair<Rule>) -> anyhow::Result<Func> {
    let mut inner = pair.into_inner();

    let next = inner.next().unwrap();

    let mut f = parse_normal_func(next)?;

    f.is_async = true;

    Ok(f)
}

fn parse_function_stmt(pair: Pair<Rule>) -> anyhow::Result<Func> {
    println!("parse_function_stmt");

    let mut inner = pair.into_inner();

    let next = inner.next().unwrap();

    let func = match next.as_rule() {
        Rule::async_func => {
            parse_async_func(next)?
        },
        Rule::normal_func => {
            parse_normal_func(next)?
        },
        _ => {
            panic!("parse function stmt error {:?}", next);
        }
    };

    Ok(func)
}

fn parse_enum_stmt(pair: Pair<Rule>) -> anyhow::Result<EnumStmt> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let mut fields = vec![];

    // for pair in inner {
    //     match pair.as_rule() {
    //         Rule::field => {
    //             let field = parse_field(pair)?;
    //             fields.push(field);
    //         }
    //         _ => {}
    //     }
    // }

    let enum_stmt = EnumStmt {
        name: name,
        fields: fields,
    };

    Ok(enum_stmt)
}

// fn parse_type_stmt(pair: Pair<Rule>) -> anyhow::Result<TypeStmt> {
//     let mut inner = pair.into_inner();
//     let name = inner.next().unwrap().as_str().to_string();
//     let mut fields = vec![];

//     for pair in inner {
//         match pair.as_rule() {
//             // Rule::field => {
//             //     let field = parse_field(pair)?;
//             //     fields.push(field);
//             // }
//             _ => {}
//         }
//     }

//     Ok(TypeStmt {
//         name: name,
//         fields: fields,
//     })
// }

fn parse_struct_stmt(pair: Pair<Rule>) -> anyhow::Result<Struct> {
    let mut inner = pair.into_inner();

    let name = inner.next().unwrap().as_str().to_string();

    let mut fields = vec![];

    for pair in inner {
        match pair.as_rule() {
            // Rule::field => {
            //     let field = parse_field(pair)?;
            //     fields.push(field);
            // }
            _ => {}
        }
    }

    let stmt = Struct {
        name: name,
        fields: fields,
    };

    Ok(stmt)
}

fn parse_logical_op(pair: Pair<Rule>) -> anyhow::Result<Operator> {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();

    let op = match next.as_rule() {
        Rule::logical_and => Operator::And,
        Rule::logical_or => Operator::Or,
        Rule::logical_eq => Operator::Eq,
        Rule::logical_lt => Operator::Lt,
        Rule::logical_lte => Operator::Lte,
        Rule::logical_gt => Operator::Gt,
        Rule::logical_gte => Operator::Gte,
        Rule::logical_neq => Operator::Neq,
        _ => {
            bail!("Parse logical op unexpected rule: {:?}", next.as_rule());
        }
    };

    Ok(op)
}
fn parse_operator(pair: Pair<Rule>) -> anyhow::Result<Operator> {
    match pair.as_rule() {
        // Rule::logical_op => {
        //     let op = parse_logical_op(pair)?;
        //     Ok(Operation::Logical(op))
        // }
        Rule::plus => Ok(Operator::Add),
        Rule::minus => Ok(Operator::Sub),
        Rule::multi => Ok(Operator::Mul),
        Rule::divide => Ok(Operator::Div),
        Rule::modulo => Ok(Operator::Modulus),
        Rule::logical_op => {
            parse_logical_op(pair)
        }
        _ => Err(anyhow::anyhow!("Parse operator unexpected rule: {:?}", pair.as_rule())),
    }
}

pub fn parse_term_inner(mut inner: Pairs<Rule>) -> anyhow::Result<Expr> {
    let next = inner.next().unwrap();
    let left = parse_factor(next)?;
    
    let next = inner.next();

    if let Some(next) = next {
        let op = parse_operator(next)?;
        let right = parse_term_inner(inner)?;
        
        let binop = BinOP {
            op: op,
            left: Box::new(left),
            right: Box::new(right),  
        };

        return Ok(Expr::BinOP(binop));
    }

    Ok(left)
}

pub fn parse_term(pair: Pair<Rule>) -> anyhow::Result<Expr> {   
    let inner = pair.into_inner();
    
    parse_term_inner(inner)
}

fn parse_property_access(pair: Pair<Rule>) -> anyhow::Result<Expr> {
    let mut inner = pair.into_inner();

    let next = inner.next().unwrap();

    let left = parse_term(next)?;

    let next = inner.next().unwrap();

    let right = parse_ident(next)?;

    let prop_access = PropAccess {
        expr: Box::new(left),
        prop: Box::new(right)
    };

    Ok(Expr::PropAccess(prop_access))
}

fn parse_assign(pair: Pair<Rule>) -> anyhow::Result<Assign> {
    println!("parse assign");

    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();

    let target = parse_ident(next)?;

    let next = inner.next().unwrap();
    let value = parse_expr(next)?;

    let a = Assign { 
        target: Box::new(target),
        value: Box::new(value)
    };

    Ok(a)
}

fn member_access(pair: Pair<Rule>) -> anyhow::Result<Expr> {
    println!("member_access");

    let mut inner = pair.into_inner();

    let next = inner.next().unwrap();

    let e = parse_ident(next)?;

    Ok(e)
}

// fn member_call(pair: Pair<Rule>) -> anyhow::Result<Call> {
//     let mut inner = pair.into_inner();

//     let next = inner.next().unwrap();

//     let left = parse_expr(next)?;

//     let next = inner.next().unwrap();

//     let right = parse_expr(next)?;

//     let member_call = Call {
//         expr: Box::new(left),
//         args: vec![right]
//     };

//     Ok(member_call)
// }

fn parse_call_args(pair: Pair<Rule>) -> anyhow::Result<Vec<Expr>> {
    let mut inner = pair.into_inner();
    let mut args = vec![];

    while let Some(next) = inner.next() {
        let rule = next.as_rule();

        match rule {
            Rule::expr => {
                args.push(parse_expr(next)?);
            },
            Rule::stmt => {
                args.push(parse_expr(next)?);
            },
            _ => {
                bail!("Parse call args unexpected rule: {:?}", rule);
            }
        }
    }

    Ok(args)
}

fn parse_method_call(pair: Pair<Rule>) -> anyhow::Result<Expr> {
    println!("parse_method_call");

    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();
    let rule = next.as_rule();

    println!("parse_method_call rule {:?}", rule);

    let callee = match rule {
        Rule::ident => {
            parse_ident(next)?
        },
        _ => {
            bail!("Parse method call unexpected rule: {:?}", rule);
        }
    };

    let next = inner.next().unwrap();
    let args = parse_call_args(next)?;

    let method_call = Call {
        callee: Box::new(callee),
        args: args
    };

    Ok(Expr::Call(method_call))
}

fn parse_ident_str(pair: Pair<Rule>) -> String {
    let ident = pair.as_str().trim().to_string();

    ident
}

fn parse_ident(pair: Pair<Rule>) -> anyhow::Result<Expr> {
    println!("parse ident");

    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();
    let rule = next.as_rule();

    println!("parse ident inner rule {:?}", rule);

    let ident_str = match rule {
        Rule::ident_str => parse_ident_str(next),
        _ => {
            bail!("Parse ident unexpected rule: {:?}", next.as_rule());
        }
    };

    println!("parse ident ident_str: {:?}", ident_str);

    let next = inner.next();

    let e = match next {
        Some(next) => {
            match next.as_rule() {
                Rule::member_access => {
                    println!("parse_ident member_access");

                    let e = member_access(next)?;

                    let p = PropAccess {
                        expr: Box::new(
                            Expr::Ident(
                                ident_str
                            )
                        ),
                        prop: Box::new(e)
                    };

                    Expr::PropAccess(p)
                },
                Rule::method_call => {
                    println!("parse_ident method_call");

                    

                    let mut inner = next.into_inner();
                    let next = inner.next().unwrap();

                    let call = Call {
                        callee: Box::new(
                            Expr::Ident(
                                ident_str
                            )
                        ),
                        args: parse_call_args(next)?
                    };

                    Expr::Call(call)
                },
                _ => {
                    bail!("Parse ident unexpected rule: {:?}", next.as_rule());
                }
            }
        },
        None => {
            Expr::Ident(ident_str)
        }
    };

    Ok(e)
}

fn parse_expr_inner(mut inner: Pairs<Rule>) -> anyhow::Result<Expr> {
    let next = inner.next().unwrap();

    let rule = next.as_rule();

    let left = match rule {
        Rule::term => {
            parse_term(next)?
        },
        Rule::expr => {
            parse_expr(next)?
        },
        Rule::object_stmt => {
            parse_object_stmt(next)?
        },
        Rule::function_stmt => {
            Expr::Func(parse_function_stmt(next)?)
        },
        Rule::assignment_stmt => {
            Expr::Assign(parse_assign(next)?)
        },
        Rule::call_stmt => {
            Expr::Call(parse_call_stmt(next)?)
        },
        // Rule::property_access => {
        //     parse_property_access(next)?
        // },
        _ => {
            bail!("Parse expr inner unexpected rule: {:?}", rule);
        }
    };

    if let Some(next) = inner.next() {
        let op = parse_operator(next)?;

        let right = parse_expr_inner(inner)?;
        
        let binop = BinOP {
            op: op,
            left: Box::new(left),
            right: Box::new(right),  
        };

        return Ok(Expr::BinOP(binop));
    }

    Ok(left)
}

pub fn parse_expr(pair: Pair<Rule>) -> anyhow::Result<Expr> {
    println!("parse_expr");

    let inner = pair.into_inner();

    parse_expr_inner(inner)
}

fn parse_term_op(pair: Pair<Rule>) -> anyhow::Result<TermOperator> {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();

    let op = match next.as_rule() {
        Rule::multi => TermOperator::Mul,
        Rule::divide => TermOperator::Div,
        Rule::modulo => TermOperator::Mod,
        _ => {
            return Err(anyhow::anyhow!("Unexpected rule: {:?}", next.as_rule()));
        }
    };

    Ok(op)
}

pub fn parse_factor(pair: Pair<Rule>) -> anyhow::Result<Expr> {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();    

    let expr = match next.as_rule() {
        Rule::string_literal => {
            let mut inner = next.into_inner();
            let next = inner.next().unwrap();
            let val = next.as_str().to_string();
            
            Expr::Const(Const::String(val))
        },
        Rule::float => {
            let val = next.as_str().to_string();
            let val = val.parse::<f64>()?;
            
            Expr::Const(Const::Float(val))
        },
        Rule::integer => {
            let val = next.as_str().to_string();
            let val = val.parse::<i64>()?;
            
            Expr::Const(Const::Int(val))
        },
        Rule::ident => {
            parse_ident(next)?
        }
        //Rule::ident => Expr::Ident(next.as_str().to_string()),
        // Rule::expr => Factor::Expr(parse_expr(next)?),
        _ => {
            return Err(anyhow::anyhow!("Parse factor unexpected rule: {:?}", next.as_rule()));
        }
    };

    Ok(expr)
}

fn parse_object_stmt(pair: Pair<Rule>) -> anyhow::Result<Expr> {
    let inner = pair.into_inner();

    let mut fields = vec![];

    for pair in inner {
        let rule = pair.as_rule();

        match rule {
            Rule::object_stmt_field => {
                let mut inner = pair.into_inner();
                let next = inner.next().unwrap();

                let target = next.as_str().to_string();

                let next = inner.next().unwrap();

                let expr = parse_expr(next)?;

                let field = ObjField {
                    target: target,
                    value: expr,
                };

                fields.push(field);
            },
            _ => {}
        }
    }

    let expr = ObjExpr {
        fields: fields,
    };

    Ok(Expr::ObjExpr(expr))
}

fn parse_const_stmt(pair: Pair<Rule>) -> anyhow::Result<Item> {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();

    let name = next.as_str().trim().to_string();

    let next = inner.next().unwrap();

    let expr = parse_expr(next)?;

    let stmt = ConstStmt {
        ident: name,
        value: expr,
    };

    Ok(Item::Const(stmt))
}

fn parse_non_null_type(pair: Pair<Rule>) -> anyhow::Result<NonNullType> {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();
    let rule = next.as_rule();

    let typ = match rule {
        Rule::int_type => {
            NonNullType::Int
        },
        Rule::float_type => {
            NonNullType::Float
        },
        Rule::string_type => {
            NonNullType::String
        },
        Rule::bool_type => {
            NonNullType::Bool
        },
        Rule::ident => {


            NonNullType::Identifier(next.as_str().to_string())
        }
        _ => {
            bail!("Unexpected rule: {:?}", rule);
        }
    };

    Ok(typ)
}


fn parse_array_type(pair: Pair<Rule>) -> anyhow::Result<VarType> {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();
    let rule = next.as_rule();

    let typ = match rule {
        Rule::non_null_type => {
            VarType {
                typ: parse_non_null_type(next)?,
                array: true,
                nullable: false
            }
        },
        _ => {
            bail!("Parse arrayt type unexpected rule: {:?}", rule);
        }
    };

    Ok(typ)
}

fn parse_maybe_type(pair: Pair<Rule>) -> anyhow::Result<VarType> {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();
    let rule = next.as_rule();

    let typ = match rule {
        Rule::array_type => {
            let mut v = parse_array_type(next)?;

            v.nullable = true;

            v
        },
        Rule::non_null_type => {
            let typ = parse_non_null_type(next)?;
            
            VarType {
                typ: typ,
                nullable: true,
                array: false
            }
        },
        _ => {
            bail!("Parse maybe type unexpected rule: {:?}", rule);
        }
    };

    Ok(typ)
}

fn parse_type(pair: Pair<Rule>) -> anyhow::Result<VarType> {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();
    let rule = next.as_rule();

    let typ = match rule {
        Rule::array_type => {
            parse_array_type(next)?
        },
        Rule::maybe_type => {
            parse_maybe_type(next)?
        },
        Rule::non_null_type => {
            let n = parse_non_null_type(next)?;

            VarType {
                typ: n,
                array: false,
                nullable: false,
            }
        },
        _ => {
            bail!("Parse type unexpected rule {:?}", rule);
        }
    };

    Ok(typ)
}

fn parse_type_stmt(pair: Pair<Rule>) -> anyhow::Result<TypeStmt> {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();

    let name = next.as_str().to_string();

    let next = inner.next().unwrap();
    let rule = next.as_rule();

    let mut stmt = TypeStmt {
        name: name,
        fields: vec![]
    };

    match rule {
        Rule::type_object => {
            let mut inner = next.into_inner();

            while let Some(next) = inner.next() {
                let rule = next.as_rule();

                match rule {
                    Rule::type_field => {
                        let mut inner = next.into_inner();
                        let next = inner.next().unwrap();

                        let name = next.as_str().to_string();

                        let next = inner.next().unwrap();

                        let ty = parse_type(next)?;

                        let field = TypeField {
                            ident: name,
                            typ: ty,
                        };

                        stmt.fields.push(field);
                    },
                    _ => {
                        bail!("Parse type stmt unexpected rule: {:?}", rule);
                    }
                }
            }
        },
        _ => {
            bail!("Parse type stmt unexpected rule: {:?}", rule);
        }
    }

    Ok(stmt)
}

fn parse_match_pattern(pair: Pair<Rule>, patterns: &mut Vec<Expr>) -> anyhow::Result<()> {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();

    patterns.push(parse_expr(next)?);

    let next = inner.next();

    if let Some(next) = next {
        parse_match_pattern(next, patterns)?;
    }

    Ok(())
}

fn parse_match_case(pair: Pair<Rule>) -> anyhow::Result<MatchCase> {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();

    let mut expr = vec![];
    
    let next = match next.as_rule() {
        Rule::match_condition => {
            parse_match_pattern(next, &mut expr)?;

            inner.next().unwrap()
        },
        _ => {
            next
        }
    };

    let case = match next.as_rule() {
        Rule::block_stmt => {
            let mut inner = next.into_inner();
            let next = inner.next().unwrap();

            let body = parse_stmts(next)?;

            MatchCase {
                patterns: expr,
                body: body,
            }
        },
        Rule::stmt => {
            let s = parse_stmt(next)?;

            MatchCase {
                patterns: expr,
                body: vec![s],
            }
        },
        _ => {
            bail!("Parse match case unexpected rule: {:?}", next.as_rule());
        }
    };

    Ok(case)
}

fn parse_match_stmt(pair: Pair<Rule>) -> anyhow::Result<Expr> {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();

    let expr = parse_expr(next)?;

    let mut match_expr = MatchExpr {
        expr: Box::new(expr),
        cases: vec![],
    };

    while let Some(next) = inner.next() {
        let rule = next.as_rule();

        match rule {
            Rule::match_case => {
                let case = parse_match_case(next)?;

                match_expr.cases.push(case);
            },
            _ => {
                bail!("Parse match unexpected rule: {:?}", rule);
            }
        }
    }

    Ok(Expr::Match(match_expr))
}

fn parse_range_expr(pair: Pair<Rule>) -> anyhow::Result<RangeExpr> {
    let mut inner = pair.into_inner();

    let mut r = RangeExpr {
        start: None,
        end: None,
    };

    while let Some(next) = inner.next() {
        let rule = next.as_rule();

        match rule {
            Rule::range_start => {
                let mut inner = next.into_inner();
                let next = inner.next().unwrap();

                r.start = Some(Box::new(parse_expr(next)?));
            },
            Rule::range_end => {
                let mut inner = next.into_inner();
                let next = inner.next().unwrap();

                r.end = Some(Box::new(parse_expr(next)?));
            },
            _ => {
                bail!("Parse range expr unexpected rule: {:?}", rule);
            }
        }
    }


    Ok(r)
}

fn parse_idents(pair: Pair<Rule>) -> anyhow::Result<Vec<Expr>> {
    let mut inner = pair.into_inner();
    let mut idents = vec![];

    while let Some(next) = inner.next() {
        let ident = parse_ident(next)?;

        idents.push(ident);
    }

    Ok(idents)
}

fn parse_for_expr(pair: Pair<Rule>) -> anyhow::Result<ForExpr> {
    println!("parse_for_expr");

    let mut inner = pair.into_inner();

    let next = inner.next().unwrap();
    let idents = parse_idents(next)?;

    let next = inner.next().unwrap();
    let expr = parse_range_expr(next)?;

    let next = inner.next();

    let body = if let Some(next) = next {
        parse_stmts(next)?
    } else {
        vec![]
    };
    

    let f = ForExpr {
        vars: idents,
        body: body,
        expr: Some(Box::new(Expr::Range(expr))),
    };

    Ok(f)
}



fn parse_call_stmt(pair: Pair<Rule>) -> anyhow::Result<Call> {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();

    let expr = parse_expr(next)?;

    let next = inner.next().unwrap();

    let args = parse_call_args(next)?;

    let c = Call {
        callee: Box::new(expr),
        args: args,
    };

    Ok(c)
}

fn parse_array_stmt(pair: Pair<Rule>) -> anyhow::Result<Array> {
    println!("parse array stmt");

    let mut inner = pair.into_inner();
    
    let mut arr = Array {
        items: vec![],
    };

    let next = inner.next().unwrap();

    match next.as_rule() {
        Rule::array_items => {},
        _ => {
            bail!("Parse array stmt unexpected rule: {:?}", next.as_rule());
        }
    };

    let mut inner = next.into_inner();

    while let Some(next) = inner.next() {
        let rule = next.as_rule();

        println!("parse_array_stmt array item: {:?}", rule);

        match rule {
            Rule::expr => {
                arr.items.push(parse_expr(next)?);
            },
            Rule::stmt => {
                arr.items.push(parse_expr(next)?);
            },
            _ => {
                bail!("Parse array stmt unexpected rule: {:?}", rule);
            }
        }
    }

    Ok(arr)
}

fn parse_xml_child(ast: Pair<Rule>) -> anyhow::Result<XmlChild> {
    println!("parse_xml_child {:#?}", ast);

    let rule = ast.as_rule();

    match rule {
        Rule::xml_stmt => {
            let mut inner = ast.into_inner();
            let next = inner.next().unwrap();

            let tag = parse_xml_stmt(next)?;

            Ok(XmlChild::Xml(tag))
        },
        Rule::xml_var => {
            let mut inner = ast.into_inner();
            let next = inner.next().unwrap();

            let e = parse_expr(next)?;

            Ok(XmlChild::Expr(e))
        },
        Rule::ident_str => {
            let ident = parse_ident_str(ast);

            Ok(XmlChild::Ident(ident))
        },
        _ => {
            bail!("Parse xml child unexpected rule: {:?}", rule);
        }
    }
}

fn parse_xml_stmt(ast: Pair<Rule>) -> anyhow::Result<Xml> {
    println!("parse_xml_stmt");

    let mut inner = ast.into_inner();
    let next = inner.next().unwrap();

    let name = match next.as_rule() {
        Rule::xml_start => {
            println!("parse_xml_stmt xml start");

            let mut inner = next.into_inner();
            let next = inner.next().unwrap();
            let mut inner = next.into_inner();
            let next = inner.next().unwrap();

            next.as_str().to_string()
        },
        _ => {
            bail!("Parse xml stmt unexpected rule: {:?}", next.as_rule());
        }
    };

    println!("parse_xml_stmt name {:?}", name);

    let mut xml = Xml {
        name: name,
        attrs: vec![],
        children: vec![],
    };

    while let Some(next) = inner.next() {
        let rule = next.as_rule();

        let x = match rule {
            Rule::xml_stmt => {
                println!("parse_xml_stmt xml stmt");

                XmlChild::Xml(
                    parse_xml_stmt(next)?
                )
            },
            Rule::xml_child => {
                println!("parse_xml_stmt xml child");

                let mut inner = next.into_inner();
                let next = inner.next().unwrap();

                parse_xml_child(next)?
            },
            Rule::xml_var => {
                println!("parse_xml_stmt xml var");

                let mut inner = next.into_inner();

                let next = inner.next().unwrap();

                let e = parse_expr(next)?;

                XmlChild::Expr(e)
            },
            Rule::ident => {
                println!("parse_xml_stmt ident");

                let mut inner = next.into_inner();

                let next = inner.next().unwrap();

                match next.as_rule() {
                    Rule::ident => {
                        let ident = next.to_string();

                        XmlChild::Ident(ident)
                    },
                    _ => {
                        bail!("Parse xml stmt unexpected rule: {:?}", next.as_rule());
                    }
                }
            },
            Rule::xml_end => {
                println!("parse_xml_stmt xml end");
                break;
            },
            _ => {
                bail!("Parse xml stmt unexpected rule: {:?}", rule);
            }
        };

        xml.children.push(x);
    }

    Ok(xml)
}

fn parse_default_export(pair: Pair<Rule>) -> anyhow::Result<Export> {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();

    let export = Export {
        item: Box::new(parse_stmt(next)?),
        default: true,
        name: None,
    };

    Ok(export)
}

fn parse_named_export(pair: Pair<Rule>) -> anyhow::Result<Export> {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();

    let ident = parse_ident_str(next);

    let next = inner.next().unwrap();

    let b = parse_stmt(next)?;

    let export = Export {
        item: Box::new(b),
        default: false,
        name: Some(ident),
    };

    Ok(export)
}

fn parse_export(ast: Pair<Rule>) -> anyhow::Result<Export> {
    println!("parse_export");

    let mut inner = ast.into_inner();
    let next = inner.next().unwrap();
    let rule = next.as_rule();

    let e = match rule {
        Rule::default_export => {
            parse_default_export(next)?
        },
        Rule::named_export => {
            parse_named_export(next)?
        },
        _ => {
            bail!("Parse export unexpected rule: {:?}", rule);
        }
    };


    // let mut e = Export {
    //     name: None,
    //     expr: None,
    // };

    // match 

    // match next.as_rule() {
    //     Rule::ident => {
    //         let ident = parse_ident(next);

    //         e.name = Some(ident);
    //     },
    //     Rule::expr => {
    //         let expr = parse_expr(next)?;

    //         e.expr = Some(Box::new(expr));
    //     },
    //     _ => {
    //         bail!("Parse export unexpected rule: {:?}", next.as_rule());
    //     }
    // };

    Ok(e)
}

fn parse_return(ast: Pair<Rule>) -> anyhow::Result<Return> {
    println!("parse_return");

    let mut inner = ast.into_inner();
    let next = inner.next().unwrap();
    let rule = next.as_rule();

    println!("parse_return rule: {:?}", rule);

    let item = match rule {
        Rule::expr => {
            Item::Expr(parse_expr(next)?)
        },
        Rule::stmt => {
            parse_stmt(next)?
        },
        _ => {
            bail!("Parse return unexpected rule: {:?}", rule);
        }
    };

    let ret = Return {
        items: vec![item],
    };

    Ok(ret)
}

pub fn parse_stmt(pair: Pair<Rule>) -> anyhow::Result<Item> {
    println!("parse_stmt {:?}", pair.as_rule());

    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();
    let rule = next.as_rule();

    println!("parse_stmt inner rule {:?}", rule);

    let item = match rule {
        Rule::expr => Item::Expr(parse_expr(next)?),
        // Rule::assignment_stmt => BodyItem::Assign(parse_assignment_stmt(next)?),
        Rule::if_stmt => Item::Expr(parse_if_stmt(next)?),
        Rule::const_stmt => parse_const_stmt(next)?,
        Rule::object_stmt => Item::Expr(parse_object_stmt(next)?),
        Rule::type_stmt => Item::Type(parse_type_stmt(next)?),
        Rule::match_stmt => Item::Expr(parse_match_stmt(next)?),
        Rule::function_stmt => Item::Expr(Expr::Func(parse_function_stmt(next)?)),
        Rule::range_expr => Item::Expr(Expr::Range(parse_range_expr(next)?)),    
        Rule::for_expr => Item::Expr(Expr::For(parse_for_expr(next)?)),
        Rule::call_stmt => Item::Expr(Expr::Call(parse_call_stmt(next)?)),
        Rule::array_stmt => Item::Expr(Expr::Array(parse_array_stmt(next)?)),
        Rule::xml_stmt => Item::Expr(Expr::Xml(parse_xml_stmt(next)?)),
        Rule::export => Item::Export(parse_export(next)?),
        Rule::return_stmt => Item::Return(parse_return(next)?),
        Rule::term => Item::Expr(parse_term(next)?),
        // Rule::struct_stmt => Stmt::StructStmt(parse_struct_stmt(next)?),
        // Rule::type_stmt => Stmt::TypeStmt(parse_type_stmt(next)?),
        // Rule::enum_stmt => Stmt::EnumStmt(parse_enum_stmt(next)?),
        // Rule::function_stmt => Stmt::FunctionStmt(parse_function_stmt(next)?),
        // Rule::continue_stmt => Stmt::ContinueStmt,
        // Rule::break_stmt => parse_break_stmt(next)?,
        // Rule::return_stmt => parse_return_stmt(next)?,
        _ => {
            return Err(anyhow::anyhow!("Parse stmt unexpected rule: {:?}", next.as_rule()));
        }
    };
    
    Ok(item)
}

pub fn parse_stmts(pair: Pair<Rule>) -> anyhow::Result<Vec<Item>> {
    println!("parse_stmts");

    let mut items = vec![];

    let rule = pair.as_rule();

    println!("parse_stmts inner next rule: {:?}", rule);

    match rule {
        Rule::stmts => {
            let inner = pair.into_inner();

            for pair in inner {
                let rule = pair.as_rule();

                match rule {
                    Rule::stmt => {
                        let stmt = parse_stmt(pair)?;
                        items.push(stmt);
                    }
                    _ => {}
                }
            }
        }
        Rule::EOI => (),
        _ => {}
    }

    Ok(items)
}

pub fn parse_raw_ast(input: &str) -> anyhow::Result<Pairs<Rule>> {
    let pairs = FlexscriptParser::parse(Rule::file, input)?;

    Ok(pairs)
}

pub fn parse_file(input: &str) -> anyhow::Result<Ast> {
    let mut file = Ast {
        body: vec![]
    };

    let pairs = FlexscriptParser::parse(Rule::file, input)?;

    for pair in pairs {
        match pair.as_rule() {
            Rule::file => {
                let mut inner = pair.into_inner();
                let next = inner.next().unwrap();

                let stmt = parse_stmts(next)?;
                file.body.extend(stmt);
            }
            Rule::EOI => {}
            _ => {}
        }
    }

    Ok(file)
}
