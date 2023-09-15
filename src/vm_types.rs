use crate::Value;


#[derive(Debug, Clone, PartialEq)]
pub enum ByteCode {
    Load(u32),
    LoadConst(u32),
    Store(u32),
    BinMul,
    BinAdd,
    BinMinus,
    BinDivide,
    Jump(u32),
    JumpIfFalse(u32),
    Call(u32),
    Cmp,
    BeginScope,
    EndScope,
    Fun(u32),
    MakeStruct,
    MakeArray(u32),
    Obj(u32),
    Assign,
    Ret(u32),
    Var(u32),
    Next,
    MakeIter,
    Await
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

#[derive(Debug, Clone, PartialEq)]
pub enum SideEffect {
    Call {
        r: usize,
        args: Vec<Value>
    },
    Return {
        value: Value
    }
}