
pub enum ByteCode {
    Load,
    Store,
    BinMul,
    BinAdd,
    BinMinus,
    BinDivide,
    Jump,
    JumpIfFalse,
    Call,
    CmpEq,
    BeginScope,
    EndScope,
}

pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    None,
    ExtRef(usize),
    Ref(usize),
}

pub struct Ins {
    pub code: ByteCode,
    pub arg: usize,
}

pub struct Scope {
    values: Vec<Value>,
    scopes: Vec<Scope>
}

pub struct File {
    pub instructions: Vec<Ins>
}

pub struct ScopeItem {
    pub inx: usize,
    pub value: Value
}

pub enum SideEffect {
    Call {
        r: usize,
        args: Vec<Value>
    }
}

pub const SMALLER_THAN_OP: u32 = 0;
pub const GREATER_THAN_OP: u32 = 1;
pub const EQUAL_TO_OP: u32 = 2;
pub const NOT_EQUAL_TO_OP: u32 = 3;
pub const GREATER_THAN_EQUAL_TO_OP: u32 = 4;
pub const SMALLER_THAN_EQUAL_TO_OP: u32 = 5;
pub const LOGICAL_AND: u32 = 6;
pub const LOGICAL_OR: u32 = 7;