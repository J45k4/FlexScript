use std::{vec, any};

use parser_gen::Rule;
use pest::{Parser, iterators::Pairs};

mod ast;
mod parser_gen;
mod parser_tests;
mod ast_parsing_tests;
mod parse;

pub use ast::*;

pub struct FlexscriptParser;

pub fn parse_raw_ast(input: &str) -> anyhow::Result<Pairs<Rule>> {
    let pairs = FlexscriptParser::parse(Rule::file, input)?;

    Ok(pairs)
}

pub fn parse_text(input: &str) -> anyhow::Result<AST> {
    // let mut stmts = vec![];

    // let pairs = FlexscriptParser::parse(Rule::file, input)?;

    // for pair in pairs {
    //     match pair.as_rule() {
    //         Rule::file => {
    //             let stmt = parse::parse_stmts(pair)?;
    //             stmts.extend(stmt);
    //         }
    //         Rule::EOI => {}
    //         _ => {}
    //     }
    // }

    Ok(AST { 
        stmts: vec![]
    })
}

pub fn parse_file(input: &str) -> anyhow::Result<CodeFile> {
    let mut file = CodeFile {
        body: vec![]
    };

    let pairs = FlexscriptParser::parse(Rule::file, input)?;

    for pair in pairs {
        match pair.as_rule() {
            Rule::file => {
                let stmt = parse::parse_stmts(pair)?;
                file.body.extend(stmt);
            }
            Rule::EOI => {}
            _ => {}
        }
    }

    Ok(file)
}
