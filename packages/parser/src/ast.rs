


#[derive(Debug, Clone, PartialEq)]
pub enum NonNullType {
    Int,
    Float,
    Bool,
    String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct  VarType {
    pub non_null: bool,
    pub array: bool,
    pub typ: NonNullType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {

}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool)
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionStmt {
    pub name: Option<String>,
    pub params: Vec<Param>,
    pub body: Stmts,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FactorStmt {
    pub name: String,
    pub type_: VarType,
    pub value: Option<Literal>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
    pub name: String,
    pub typ: VarType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeField {
    pub ident: String,
    pub typ: VarType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeStmt {
    pub name: String,
    pub fields: Vec<TypeField>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumField {
    pub name: String,
    pub value: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumStmt {
    pub name: String,
    pub fields: Vec<EnumField>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchCase {
    pub pattern: Expr,
    pub body: Stmts,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchExpr {
    pub expr: Box<Expr>,
    pub cases: Vec<MatchCase>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfExpr {
    pub condition: Box<Expr>,
    pub body: Stmts,
    pub else_body: Option<Stmts>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SqlExpr {

}

#[derive(Debug, Clone, PartialEq)]
pub struct XmlExpr {

}

#[derive(Debug, Clone, PartialEq)]
pub struct RangeExpr {
    pub start: Option<Box<Expr>>,
    pub end: Option<Box<Expr>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForExpr {
    pub var: Option<String>,
    pub expr: Option<Box<Expr>>,
    pub body: Stmts,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropertyAccessExpr {
    pub expr: Box<Expr>,
    pub property: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexExpr {
    pub expr: Box<Expr>,
    pub index: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Factor {
    Float(f64),
    Int(i64),
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TermOperator {
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TermRightSide {
    pub val: Box<Factor>,
    pub op: TermOperator,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Term {
    pub left: Factor,
    pub right: Vec<TermRightSide>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprOperator {
    Add,
    Sub,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprRightSide {
    pub val: Box<Term>,
    pub op: ExprOperator,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct ExprTerminal {
    pub val: Box<Expr>,
    pub op: ExprTerminalOperator,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Const {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfBranch {
    pub condition: Expr,
    pub body: Body,
}

#[derive(Debug, Clone, PartialEq)]
pub struct If {
    pub branches: Vec<IfBranch>,
    pub else_body: Body,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjField {
    pub target: String,
    pub value: Expr
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjExpr {
    pub fields: Vec<ObjField>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Const(Const),
    BinOP(BinOP),
    Identifier(String),
    If(If),
    ObjExpr(ObjExpr),
}

pub enum Value {
    Const(Const),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    LogicalAnd,
    LogicalOr,
    Modulus,
    Pow,
    And,
    Or,
    Eq,
    Lt,
    Lte,
    Gt,
    Gte,
    Neq
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinOP {
    pub left: Box<Expr>,
    pub op: Operator,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    StructStmt(Struct),
    TypeStmt(TypeStmt),
    EnumStmt(EnumStmt),
    FunctionStmt(FunctionStmt),
    ContinueStmt,
    BreakStmt(Option<Box<Stmt>>),
    ReturnStmt(Option<Box<Stmt>>),
    Literal(Literal),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assign {
    pub target: Expr,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstStmt {
    pub ident: String,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BodyItem {
    Expr(Expr),
    Assign(Assign),
    Identifier(String),
    Struct(Struct),
    Const(ConstStmt),
    Type(TypeStmt),
}

pub type Body = Vec<BodyItem>;

#[derive(Debug, Clone, PartialEq)]
pub struct CodeFile {
    pub body: Body,
}


pub type Stmts = Vec<Stmt>;

#[derive(Debug, Clone, PartialEq)]
pub struct AST {
    pub stmts: Stmts
}
