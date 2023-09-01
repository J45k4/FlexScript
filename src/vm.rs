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
}

pub struct Vm {
    pc: usize,
    stack: Vec<Value>,
    stack_mem: Vec<Value>,
    files: Vec<File>,
    scopes: Vec<usize>,
    constants: Vec<Value>,
    code_blocks: Vec<Vec<ByteCode>>,
    call_stack: Vec<CallItem>,
    idt_map: HashMap<String, usize>,
    next_idt: usize,
    scope: Scope,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            pc: 0,
            stack: Vec::new(),
            stack_mem: Vec::new(),
            files: Vec::new(),
            scopes: Vec::new(),
            constants: Vec::new(),
            code_blocks: Vec::new(),
            call_stack: Vec::new(),
            scope: Scope::new(),
            idt_map: HashMap::new(),
            next_idt: 0,
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
            _ => todo!("{:?}", node)
        }
    }

    pub fn run(&mut self, block_id: usize) -> Vec<SideEffect> {
        let code_block = &self.code_blocks[block_id];

        let len = code_block.len();

        let mut side_effects = Vec::new();

        while self.pc < len {
            let pc = self.pc;
            self.pc += 1;

            let c = &code_block[pc];

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
                                self.pc = *inx;
                            }
                        },
                        _ => panic!("Invalid operation")
                    }
                },
                ByteCode::Call => {
                    // let mut args = vec![];
    
                    // for _ in 0..ins.arg {
                    //     let v = self.stack.pop().unwrap();
                    //     args.push(v);
                    // }

                    // let v = self.stack.pop().unwrap();

                    // match v {
                    //     Value::ExtRef(r) => {
                    //         let call = SideEffect::Call {
                    //             r,
                    //             args
                    //         };

                    //         side_effects.push(call);
                    //     },
                    //     _ => panic!("Invalid call")
                    // }
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
                ByteCode::MakeFunction => todo!(),
                ByteCode::MakeStruct => todo!(),
                ByteCode::MakeArray => todo!(),
                ByteCode::Obj => todo!(),
                ByteCode::Assign => todo!(),
                ByteCode::Ret(c) => {
                    let v = match self.stack.pop() {
                        Some(v) => v,
                        None => Value::None
                    };

                    let side_effect = SideEffect::Return {
                        value: v
                    };

                    side_effects.push(side_effect);
                },
                _ => todo!()
            };

            if side_effects.len() > 0 {
                break;
            }
        }

        side_effects
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
                self.idt_map.insert(name, i);
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
    use crate::Assign;
    use crate::BinOp;
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
        let side_effects = vm.run(file);

        assert_eq!(side_effects.len(), 1);

        let effect = &side_effects[0];
        let expeted = SideEffect::Return {
            value: Value::Int(1)
        };

        assert_eq!(effect, &expeted);
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
        let side_effects = vm.run(block);

        assert_eq!(side_effects.len(), 1);

        let effect = &side_effects[0];
        let expeted = SideEffect::Return {
            value: Value::Int(2)
        };

        assert_eq!(effect, &expeted);
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
        let side_effects = vm.run(block);

        assert_eq!(side_effects.len(), 1);

        let effect = &side_effects[0];
        let expeted = SideEffect::Return {
            value: Value::Bool(true)
        };

        assert_eq!(effect, &expeted);
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
        let side_effects = vm.run(block);

        assert_eq!(side_effects.len(), 1);

        let effect = &side_effects[0];
        let expeted = SideEffect::Return {
            value: Value::Int(1)
        };

        assert_eq!(effect, &expeted);
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
        let side_effects = vm.run(block);

        assert_eq!(side_effects.len(), 1);

        let effect = &side_effects[0];
        let expeted = SideEffect::Return {
            value: Value::None
        };

        assert_eq!(effect, &expeted);
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
        let side_effects = vm.run(block);

        assert_eq!(side_effects.len(), 1);

        let effect = &side_effects[0];
        let expeted = SideEffect::Return {
            value: Value::Int(1)
        };

        assert_eq!(effect, &expeted);
    }
}