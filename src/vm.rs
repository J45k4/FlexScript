use std::collections::HashMap;

use crate::ASTNode;
use crate::Op;
use crate::Value;
use crate::vm_types::ByteCode;
use crate::vm_types::File;
use crate::vm_types::SideEffect;

#[derive(Debug)]
struct Scope {
    scopes: Vec<HashMap<usize, Value>>
}

impl Scope {
    fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn insert(&mut self, id: usize, val: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(id, val);
        }
    }

    fn get(&self, var: &usize) -> Option<&Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(var) {
                return Some(val);
            }
        }
        None
    }
}

#[derive(Debug)]
struct CallItem {
    blk: usize,
    pc: usize,
    args: Vec<Value>
}

pub struct Vm {
    stack: Vec<Value>,
    stack_mem: Vec<Value>,
    files: Vec<File>,
    scopes: Vec<usize>,
    constants: Vec<Value>,
    code_blocks: Vec<Vec<ByteCode>>,
    call_stack: Vec<CallItem>,
    idt_map: HashMap<String, usize>,
    id_idt_map: HashMap<usize, String>,
    next_idt: usize,
    scope: Scope,
    pub log: usize
}

impl Vm {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            stack_mem: Vec::new(),
            files: Vec::new(),
            scopes: Vec::new(),
            constants: Vec::new(),
            code_blocks: Vec::new(),
            call_stack: Vec::new(),
            scope: Scope::new(),
            idt_map: HashMap::new(),
            id_idt_map: HashMap::new(),
            next_idt: 0,
            log: 0
        }
    }

    pub fn add_file(&mut self, ast: &Vec<ASTNode>) -> usize {
        let mut code_block = Vec::new();

        for node in ast {
            self.compile_node(&mut code_block, node);
        }

        self.code_blocks.push(code_block);
        
        self.code_blocks.len() - 1
    }

    pub fn compile_node(&mut self, block: &mut Vec<ByteCode>, node: &ASTNode) {
        match node {
            ASTNode::Lit(lit) => {
                let i = self.store_const(lit.clone());
                block.push(ByteCode::LoadConst(i));
            },
            ASTNode::Ret(ret) => {
                let a = match &*ret.value {
                    Some(v) =>  {
                        self.compile_node(block, v);
                        1
                    },
                    None => 0,
                };

                block.push(ByteCode::Ret(a));
            },
            ASTNode::BinOp(binop) => {
                self.compile_node(block, &binop.left);
                self.compile_node(block, &binop.right);

                match binop.op {
                    Op::Plus => block.push(ByteCode::BinAdd),
                    Op::Minus => block.push(ByteCode::BinMinus),
                    Op::Mul => block.push(ByteCode::BinMul),
                    Op::Div => block.push(ByteCode::BinDivide),
                    Op::Eq => block.push(ByteCode::Cmp),
                    // Op::Neq => block.push(ByteCode::Cmp),
                    // Op::Lt => block.push(ByteCode::Cmp),
                    // Op::Gt => block.push(ByteCode::Cmp),
                    // Op::Leq => block.push(ByteCode::Cmp),
                    // Op::Geq => block.push(ByteCode::Cmp),
                    _ => panic!("Invalid operation")
                }
            },
            ASTNode::If(ifs) => {
                self.compile_node(block, &ifs.cond);
                block.push(ByteCode::JumpIfFalse(0));
                let jump_if_false_inx = block.len() - 1;

                for node in &ifs.body {
                    self.compile_node(block, node);
                }

                block[jump_if_false_inx] = ByteCode::JumpIfFalse(block.len());

                if let Some(els) = &ifs.els {
                    for node in els {
                        self.compile_node(block, node);
                    }
                }
            },
            ASTNode::Assign(a) => {
                self.compile_node(block, &a.right);
                match &*a.left {
                    ASTNode::Ident(ref name) => {
                        let i = self.store_idt(name.clone());
                        block.push(ByteCode::Store(i));
                    },
                    _ => panic!("Invalid operation")
                }
            },
            ASTNode::Ident(idt) => {
                let i = self.store_idt(idt.clone());
                block.push(ByteCode::Load(i));
            },
            ASTNode::Array(a) => {
                for item in &a.items {
                    self.compile_node(block, item);
                }
                block.push(ByteCode::MakeArray(a.items.len()));
            },
            ASTNode::Fun(f) => {
                let mut fun_block = Vec::new();
                for node in &f.body {
                    self.compile_node(&mut fun_block, node);
                }
                self.code_blocks.push(fun_block);
                block.push(ByteCode::Fun(self.code_blocks.len() - 1));
            },
            ASTNode::Call(c) => {
                for arg in &c.args {
                    self.compile_node(block, arg);
                }
                self.compile_node(block, &c.callee);
                block.push(ByteCode::Call(c.args.len()));
            },
            _ => todo!("{:?}", node)
        }
    }

    pub fn run(&mut self, blk: usize, args: Vec<Value>) -> Value {
        if self.log > 0 {
            println!("run: {} {:?}", blk, args);
        }

        let mut current = CallItem {
            args: args,
            blk,
            pc: 0
        };
        let code_block = &self.code_blocks[blk];
        let len = code_block.len();

        loop {
            while current.pc < len {
                let pc = current.pc;
                current.pc += 1;

                let c = &self.code_blocks[current.blk][pc];

                if self.log > 0 {
                    print!("pc: {}, code: {:?}", pc, c);

                    match c {
                        ByteCode::Load(i) => print!(" {}", self.id_idt_map.get(&i).unwrap()),
                        ByteCode::Store(i) => print!(" {}", self.id_idt_map.get(&i).unwrap()),
                        _ => {}
                    }

                    println!("");
                }

                match c {
                    ByteCode::Load(i) => {
                        let v = match self.scope.get(i){
                            Some(v) => v.clone(),
                            None => panic!("variable not found")
                        };
                        self.stack.push(v);
                    },
                    ByteCode::Store(i) => {
                        let v = self.stack.pop().unwrap();
                        self.scope.insert(*i, v);
                    },
                    ByteCode::BinMul |
                    ByteCode::BinAdd |
                    ByteCode::BinMinus |
                    ByteCode::BinDivide => {
                        let tos = self.stack.pop().unwrap();
                        let tos1 = self.stack.pop().unwrap();

                        let v = match (tos, tos1) {
                            (Value::Int(a), Value::Int(b)) => {
                                match c {
                                    ByteCode::BinMul => Value::Int(a * b),
                                    ByteCode::BinAdd => Value::Int(a + b),
                                    ByteCode::BinMinus => Value::Int(a - b),
                                    ByteCode::BinDivide => Value::Int(a / b),
                                    _ => panic!("Invalid operation")
                                }
                            },
                            (Value::Float(a), Value::Float(b)) => {
                                match c {
                                    ByteCode::BinMul => Value::Float(a * b),
                                    ByteCode::BinAdd => Value::Float(a + b),
                                    ByteCode::BinMinus => Value::Float(a - b),
                                    ByteCode::BinDivide => Value::Float(a / b),
                                    _ => panic!("Invalid operation")
                                }
                            },
                            (Value::Float(a), Value::Int(b)) => {
                                match c {
                                    ByteCode::BinMul => Value::Float(a * b as f64),
                                    ByteCode::BinAdd => Value::Float(a + b as f64),
                                    ByteCode::BinMinus => Value::Float(a - b as f64),
                                    ByteCode::BinDivide => Value::Float(a / b as f64),
                                    _ => panic!("Invalid operation")
                                }
                            },
                            (Value::Int(a), Value::Float(b)) => {
                                match c {
                                    ByteCode::BinMul => Value::Float(a as f64 * b),
                                    ByteCode::BinAdd => Value::Float(a as f64 + b),
                                    ByteCode::BinMinus => Value::Float(a as f64 - b),
                                    ByteCode::BinDivide => Value::Float(a as f64 / b),
                                    _ => panic!("Invalid operation")
                                }
                            },
                            _ => panic!("Invalid operation")
                        };

                        self.stack.push(v);
                    },
                    ByteCode::Jump => {
                        // self.pc = ins.arg;

                        // log::debug!("jumping to {}", self.pc);
                    },
                    ByteCode::JumpIfFalse(inx) => {
                        let v = self.stack.pop().unwrap();

                        match v {
                            Value::Bool(b) => {
                                if !b {
                                    current.pc = *inx;
                                }
                            },
                            _ => panic!("Invalid operation")
                        }
                    },
                    ByteCode::Call(arg_count) => {
                        let mut args = Vec::new();

                        for _ in 0..*arg_count {
                            let v = self.stack.pop().unwrap();
                            args.push(v);
                        }

                        args.reverse();

                        let callee = self.stack.pop().unwrap();

                        match callee {
                            Value::Fn(i) => {
                                match self.run(i, args) {
                                    Value::None => {},
                                    v => self.stack.push(v)
                                }
                            },
                            _ => panic!("Invalid operation")
                        }
                    },
                    ByteCode::Cmp => {
                        let tos = self.stack.pop().unwrap();
                        let tos1 = self.stack.pop().unwrap();
                        
                        let v = match (tos, tos1) {
                            (Value::Int(a), Value::Int(b)) => Value::Bool(a == b),
                            (Value::Float(a), Value::Float(b)) => Value::Bool(a == b),
                            (Value::Float(a), Value::Int(b)) => Value::Bool(a == b as f64),
                            (Value::Int(a), Value::Float(b)) => Value::Bool(a as f64 == b),
                            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a == b),
                            (Value::Str(a), Value::Str(b)) => Value::Bool(a == b),
                            _ => panic!("Invalid operation")
                        };

                        self.stack.push(v);
                    },
                    ByteCode::BeginScope => {
                        self.scopes.push(self.stack_mem.len());
                    },
                    ByteCode::EndScope => {
                        let scope_start = self.scopes.pop().unwrap();
                        self.stack_mem.truncate(scope_start);
                    },
                    ByteCode::LoadConst(a) => {
                        let v = self.constants[*a].clone();
                        self.stack.push(v);
                    },
                    ByteCode::StoreName => todo!(),
                    ByteCode::BinOP => todo!(),
                    ByteCode::MakeStruct => todo!(),
                    ByteCode::MakeArray(len) => {
                        let mut items = vec![];

                        for _ in 0..*len {
                            let v = self.stack.pop().unwrap();
                            items.push(v);
                        }

                        items.reverse();

                        self.stack.push(Value::Array(items));
                    },
                    ByteCode::Obj => todo!(),
                    ByteCode::Assign => todo!(),
                    ByteCode::Ret(c) => {
                        return match self.stack.pop() {
                            Some(v) => v,
                            None => Value::None
                        };
                    },
                    ByteCode::Fun(i) => {
                        self.stack.push(Value::Fn(*i));
                    },
                    _ => todo!("{:?}", c)
                };
            }

            match self.call_stack.pop() {
                Some(item) => {
                    current = item;
                },
                None => break
            }
        }

        Value::None
    }

    pub fn store_const(&mut self, v: Value) -> usize {
        self.constants.push(v);

        self.constants.len() - 1
    }

    pub fn store_idt(&mut self, name: String) -> usize {
        match self.idt_map.get(&name) {
            Some(i) => *i,
            None => {
                let i = self.next_idt;
                self.next_idt += 1;
                self.idt_map.insert(name.clone(), i);
                self.id_idt_map.insert(i, name);
                i
            }
        }
    }

    pub fn push(&mut self, v: Value) {
        self.stack.push(v);
    }

    // fn store(&mut self, index: usize, value: Value) {
    //     self.scopes[self.current].values[index] = value;
    // }
}

