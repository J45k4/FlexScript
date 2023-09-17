use crate::StackValue;

#[derive(Debug, PartialEq, Clone)]
pub enum BuildIn {
    Map {
        obj: u32,
        inx: u32,
        blk: u32,
    },
    None
}

impl Default for BuildIn {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Default)]
pub struct Call {
    pub blk: u32,
    pub pc: u32,
    pub scope_id: u32,
    pub values: Vec<StackValue>,
    pub buildin: BuildIn,
}

impl Call {
    pub fn push(&mut self, val: StackValue) {
        self.values.push(val);
    }

    pub fn pop(&mut self) -> Option<StackValue> {
        self.values.pop()
    }

    pub fn peek(&self) -> Option<&StackValue> {
        self.values.last()
    }
}

#[derive(Debug, Default)]
pub struct Callstack {
    pub log: usize,
    stack: Vec<Call>,
}

impl Callstack {
    pub fn new() -> Self {
        Self {
            log: 0,
            stack: vec![],
        }
    }

    pub fn push_value(&mut self, val: StackValue) {
        self.stack.last_mut().unwrap().push(val);
    }
 
    pub fn pop_value(&mut self) -> Option<StackValue> {
        self.stack.last_mut().unwrap().pop()
    }

    pub fn peek_value(&self) -> Option<&StackValue> {
        self.stack.last().unwrap().peek()
    }

    pub fn peek_mut_value(&mut self) -> Option<&mut StackValue> {
        self.stack.last_mut().unwrap().values.last_mut()
    }

    pub fn push(&mut self, call: Call) {
        self.stack.push(call);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn pc(&self) -> u32 {
        self.stack.last().unwrap().pc
    }

    pub fn blk(&self) -> Option<u32> {
        match self.stack.last() {
            Some(call) => Some(call.blk),
            None => None,
        }
    }

    pub fn scope_id(&self) -> u32 {
        self.stack.last().unwrap().scope_id
    }

    pub fn increment_pc(&mut self) {
        self.stack.last_mut().unwrap().pc += 1;
    }

    pub fn set_pc(&mut self, pc: u32) {
        self.stack.last_mut().unwrap().pc = pc;
    }

    pub fn set_scope_id(&mut self, scope_id: u32) {
        self.stack.last_mut().unwrap().scope_id = scope_id;
    }

    pub fn set_buildin(&mut self, buildin: BuildIn) {
        self.stack.last_mut().unwrap().buildin = buildin;
    }

    pub fn get_buildin(&mut self) -> &mut BuildIn {
        &mut self.stack.last_mut().unwrap().buildin
    }

    pub fn depth(&self) -> usize {
        self.stack.len()
    }
}