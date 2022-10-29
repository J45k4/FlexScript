


#[derive(Debug, Clone, PartialEq)]
pub enum NonNullType {
    Int,
    Float,
    Bool,
    String,
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct  VarType {
    pub nullable: bool,
    pub array: bool,
    pub typ: NonNullType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub typ: Option<VarType>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Func {
    pub is_async: bool,
    pub params: Vec<Param>,
    pub body: Body,
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
    pub patterns: Vec<Expr>,
    pub body: Body,
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
pub struct Call {
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
    pub vars: Vec<Expr>,
    pub expr: Option<Box<Expr>>,
    pub body: Body,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropAccess {
    pub expr: Box<Expr>,
    pub prop: Box<Expr>,
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
pub enum XmlChild {
    Xml(Xml),
    Expr(Expr),
    Ident(String)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Xml {
    pub name: String,
    pub attrs: Vec<ObjField>,
    pub children: Vec<XmlChild>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    pub items: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectSql {
    pub fields: Vec<Expr>,
    pub from: Option<Expr>,
    pub where_: Option<Expr>,
    pub group_by: Option<Expr>,
    pub having: Option<Expr>,
    pub order_by: Option<Expr>,
    pub limit: Option<Expr>,
    pub offset: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InsertSql {
    pub table: Expr,
    pub fields: Vec<Expr>,
    pub values: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateSql {
    pub table: Expr,
    pub fields: Vec<Expr>,
    pub values: Vec<Expr>,
    pub where_: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sql {
    SelectSql,
    InsertSql,
    UpdateSql
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Const(Const),
    BinOP(BinOP),
    Ident(String),
    If(If),
    ObjExpr(ObjExpr),
    Match(MatchExpr),
    Func(Func),
    For(ForExpr),
    Call(Call),
    Range(RangeExpr),
    PropAccess(PropAccess),
    Assign(Assign),
    Array(Array),
    Xml(Xml),
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
pub struct Assign {
    pub target: Box<Expr>,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    StructStmt(Struct),
    TypeStmt(TypeStmt),
    EnumStmt(EnumStmt),
    FunctionStmt(Func),
    ContinueStmt,
    BreakStmt(Option<Box<Stmt>>),
    ReturnStmt(Option<Box<Stmt>>),
    Literal(Literal),
    Assign(Assign),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstStmt {
    pub ident: String,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Export {
    pub default: bool,
    pub name: Option<String>,
    pub item: Box<Item>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Expr(Expr),
    Assign(Assign),
    Identifier(String),
    Struct(Struct),
    Const(ConstStmt),
    Type(TypeStmt),
    Export(Export),
    Return(Return),
    Literal(Literal),
}

pub type Body = Vec<Item>;

#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
    pub body: Body,
}


pub type Stmts = Vec<Stmt>;