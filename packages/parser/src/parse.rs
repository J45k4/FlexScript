use std::{any, vec};

use anyhow::{Ok, bail};
use pest::iterators::{Pair, Pairs};

use crate::{parser_gen::Rule, Stmts, Stmt, Expr, Term, Factor, ExprRightSide, ExprOperator, ExprTerminalOperator, ExprTerminal, TermOperator, TermRightSide, Struct, TypeStmt, EnumStmt, FunctionStmt, BodyItem, Operator, BinOP, Const, Assign, If, IfBranch, ObjExpr, ObjField, ConstStmt, TypeField, VarType, NonNullType};

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

fn parse_assignment_stmt(pair: Pair<Rule>) -> anyhow::Result<Assign> {
    let mut inner = pair.into_inner();

    let next = inner.next().unwrap();

    let target = parse_expr(next)?;

    let next = inner.next().unwrap();

    let value = parse_expr(next)?;

    let assign = Assign {
        target: target,
        value: value
    };

    Ok(assign)
}

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

    // let next = inner.next().unwrap();

    // let condition = parse_expr(next)?;

    // let next = inner.next().unwrap();

    // let body = parse_stmts(next)?;

    // let mut else_body = None;

    // if let Some(next) = inner.next() {
    //     let else_body = parse_stmts(next)?;
    //     else_body = Some(else_body);
    // }

    Ok(Expr::If(if_expr))
}

fn parse_function_stmt(pair: Pair<Rule>) -> anyhow::Result<FunctionStmt> {
    let mut name = String::new();
    let mut params = vec![];

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::identifier => {
                name = pair.as_str().to_string();
            }
            // Rule::field => {
            //     let field = parse_field(pair)?;
            //     fields.push(field);
            // }
            _ => {}
        }
    }

    let stmt = FunctionStmt { 
        name: Some(name), 
        params: params, 
        body: vec![]
    };

    Ok(stmt)
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
        }
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
        Rule::identifier => Expr::Identifier(next.as_str().to_string()),
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

fn parse_const_stmt(pair: Pair<Rule>) -> anyhow::Result<BodyItem> {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();

    let name = next.as_str().to_string();

    let next = inner.next().unwrap();

    let expr = parse_expr(next)?;

    let stmt = ConstStmt {
        ident: name,
        value: expr,
    };

    Ok(BodyItem::Const(stmt))
}

// fn parse_non_null_type(pair: Pair<Rule>) -> anyhow::Result<NonNullType> {
//     let mut inner = pair.into_inner();
//     let next = inner.next().unwrap();

//     let type_name = next.as_str().to_string();

//     let type_name = match type_name.as_str() {
//         "int" => Type::Int,
//         "float" => Type::Float,
//         "string" => Type::String,
//         "bool" => Type::Bool,
//         _ => {
//             return Err(anyhow::anyhow!("Unexpected type name: {}", type_name));
//         }
//     };

//     Ok(NonNullType {
//         type_name: type_name,
//     })
// }



// fn parse_array_type(pair: Pair<Rule>) -> anyhow::Result<Type> {
//     let mut inner = pair.into_inner();
//     let next = inner.next().unwrap();

//     let type_name = next.as_str().to_string();

//     // let type_name = match type_name.as_str() {
//     //     "int" => Type::Int,
//     //     "float" => Type::Float,
//     //     "string" => Type::String,
//     //     "bool" => Type::Bool,
//     //     _ => {
//     //         return Err(anyhow::anyhow!("Unexpected type name: {}", type_name));
//     //     }
//     // };

//     Ok(Type::Array(Box::new(type_name)))
// }

// fn parse_maybe_type(pair: Pair<Rule>) -> anyhow::Result<Type> {
//     let mut inner = pair.into_inner();
//     let next = inner.next().unwrap();

//     let type_name = next.as_str().to_string();

//     let type_name = match type_name.as_str() {
//         "int" => Type::Int,
//         "float" => Type::Float,
//         "string" => Type::String,
//         "bool" => Type::Bool,
//         _ => {
//             return Err(anyhow::anyhow!("Unexpected type name: {}", type_name));
//         }
//     };

//     Ok(type_name)
// }

// fn parse_non_null_type(pair: Pair<Rule>) -> anyhow::Result<NonNullType> {
//     let mut inner = pair.into_inner();
//     let next = inner.next().unwrap();
//     let rule = next.as_rule();

//     match rule {
//         Rule::int_type => {

//         }
//     }

//     let type_name = next.as_str().to_string();

//     let type_name = match type_name.as_str() {
//         "int" => Type::Int,
//         "float" => Type::Float,
//         "string" => Type::String,
//         "bool" => Type::Bool,
//         _ => {
//             return Err(anyhow::anyhow!("Unexpected type name: {}", type_name));
//         }
//     };

//     Ok(NonNullType {
//         type_name: type_name,
//     })
// }

fn parse_type(pair: Pair<Rule>) -> anyhow::Result<VarType> {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();
    let rule = next.as_rule();

    match rule {
        Rule::array_type => {

        },
        Rule::maybe_type => {

        },
        Rule::non_null_type => {

        },
        _ => {
            bail!("Parse type unexpected rule {:?}", rule);
        }
    }

    let typ = VarType {
        array: false,
        non_null: false,
        typ: NonNullType::Int
    };

    Ok(typ)
}

fn parse_type_stmt(pair: Pair<Rule>) -> anyhow::Result<TypeStmt> {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();

    let name = next.as_str().to_string();

    let next = inner.next().unwrap();
    let rule = next.as_rule();

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

                        println!("Field: {:?}", field);
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


    // let ty = parse_type(next)?;

    let stmt = TypeStmt {
        name: name,
        fields: vec![]
    };

    Ok(stmt)
}

pub fn parse_stmt(pair: Pair<Rule>) -> anyhow::Result<BodyItem> {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();
    let rule = next.as_rule();

    let item = match rule {
        Rule::expr => BodyItem::Expr(parse_expr(next)?),
        Rule::assignment_stmt => BodyItem::Assign(parse_assignment_stmt(next)?),
        Rule::if_stmt => BodyItem::Expr(parse_if_stmt(next)?),
        Rule::const_stmt => parse_const_stmt(next)?,
        Rule::object_stmt => BodyItem::Expr(parse_object_stmt(next)?),
        Rule::type_stmt => BodyItem::Type(parse_type_stmt(next)?),
        
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

pub fn parse_stmts(pair: Pair<Rule>) -> anyhow::Result<Vec<BodyItem>> {
    let mut items = vec![];

    for pair in pair.into_inner() {
        let rule = pair.as_rule();

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
    }

    Ok(items)
}
