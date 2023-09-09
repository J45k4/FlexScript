
#[derive(Debug, PartialEq, Clone)]
pub struct ObjProp {
	pub name: String,
	pub value: Value
}

#[derive(Debug, PartialEq, Clone)]
pub struct Obj {
	pub name: Option<String>,
	pub props: Vec<ObjProp>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Array(Vec<Value>),
	Ptr(usize),
	Fn(usize),
	Obj(Obj),
	ArrayIter {
		inx: usize,
		arr: Vec<Value>
	},
	UndefIdent(usize),
	UndefCall {
		ident: usize,
		args: Vec<Value>
	},
    None,
}

impl Default for Value {
	fn default() -> Self {
		Self::None
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct Assign {
	pub left: Box<ASTNode>,
	pub right: Box<ASTNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
	pub name: String,
	pub value: Box<ASTNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjIns {
	pub name: Option<String>,
	pub props: Vec<Property>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Array {
	pub items: Vec<ASTNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Call {
	pub callee: Box<ASTNode>,
	pub args: Vec<ASTNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VarType {
	Int,
	Float,
	String,
	Var(String),
	StrLit(String),
	FnDef(Fun),
	Ident(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeField {
	pub name: String,
	pub typ: VarType
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructDef {
	pub name: String,
	pub fields: Vec<TypeField>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Fun {
	pub params: Vec<ASTNode>,
	pub body: Vec<ASTNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeDef {
	pub name: String,
	pub fields: Vec<TypeField>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Var {
	pub name: String,
	pub typ: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ProbAccess {
	pub object: Box<ASTNode>,
	pub property: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
	Plus,
	Minus,
	Mul,
	Div,
	Eq,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinOp {
	pub left: Box<ASTNode>,
	pub op: Op,
	pub right: Box<ASTNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ret {
	pub value: Box<Option<ASTNode>>
}

#[derive(Debug, PartialEq, Clone)]
pub struct If {
	pub cond: Box<ASTNode>,
	pub body: Vec<ASTNode>,
	pub els: Option<Vec<ASTNode>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ForCond {
	None,
	FromIt {
		ident: String,
		it: Box<ASTNode>,
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct For {
	pub cond: ForCond,
	pub body: Vec<ASTNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
	Ident(String),
	Assign(Assign),
	ObjIns(ObjIns),
	Array(Array),
	Call(Call),
	Property(String, Box<ASTNode>),
	Lit(Value),
	LiteralPercent(f64),
	Fun(Fun),
	StructDef(StructDef),
	TypeDef(TypeDef),
	Var(Var),
	ProbAccess(ProbAccess),
	Ret(Ret),
	BinOp(BinOp),
	If(If),
	For(For),
}

#[derive(Debug, PartialEq, Clone)]
pub enum RunResult {
	Value(Value),
	Await {
		stack_id: usize,
		value: Value,
	},
	None
}