#[cfg(test)]
mod tests {
    use crate::Array;
    use crate::Assign;
    use crate::BinOp;
    use crate::Call;
    use crate::Fun;
    use crate::Op;
    use crate::Ret;

    use super::*;

    #[test]
    fn test_return_number() {
        let mut vm = Vm::new();
        let ret = ASTNode::Ret(
            Ret {
                value: Box::new(Some(ASTNode::Lit(Value::Int(1))))
            }
        );
        let file = vm.add_file(&vec![ret]);
        let val = vm.run(file, vec![]);

        assert_eq!(val, Value::Int(1));
    }

    #[test]
    fn simple_expr() {
        let mut vm = Vm::new();
        let ret = ASTNode::Ret(
            Ret {
                value: Box::new(Some(
                    ASTNode::BinOp(
                        BinOp {
                            left: Box::new(ASTNode::Lit(Value::Int(1))),
                            op: Op::Plus,
                            right: Box::new(ASTNode::Lit(Value::Int(1)))
                        }
                    )
                ))
            }
        );
        let block = vm.add_file(&vec![ret]);
        let val = vm.run(block, vec![]);
        assert_eq!(val, Value::Int(2));
    }

    #[test]
    fn simple_comparsion() {
        let mut vm = Vm::new();
        let ret = ASTNode::Ret(
            Ret {
                value: Box::new(Some(
                    ASTNode::BinOp(
                        BinOp {
                            left: Box::new(ASTNode::Lit(Value::Int(1))),
                            op: Op::Eq,
                            right: Box::new(ASTNode::Lit(Value::Int(1)))
                        }
                    )
                ))
            }
        );
        let block = vm.add_file(&vec![ret]);
        let val = vm.run(block, vec![]);
        assert_eq!(val, Value::Bool(true));
    }

