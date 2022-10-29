use logos::Logos;
use logos::Lexer;

#[derive(Logos, Debug, PartialEq)]
pub enum Token<'a>  {
    #[regex(r#""(?:[^"]|\\")*""#, |lex| &lex.slice()[1..lex.slice().len() - 1])]
    StrLit(&'a str),
    #[token("(")]
    RoundOpen,
    #[token(")")]
    RoundClose,
    #[token("{")]
    CurlyOpen,
    #[token("}")]
    CurlyClose,
    #[token("[")]
    SquareOpen,
    #[token("]")]
    SquareClose,
    #[token("<")]
    AngleOpen,
    #[token(">")]
    AngleClose,
    #[token(".")]
    Period,
    #[token(",")]
    Comma,
    #[token("::")]
    DoubleColon,
    #[token(":")]
    Colon,
    #[token("=>")]
    Arrow,
    #[token("==")]
    Eq,
    #[token("=")]
    Assign,
    #[token("\"")]
    DoubleQuote,
    #[token("'")]
    SingleQuote,
    #[token("*")]
    MulOp,
    #[token("/")]
    DivOp,
    #[token("+")]
    PlusOp,
    #[token("-")]
    MinusOp,
    #[token("%")]
    ModOp,
    
    #[token("\n")]
    LineBreak,
    // #[token("\t")]
    // Indention,
    #[token("&&")]
    LogicalAnd,
    #[token("||")]
    LogicalOr,
    #[token("!")]
    Not,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token(";")]
    Semicolo,
    #[regex("[0-9]+.[0-9]", |lex| lex.slice().parse())]
    Float(f64),
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Int(i64),
    #[regex("[a-zA-Z]+")]
    Text(&'a str),
    #[error]
    #[regex(r"[ \t\f]+", logos::skip)]
    Error
}

pub fn print_tokenstream(code: &str) {
    let lex: Vec<_> = Token::lexer(code).spanned().collect();

    println!("tokenstream {:?}", lex);
}

pub struct Tokenizer<'a> {
    lex: Lexer<'a, Token<'a>>,
    next: Option<Token<'a>>
}

impl <'a> Tokenizer<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            lex: Token::lexer(code),
            next: None
        }
    }

    pub fn next(&mut self) -> Option<Token<'a>> {
        match self.next.take() {
            Some(next) => {
                self.next = None;
                Some(next)
            },
            None => {
                self.lex.next()
            }
        }
    }

    pub fn peek(&mut self) -> Option<&Token<'a>> {
        let next = self.next();

        self.next = next;

        self.next.as_ref()
    }

    pub fn span(&self) -> std::ops::Range<usize> {
        self.lex.span()
    }
}



#[cfg(test)]
mod tests {
    use logos::Logos;

    use super::Token;

    #[test]
    fn test_math_syntax() {
        let code = r"num = 5 * (5 + 2) / 2 % 5";

        let lex: Vec<_> = Token::lexer(code).spanned().collect();

        println!("lex {:?}", lex);
    } 

    #[test]
    fn test_condition_logic() {
        let code = r"if x == 4 || y <= 2 && b >= 2 {}";

        let lex: Vec<_> = Token::lexer(code).spanned().collect();

        println!("lex {:?}", lex);
    }
}