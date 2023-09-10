
#[derive(Debug, PartialEq, Clone)]
pub struct ObjProp {
	pub name: String,
	pub value: HeapValue
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
	ObjRef(usize),
	ListRef(usize),
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
pub enum HeapValue {
	Obj(Obj),
	List(Vec<HeapValue>),
	Fn(usize),
	Ref(usize),
	Int(i64),
	Float(f64),
	Bool(bool),
	Str(String),
	None
}

impl From<StackValue> for HeapValue {
	fn from(val: StackValue) -> Self {
		match val {
			StackValue::Int(i) => Self::Int(i),
			StackValue::Float(f) => Self::Float(f),
			StackValue::Str(s) => Self::Str(s),
			StackValue::Bool(b) => Self::Bool(b),
			StackValue::Ref(r) => Self::Ref(r),
			StackValue::UndefRef(r) => Self::Ref(r),
			StackValue::FnRef(r) => Self::Fn(r),
			StackValue::ArrayIter { i, list_id } => {
				let list = vec![Self::Ref(list_id); i];
				Self::List(list)
			},
			StackValue::Prop { obj_id, prop } => {
				let obj = Obj {
					name: None,
					props: vec![ObjProp {
						name: "prop".to_string(),
						value: Self::Ref(obj_id)
					}]
				};

				Self::Obj(obj)
			},
			StackValue::None => Self::None
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum StackValue {
	Int(i64),
	Float(f64),
	Str(String),
	Bool(bool),
	Ref(usize),
	UndefRef(usize),
	FnRef(usize),
	ArrayIter {
		i: usize,
		list_id: usize
	},
	Prop {
		obj_id: usize,
		prop: usize
	},
	None
}

impl Default for StackValue {
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
	Value(HeapValue),
	Await {
		stack_id: usize,
		value: HeapValue,
	},
	None
}