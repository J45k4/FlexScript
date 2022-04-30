
#[derive(Debug, PartialEq)]
pub struct FunStmt {
    pub name: String
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    For,
    If,
    Match,
    Return,
    While,
    Break,
    Pass,
    Continue,
    Fun(FunStmt)
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    Div,
    Mod
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    BoolOp,
    NamedExpr,
    BinOp,
    IfExp(Box<Expr>, Box<Expr>, Box<Expr>)
}

#[derive(Debug, PartialEq)]
pub enum CodeFileItem {
    Stmt(Stmt),
    Expr(Expr)
}

#[derive(Debug, PartialEq)]
pub struct CodeFile {
    pub body: Vec<CodeFileItem>
}

#[derive(Debug, PartialEq)]
pub enum AstItem {
    Stmt(Stmt),
    Expr(Expr),
    CodeFile(CodeFile),
    CodeFileItem(CodeFileItem)
}

#[derive(Debug, PartialEq)]
pub struct Ast {
    pub body: Vec<AstItem>
}