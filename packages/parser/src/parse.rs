use std::{any, vec};

use anyhow::Ok;
use pest::iterators::Pair;

use crate::{parser_gen::Rule, Stmts, Stmt, Expr, Term, Factor, ExprRightSide, ExprOperator, ExprTerminalOperator, ExprTerminal, TermOperator, TermRightSide, StructStmt, TypeStmt, EnumStmt, FunctionStmt};


pub fn parse_stmts(pair: Pair<Rule>) -> anyhow::Result<Stmts> {
    let mut stmts = vec![];

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::stmt => {
                let stmt = parse_stmt(pair)?;
                stmts.push(stmt);
            }
            Rule::EOI => (),
            _ => {}
        }
    }

    Ok(stmts)
}

fn parse_break_stmt(pair: Pair<Rule>) -> anyhow::Result<Stmt> {


    Ok(Stmt::BreakStmt(None))
}

fn parse_return_stmt(pair: Pair<Rule>) -> anyhow::Result<Stmt> {
    Ok(Stmt::ReturnStmt(None))
}

pub fn parse_stmt(pair: Pair<Rule>) -> anyhow::Result<Stmt> {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();

    let stmt = match next.as_rule() {
        Rule::expr => Stmt::Expr(parse_expr(next)?),
        Rule::struct_stmt => Stmt::StructStmt(parse_struct_stmt(next)?),
        Rule::type_stmt => Stmt::TypeStmt(parse_type_stmt(next)?),
        Rule::enum_stmt => Stmt::EnumStmt(parse_enum_stmt(next)?),
        Rule::function_stmt => Stmt::FunctionStmt(parse_function_stmt(next)?),
        Rule::continue_stmt => Stmt::ContinueStmt,
        Rule::break_stmt => parse_break_stmt(next)?,
        Rule::return_stmt => parse_return_stmt(next)?,
        _ => {
            return Err(anyhow::anyhow!("Unexpected rule: {:?}", next.as_rule()));
        }
    };
    
    Ok(stmt)
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

fn parse_type_stmt(pair: Pair<Rule>) -> anyhow::Result<TypeStmt> {
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

    Ok(TypeStmt {
        name: name,
        fields: fields,
    })
}

fn parse_struct_stmt(pair: Pair<Rule>) -> anyhow::Result<StructStmt> {
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

    let stmt = StructStmt {
        name: name,
        fields: fields,
    };

    Ok(stmt)
}

fn parse_logical_op(pair: Pair<Rule>) -> anyhow::Result<ExprTerminalOperator> {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();

    let op = match next.as_rule() {
        Rule::logical_and => ExprTerminalOperator::And,
        Rule::logical_or => ExprTerminalOperator::Or,
        Rule::logical_eq => ExprTerminalOperator::Eq,
        Rule::logical_lt => ExprTerminalOperator::Lt,
        Rule::logical_lte => ExprTerminalOperator::Lte,
        Rule::logical_gt => ExprTerminalOperator::Gt,
        Rule::logical_gte => ExprTerminalOperator::Gte,
        Rule::logical_neq => ExprTerminalOperator::Neq,
        _ => {
            return Err(anyhow::anyhow!("Unexpected rule: {:?}", next.as_rule()));
        }
    };

    Ok(op)
}

pub fn parse_expr(pair: Pair<Rule>) -> anyhow::Result<Expr> {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();

    let left = parse_term(next)?;
    let mut rights = vec![];
    let mut terminal = None;

    for pair in inner {
        // match pair.as_rule() {
        //     Rule::logical_op => {
        //         let op = parse_logical_op(pair)?;
        //         let inner = pair.into_inner();
        //         let next = inner.next().unwrap();
        //         let expr = parse_expr(pair)?;
                
        //         terminal = Some(ExprTerminal {
        //             val: Box::new(expr),
        //             op,
        //         });
        //     }
        //     _ => {}
        // };

        // let term = parse_term(pair)?;

        // let right = ExprRightSide {
        //     val: Box::new(term),
        //     op: ExprOperator::Add,
        // };

        // rights.push(right);
    }

    let expr = Expr {
        left: left,
        right: rights,
        terminal: terminal,
    };
    
    Ok(expr)
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

pub fn parse_term(pair: Pair<Rule>) -> anyhow::Result<Term> {   
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();
    let left = parse_factor(next)?;

    let mut rights = vec![];

    for pair in inner {
        let mut inner = pair.into_inner();
        // let next = inner.next().unwrap();
        // let op = parse_term_op(pair)?;
        // let next = inner.next().unwrap();
        // let factor = parse_factor(next)?;

        // let right = TermRightSide {
        //     val: Box::new(factor),
        //     op,
        // };

        // rights.push(right);
    }

    let term = Term {
        left: left,
        right: rights,
    };

    Ok(term)
}

pub fn parse_factor(pair: Pair<Rule>) -> anyhow::Result<Factor> {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();

    let factor = match next.as_rule() {
        // Rule::literal => Factor::Literal(parse_literal(next)?),
        // Rule::expr => Factor::Expr(parse_expr(next)?),
        _ => {
            return Err(anyhow::anyhow!("Unexpected rule: {:?}", next.as_rule()));
        }
    };

    Ok(factor)
}
