


pub enum NonNullType {
    Int,
    Float,
    Bool,
    String,
}

pub struct  VarType {
    pub non_null: bool,
    pub array: bool,
    pub type_: NonNullType,
}

pub struct Param {

}

pub enum Literal {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool)
}

pub struct FunctionStmt {
    pub name: Option<String>,
    pub params: Vec<Param>,
    pub body: Stmts,
}

pub struct FactorStmt {
    pub name: String,
    pub type_: VarType,
    pub value: Option<Literal>,
}

pub struct StructField {
    pub name: String,
    pub type_: VarType,
}

pub struct StructStmt {
    pub name: String,
    pub fields: Vec<StructField>,
}

pub struct TypeStmt {
    pub name: String,
    pub fields: Vec<Param>,
}

pub struct EnumField {
    pub name: String,
    pub value: Option<Expr>,
}

pub struct EnumStmt {
    pub name: String,
    pub fields: Vec<EnumField>,
}

pub struct MatchCase {
    pub pattern: Expr,
    pub body: Stmts,
}

pub struct MatchExpr {
    pub expr: Box<Expr>,
    pub cases: Vec<MatchCase>,
}

pub struct IfExpr {
    pub condition: Box<Expr>,
    pub body: Stmts,
    pub else_body: Option<Stmts>,
}

pub struct CallExpr {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
}

pub struct SqlExpr {

}

pub struct XmlExpr {

}

pub struct RangeExpr {
    pub start: Option<Box<Expr>>,
    pub end: Option<Box<Expr>>,
}

pub struct ForExpr {
    pub var: Option<String>,
    pub expr: Option<Box<Expr>>,
    pub body: Stmts,
}

pub struct PropertyAccessExpr {
    pub expr: Box<Expr>,
    pub property: String,
}

pub struct IndexExpr {
    pub expr: Box<Expr>,
    pub index: Box<Expr>,
}

pub enum Factor {
    Float(f64),
    Int(i64),
    Bool(bool),
    String(String),
}

pub enum TermOperator {
    Mul,
    Div,
    Mod,
}

pub struct TermRightSide {
    pub val: Box<Factor>,
    pub op: TermOperator,
}

pub struct Term {
    pub left: Factor,
    pub right: Vec<TermRightSide>,
}

pub enum ExprOperator {
    Add,
    Sub,
}

pub struct ExprRightSide {
    pub val: Box<Term>,
    pub op: ExprOperator,
}

pub enum ExprTerminalOperator {
    And,
    Or,
    Eq,
    Lt,
    Lte,
    Gt,
    Gte,
    Neq
}

pub struct ExprTerminal {
    pub val: Box<Expr>,
    pub op: ExprTerminalOperator,
}

pub struct Expr {
    pub left: Term,
    pub right: Vec<ExprRightSide>,
    pub terminal: Option<ExprTerminal>,
}

pub enum Stmt {
    Expr(Expr),
    StructStmt(StructStmt),
    TypeStmt(TypeStmt),
    EnumStmt(EnumStmt),
    FunctionStmt(FunctionStmt),
    ContinueStmt,
    BreakStmt(Option<Box<Stmt>>),
    ReturnStmt(Option<Box<Stmt>>),
    Literal(Literal),
}

pub type Stmts = Vec<Stmt>;

pub struct AST {
    pub stmts: Stmts
}

// #[derive(Debug, PartialEq)]
// pub struct FunStmt {
//     pub name: String
// }

// #[derive(Debug, PartialEq)]
// pub enum Stmt {
//     For,
//     If,
//     Match,
//     Return,
//     While,
//     Break,
//     Pass,
//     Continue,
//     Fun(FunStmt)
// }

// #[derive(Debug, PartialEq)]
// pub enum Operator {
//     Add,
//     Sub,
//     Mult,
//     Div,
//     Mod
// }

// #[derive(Debug, PartialEq)]
// pub enum Expr {
//     BoolOp,
//     NamedExpr,
//     BinOp,
//     IfExp(Box<Expr>, Box<Expr>, Box<Expr>)
// }

// #[derive(Debug, PartialEq)]
// pub enum CodeFileItem {
//     Stmt(Stmt),
//     Expr(Expr)
// }

// #[derive(Debug, PartialEq)]
// pub struct CodeFile {
//     pub body: Vec<CodeFileItem>
// }

// #[derive(Debug, PartialEq)]
// pub enum AstItem {
//     Stmt(Stmt),
//     Expr(Expr),
//     CodeFile(CodeFile),
// }

// #[derive(Debug, PartialEq)]
// pub struct Ast {
//     pub body: Vec<AstItem>
// }