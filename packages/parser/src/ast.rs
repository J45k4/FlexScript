
pub struct Param {

}

pub struct FunctionStmt {
    pub name: Option<String>,
    pub params: Vec<Param>,
    pub body: Stmts,
}

pub struct StructStmt {
    pub name: Option<String>,
    pub fields: Vec<Param>,
}

pub struct TypeStmt {
    pub name: Option<String>,
    pub fields: Vec<Param>,
}

pub struct EnumStmt {

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
    
}

pub enum Expr {
    MatchExpr(MatchExpr),
    IfExpr(IfExpr),
}

pub enum Stmt {
    Expr(Expr),
}

type Stmts = Vec<Stmt>;

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