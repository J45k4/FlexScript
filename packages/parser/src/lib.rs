use parser_gen::Rule;
use pest::{Parser, iterators::Pairs};

mod ast;
mod parser_gen;
mod parser_tests;

pub use ast::*;

pub struct FlexscriptParser;

pub fn parse_raw_ast(input: &str) -> anyhow::Result<Pairs<Rule>> {
    let pairs = FlexscriptParser::parse(Rule::file, input)?;

    Ok(pairs)
}

pub fn parse_text(input: &str) -> anyhow::Result<AST> {
    let pairs = FlexscriptParser::parse(Rule::file, input)?;

    Ok(AST { 
        stmts:vec![] 
    })
}