    #[test]
    fn simple_if_true() {
        let mut vm = Vm::new();
        let ret = ASTNode::Ret(
            Ret {
                value: Box::new(Some(
                    ASTNode::If(
                        crate::If {
                            cond: Box::new(ASTNode::Lit(Value::Bool(true))),
                            body: vec![
                                ASTNode::Ret(
                                    Ret {
                                        value: Box::new(Some(ASTNode::Lit(Value::Int(1))))
                                    }
                                )
                            ],
                            els: None
                        }
                    )
                ))
            }
        );
        let block = vm.add_file(&vec![ret]);
        let val = vm.run(block, vec![]);
        assert_eq!(val, Value::Int(1));
    }

    #[test]
    fn simple_if_false() {
        let mut vm = Vm::new();
        let ret = ASTNode::Ret(
            Ret {
                value: Box::new(Some(
                    ASTNode::If(
                        crate::If {
                            cond: Box::new(ASTNode::Lit(Value::Bool(false))),
                            body: vec![
                                ASTNode::Ret(
                                    Ret {
                                        value: Box::new(Some(ASTNode::Lit(Value::Int(1))))
                                    }
                                )
                            ],
                            els: None
                        }
                    )
                ))
            }
        );
        let block = vm.add_file(&vec![ret]);
        let val = vm.run(block, vec![]);
        assert_eq!(val, Value::None);
    }

