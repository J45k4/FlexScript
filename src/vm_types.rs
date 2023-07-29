
pub enum ByteCode {
    Load,
    LoadConst,
    Store,
    StoreName,
    BinOP,
    BinMul,
    BinAdd,
    BinMinus,
    BinDivide,
    Jump,
    JumpIfFalse,
    Call,
    Cmp,
    BeginScope,
    EndScope,
    MakeFunction,
    MakeStruct,
    MakeArray,
    Obj,
    Assign
}

#[derive(Debug, Clone, PartialEq)]
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

pub const SMALLER_THAN_OP: usize = 0;
pub const GREATER_THAN_OP: usize = 1;
pub const EQUAL_TO_OP: usize = 2;
pub const NOT_EQUAL_TO_OP: usize = 3;
pub const GREATER_THAN_EQUAL_TO_OP: usize = 4;
pub const SMALLER_THAN_EQUAL_TO_OP: usize = 5;
pub const LOGICAL_AND: usize = 6;
pub const LOGICAL_OR: usize = 7;
pub const ADD_OP: usize = 8;
pub const SUB_OP: usize = 9;
pub const MUL_OP: usize = 10;
pub const DIV_OP: usize = 11;
pub const MOD_OP: usize = 12;