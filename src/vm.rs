use std::collections::HashMap;

use crate::ASTNode;
use crate::Value;
use crate::vm_types::ByteCode;
use crate::vm_types::File;
use crate::vm_types::Ins;
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
    current: usize,
    stack: Vec<Value>,
    stack_mem: Vec<Value>,
    files: Vec<File>,
    scopes: Vec<usize>,
    constants: Vec<Value>,
    code_blocks: Vec<Vec<ByteCode>>,
    call_stack: Vec<CallItem>,
    scope: Scope,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            pc: 0,
            current: 0,
            stack: Vec::new(),
            stack_mem: Vec::new(),
            files: Vec::new(),
            scopes: Vec::new(),
            constants: Vec::new(),
            code_blocks: Vec::new(),
            call_stack: Vec::new(),
            scope: Scope::new(),
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
            _ => {}
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
                ByteCode::Load => {
                    // let v = &self.stack_mem[self.stack_mem.len() - ins.arg];

                    // self.stack.push(v.clone());
                },
                ByteCode::Store => {
                    // let v = self.stack.pop().unwrap();

                    // let l = self.stack_mem.len();

                    // self.stack_mem[l - ins.arg] = v;
                },
                ByteCode::BinMul |
                ByteCode::BinAdd |
                ByteCode::BinMinus |
                ByteCode::BinDivide => {
                    // let tos = self.stack.pop().unwrap();
                    // let tos1 = self.stack.pop().unwrap();

                    // let v = match (tos, tos1) {
                    //     (Value::Int(a), Value::Int(b)) => {
                    //         match ins.code {
                    //             ByteCode::BinMul => Value::Int(a * b),
                    //             ByteCode::BinAdd => Value::Int(a + b),
                    //             ByteCode::BinMinus => Value::Int(a - b),
                    //             ByteCode::BinDivide => Value::Int(a / b),
                    //             _ => panic!("Invalid operation")
                    //         }
                    //     },
                    //     (Value::Float(a), Value::Float(b)) => {
                    //         match ins.code {
                    //             ByteCode::BinMul => Value::Float(a * b),
                    //             ByteCode::BinAdd => Value::Float(a + b),
                    //             ByteCode::BinMinus => Value::Float(a - b),
                    //             ByteCode::BinDivide => Value::Float(a / b),
                    //             _ => panic!("Invalid operation")
                    //         }
                    //     },
                    //     (Value::Float(a), Value::Int(b)) => {
                    //         match ins.code {
                    //             ByteCode::BinMul => Value::Float(a * b as f64),
                    //             ByteCode::BinAdd => Value::Float(a + b as f64),
                    //             ByteCode::BinMinus => Value::Float(a - b as f64),
                    //             ByteCode::BinDivide => Value::Float(a / b as f64),
                    //             _ => panic!("Invalid operation")
                    //         }
                    //     },
                    //     (Value::Int(a), Value::Float(b)) => {
                    //         match ins.code {
                    //             ByteCode::BinMul => Value::Float(a as f64 * b),
                    //             ByteCode::BinAdd => Value::Float(a as f64 + b),
                    //             ByteCode::BinMinus => Value::Float(a as f64 - b),
                    //             ByteCode::BinDivide => Value::Float(a as f64 / b),
                    //             _ => panic!("Invalid operation")
                    //         }
                    //     },
                    //     _ => panic!("Invalid operation")
                    // };

                    // self.stack.push(v);
                },
                ByteCode::Jump => {
                    // self.pc = ins.arg;

                    // log::debug!("jumping to {}", self.pc);
                },
                ByteCode::JumpIfFalse => {
                    // let tos = self.stack.pop().unwrap();
    
                    // match tos {
                    //     Value::Bool(true) => {},
                    //     Value::Float(1.0..) => {},
                    //     Value::Int(1..) => {},
                    //     _ => {
                    //         self.pc = ins.arg as usize;

                    //         log::debug!("jumping to {}", self.pc);
                    //     }
                    // };
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
                    let v = self.stack.pop().unwrap();

                    let side_effect = SideEffect::Return {
                        value: v
                    };

                    side_effects.push(side_effect);
                }
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

    pub fn store_name(&mut self, name: String) -> usize {
        self.stack_mem.push(Value::None);

        self.stack_mem.len() - 1
    }

    pub fn push(&mut self, v: Value) {
        self.stack.push(v);
    }

    pub fn add_ins(&mut self, ins: Ins) {
        self.files[self.current].instructions.push(ins);
    }

    // fn store(&mut self, index: usize, value: Value) {
    //     self.scopes[self.current].values[index] = value;
    // }
}

#[cfg(test)]
mod tests {
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
}