    #[test]
    fn assign_to_var() {
        let mut vm = Vm::new();
        let assign = ASTNode::Assign(
            Assign {
                left: Box::new(ASTNode::Ident("a".to_string())),
                right: Box::new(ASTNode::Lit(Value::Int(1)))
            }
        );

        let ret = ASTNode::Ret(
            Ret {
                value: Box::new(Some(
                    ASTNode::Ident("a".to_string())
                ))
            }
        );
        let block = vm.add_file(&vec![assign, ret]);
        let val = vm.run(block, vec![]);
        assert_eq!(val, Value::Int(1));
    }

    #[test]
    fn simple_array() {
        let mut vm = Vm::new();
        let ret = ASTNode::Ret(
            Ret {
                value: Box::new(Some(
                    ASTNode::Array(
                        Array {
                            items: vec![
                                ASTNode::Lit(Value::Int(1)),
                                ASTNode::Lit(Value::Int(2)),
                                ASTNode::Lit(Value::Int(3)),
                            ]
                        }
                    )
                ))
            }
        );
        let block = vm.add_file(&vec![ret]);
        let val = vm.run(block, vec![]);
        assert_eq!(val, Value::Array(vec![
            Value::Int(1),
            Value::Int(2),
            Value::Int(3),
        ]));
    }

    #[test]
    fn function_calling() {
        let mut vm = Vm::new();
        let fun = ASTNode::Assign(
            Assign {
                left: Box::new(ASTNode::Ident("a".to_string())),
                right: Box::new(ASTNode::Fun(
                    Fun {
                        params: vec![],
                        body: vec![
                            ASTNode::Ret(
                                Ret {
                                    value: Box::new(Some(
                                        ASTNode::Lit(Value::Int(1))
                                    ))
                                }
                            )
                        ]  
                    }
                ))
            }
        );

        let ret = ASTNode::Ret(
            Ret {
                value: Box::new(Some(
                    ASTNode::Call(
                        Call {
                            callee: Box::new(ASTNode::Ident("a".to_string())),
                            args: vec![]
                        }
                    )
                ))
            }
        );
        let block = vm.add_file(&vec![fun, ret]);
        let val = vm.run(block, vec![]);
        assert_eq!(val, Value::Int(1));
    }
}