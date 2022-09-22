use parser_gen::Rule;
use pest::Parser;

mod ast;
mod parser_gen;

pub struct FlexscriptParser;

pub fn parse_text(input: &str) -> anyhow::Result<()> {
    let pairs = FlexscriptParser::parse(Rule::file, input)?;

    Ok(())
}