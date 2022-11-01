use std::collections::HashMap;

use crate::vm_types::ByteCode;
use crate::vm_types::File;
use crate::vm_types::Ins;
use crate::vm_types::SideEffect;
use crate::vm_types::Value;


pub struct Vm {
    pc: usize,
    current: usize,
    heap: Vec<Value>,
    stack: Vec<Value>,
    stack_mem: Vec<Value>,
    files: Vec<File>,
    scopes: Vec<usize>,
    constants: Vec<Value>,
}

impl Vm {
    pub fn run(&mut self) -> Vec<SideEffect> {
        let instructions = &self.files[self.current].instructions;

        let len = instructions.len();

        let mut side_effects = Vec::new();

        while self.pc < len {
            let pc = self.pc;
            self.pc += 1;

            let ins = &instructions[pc];

            match ins.code {
                ByteCode::Load => {
                    let v = &self.stack_mem[self.stack_mem.len() - ins.arg];

                    self.stack.push(v.clone());
                },
                ByteCode::Store => {
                    let v = self.stack.pop().unwrap();

                    let l = self.stack_mem.len();

                    self.stack_mem[l - ins.arg] = v;
                },
                ByteCode::BinMul |
                ByteCode::BinAdd |
                ByteCode::BinMinus |
                ByteCode::BinDivide => {
                    let tos = self.stack.pop().unwrap();
                    let tos1 = self.stack.pop().unwrap();

                    let v = match (tos, tos1) {
                        (Value::Int(a), Value::Int(b)) => {
                            match ins.code {
                                ByteCode::BinMul => Value::Int(a * b),
                                ByteCode::BinAdd => Value::Int(a + b),
                                ByteCode::BinMinus => Value::Int(a - b),
                                ByteCode::BinDivide => Value::Int(a / b),
                                _ => panic!("Invalid operation")
                            }
                        },
                        (Value::Float(a), Value::Float(b)) => {
                            match ins.code {
                                ByteCode::BinMul => Value::Float(a * b),
                                ByteCode::BinAdd => Value::Float(a + b),
                                ByteCode::BinMinus => Value::Float(a - b),
                                ByteCode::BinDivide => Value::Float(a / b),
                                _ => panic!("Invalid operation")
                            }
                        },
                        (Value::Float(a), Value::Int(b)) => {
                            match ins.code {
                                ByteCode::BinMul => Value::Float(a * b as f64),
                                ByteCode::BinAdd => Value::Float(a + b as f64),
                                ByteCode::BinMinus => Value::Float(a - b as f64),
                                ByteCode::BinDivide => Value::Float(a / b as f64),
                                _ => panic!("Invalid operation")
                            }
                        },
                        (Value::Int(a), Value::Float(b)) => {
                            match ins.code {
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
                    self.pc = ins.arg;

                    log::debug!("jumping to {}", self.pc);
                },
                ByteCode::JumpIfFalse => {
                    let tos = self.stack.pop().unwrap();
    
                    match tos {
                        Value::Bool(true) => {},
                        Value::Float(1.0..) => {},
                        Value::Int(1..) => {},
                        _ => {
                            self.pc = ins.arg as usize;

                            log::debug!("jumping to {}", self.pc);
                        }
                    };
                },
                ByteCode::Call => {
                    let mut args = vec![];
    
                    for _ in 0..ins.arg {
                        let v = self.stack.pop().unwrap();
                        args.push(v);
                    }

                    let v = self.stack.pop().unwrap();

                    match v {
                        Value::ExtRef(r) => {
                            let call = SideEffect::Call {
                                r,
                                args
                            };

                            side_effects.push(call);
                        },
                        _ => panic!("Invalid call")
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
                        (Value::String(a), Value::String(b)) => Value::Bool(a == b),
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
                ByteCode::LoadConst => todo!(),
                ByteCode::StoreName => todo!(),
                ByteCode::BinOP => todo!(),
                ByteCode::MakeFunction => todo!(),
                ByteCode::MakeStruct => todo!(),
                ByteCode::MakeArray => todo!(),
                ByteCode::Obj => todo!(),
                ByteCode::Assign => todo!(),
                
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