use crate::Value;


#[derive(Debug, Clone, PartialEq)]
pub enum ByteCode {
    Load(usize),
    LoadConst(usize),
    Store(usize),
    StoreName,
    BinOP,
    BinMul,
    BinAdd,
    BinMinus,
    BinDivide,
    Jump(usize),
    JumpIfFalse(usize),
    Call(usize),
    Cmp,
    BeginScope,
    EndScope,
    Fun(usize),
    MakeStruct,
    MakeArray(usize),
    Obj(usize),
    Assign,
    Ret(usize),
    Var(usize),
    Next,
    MakeIter,
    Await
}

// #[derive(Debug, Clone, PartialEq)]
// pub enum Value {
//     Int(i64),
//     Float(f64),
//     String(String),
//     Bool(bool),
//     None,
//     ExtRef(usize),
//     Ref(usize),
// }

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