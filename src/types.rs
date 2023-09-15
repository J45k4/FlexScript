
#[derive(Debug, PartialEq, Clone)]
pub enum StackValue {
	Int(i64),
	Float(f64),
	Str(String),
	Bool(bool),
	Ref(u32),
	Undef(u32),
	Fn(u32),
	UndefCall {
		ident: u32,
		args: Vec<StackValue>
	},
	None,
}

impl Default for StackValue {
	fn default() -> Self {
		Self::None
	}
}

impl From<&Value> for StackValue {
	fn from(val: &Value) -> Self {
		match val {
			Value::Int(i) => Self::Int(*i),
			Value::Float(f) => Self::Float(*f),
			Value::Str(s) => Self::Str(s.clone()),
			Value::Bool(b) => Self::Bool(*b),
			Value::Ptr(r) => Self::Ref(*r),
			Value::UndefIdent(u) => Self::Undef(*u),
			Value::Fn(f) => Self::Fn(*f),
			Value::None => Self::None,
			_ => panic!("Cannot convert value to stack value")
		}
	}
}

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
    List(Vec<Value>),
	Ptr(u32),
	Fn(u32),
	Obj(Obj),
	ListIter {
		inx: u32,
		id: u32
	},
	UndefIdent(u32),
	UndefCall {
		ident: u32,
		args: Vec<Value>
	},
    None,
}

impl Default for Value {
	fn default() -> Self {
		Self::None
	}
}

impl From<StackValue> for Value {
	fn from(val: StackValue) -> Self {
		match val {
			StackValue::Int(i) => Self::Int(i),
			StackValue::Float(f) => Self::Float(f),
			StackValue::Str(s) => Self::Str(s),
			StackValue::Bool(b) => Self::Bool(b),
			StackValue::Ref(r) => Self::Ptr(r),
			StackValue::Undef(u) => Self::UndefIdent(u),
			StackValue::Fn(f) => Self::Fn(f),
			StackValue::None => Self::None,
			_ => todo!("{:?}", val)
		}
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
	Value {
		value: Value,
		scope_id: u32,
	},
	Await {
		stack_id: usize,
		value: Value,
	},
	None
}