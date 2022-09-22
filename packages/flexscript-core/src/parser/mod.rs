
pub struct FlexscriptParser;

pub fn parse_text(input: &str) -> Result<ast::Script, ParseError> {
    let mut parser = FlexscriptParser::new(input);
    parser.parse()
}