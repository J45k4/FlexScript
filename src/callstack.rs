use crate::StackValue;

#[derive(Debug, Default)]
pub struct Call {
    pub blk: usize,
    pub pc: usize,
    pub scope_id: usize,
    pub args: Vec<StackValue>,
    pub values: Vec<StackValue>,
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
        if self.log > 0 {
            println!("Push: {:?}", call);
        }

        self.stack.push(call);
    }

    pub fn pop(&mut self) {
        if self.log > 0 {
            println!("Pop: {:?}", self.stack.last().unwrap());
        }

        self.stack.pop();
    }

    pub fn pc(&self) -> usize {
        self.stack.last().unwrap().pc
    }

    pub fn blk(&self) -> Option<usize> {
        match self.stack.last() {
            Some(call) => Some(call.blk),
            None => None,
        }
    }

    pub fn scope_id(&self) -> usize {
        self.stack.last().unwrap().scope_id
    }

    pub fn increment_pc(&mut self) {
        self.stack.last_mut().unwrap().pc += 1;
    }

    pub fn set_pc(&mut self, pc: usize) {
        self.stack.last_mut().unwrap().pc = pc;
    }

    pub fn set_scope_id(&mut self, scope_id: usize) {
        self.stack.last_mut().unwrap().scope_id = scope_id;
    }
}