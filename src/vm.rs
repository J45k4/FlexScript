use std::collections::HashMap;

use crate::ASTNode;
use crate::ForCond;
use crate::Obj;
use crate::ObjProp;
use crate::Op;
use crate::Parser;
use crate::RunResult;
use crate::StackValue;
use crate::Value;
use crate::callstack::Call;
use crate::callstack::Callstack;
use crate::scope::ScopeManager;
use crate::vm_types::ByteCode;

pub struct Vm {
    scopes: Vec<usize>,
    constants: Vec<Value>,
    code_blocks: Vec<Vec<ByteCode>>,
    callstacks: Vec<Callstack>,
    idt_map: HashMap<String, u32>,
    id_idt_map: HashMap<u32, String>,
    next_idt: u32,
    scope: ScopeManager,
    pub log: usize
}

impl Vm {
    pub fn new() -> Self {
        Self {
            scopes: Vec::new(),
            constants: Vec::new(),
            code_blocks: Vec::new(),
            callstacks: Vec::new(),
            scope: ScopeManager::new(),
            idt_map: HashMap::new(),
            id_idt_map: HashMap::new(),
            next_idt: 0,
            log: 0
        }
    }

    pub fn compile_ast(&mut self, ast: &Vec<ASTNode>) -> u32 {
        let mut code_block = Vec::new();
        for node in ast {
            self.compile_node(&mut code_block, node);
        }   
        self.code_blocks.push(code_block);
        (self.code_blocks.len() - 1) as u32
    }

    pub fn compile_code(&mut self, code: &str) -> u32 {
        let ast = Parser::new(code).parse();
        self.compile_ast(&ast)
    }

    pub fn compile_node(&mut self, block: &mut Vec<ByteCode>, node: &ASTNode) {
        if self.log > 0 {
            println!("compile: {:?}", node);
        }

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

                block[jump_if_false_inx] = ByteCode::JumpIfFalse(block.len() as u32);

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
                block.push(ByteCode::MakeArray(a.items.len() as u32));
            },
            ASTNode::Fun(f) => {
                let mut fun_block = Vec::new();
                for param in &f.params {
                    let i = self.store_idt(param.name.clone());
                    fun_block.push(ByteCode::Store(i));
                }
                for node in &f.body {
                    self.compile_node(&mut fun_block, node);
                }
                self.code_blocks.push(fun_block);
                block.push(ByteCode::Fun((self.code_blocks.len() - 1) as u32));
            },
            ASTNode::Call(c) => {
                for arg in &c.args {
                    self.compile_node(block, arg);
                }

                match &*c.callee {
                    ASTNode::Ident(idt) => {
                        if idt == "await" {
                            block.push(ByteCode::Await);
                            return;
                        }
                    },
                    _ => {}
                }

                self.compile_node(block, &c.callee);
                block.push(ByteCode::Call(c.args.len() as u32));
            },
            ASTNode::For(f) => {
                let mut start_pc = 0;
                let mut false_jump_pc = 0;
                match &f.cond {
                    ForCond::FromIt { ident, it } => {
                        self.compile_node(block, it);
                        block.push(ByteCode::MakeIter);
                        block.push(ByteCode::Next);
                        start_pc = block.len() - 1;
                        let i = self.store_idt(ident.clone());
                        block.push(ByteCode::Store(i));
                        block.push(ByteCode::Load(i));
                        block.push(ByteCode::JumpIfFalse(0));
                        false_jump_pc = block.len() - 1;
                    },
                    _ => todo!()
                }

                for node in &f.body {
                    self.compile_node(block, node);
                }

                block.push(ByteCode::Jump(start_pc as u32));
                block[false_jump_pc] = ByteCode::JumpIfFalse(block.len() as u32);
            },
            ASTNode::ObjIns(obj) => {
                // block.push(ByteCode::LoadConst(self.store_const(Value::Str(obj.name.clone()))));
                for prop in &obj.props {
                    let c = self.store_const(Value::Str(prop.name.clone()));
                    block.push(ByteCode::LoadConst(c));
                    self.compile_node(block, &prop.value);
                }
                if let Some(name) = &obj.name {
                    block.push(ByteCode::LoadConst(self.store_const(Value::Str(name.clone()))));
                } else {
                    block.push(ByteCode::LoadConst(self.store_const(Value::None)));
                }
                block.push(ByteCode::Obj(obj.props.len() as u32));
            },
            ASTNode::ProbAccess(a) => {
                self.compile_node(block, &a.object);
                let i = self.store_idt(a.property.clone());
                block.push(ByteCode::AccessProp(i));
            },
            _ => todo!("{:?}", node)
        }
    }

    pub fn cont(&mut self, stack_id: usize, value: Value) -> RunResult {
        let stack = self.callstacks.get_mut(stack_id).unwrap();
        let val = match value {
            Value::Int(i) => StackValue::Int(i),
            Value::Float(f) => StackValue::Float(f),
            Value::Str(s) => StackValue::Str(s),
            Value::Bool(b) => StackValue::Bool(b),
            _ => todo!("{:?}", value)
        };
        stack.push_value(val);
        self.run_stack(stack_id)
    }

    pub fn run_stack(&mut self, stack_id: usize) -> RunResult {
        if self.log > 0 {
            println!("run stack: {}", stack_id);
        }
        
        loop {
            let stack = match self.callstacks.get_mut(stack_id) {
                Some(s) => s,
                None => {
                    if self.log > 0 {
                        println!("no stack");
                    }
                    return RunResult::None;
                }
            };
            let mut curr_blk = match stack.blk() {
                Some(b) => b,
                None => {
                    if self.log > 0 {
                        println!("no blk");
                    }
                    return RunResult::None;
                }
            };

            if self.log > 1 {
                println!("stack: {:?}", stack);
            }

            while stack.pc() < self.code_blocks[curr_blk as usize].len() as u32 {
                let pc = stack.pc();
                stack.increment_pc();

                let c = &self.code_blocks[curr_blk as usize][pc as usize];

                if self.log > 0 {
                    print!("blk: {} pc: {}, code: {:?}", curr_blk, pc, c);

                    match c {
                        ByteCode::Load(i) => {
                            let name = self.id_idt_map.get(&i).unwrap();
                            let v = self.scope.lookup(stack.scope_id(), i);
                            print!(" {} {:?}", name, v);
                        },
                        ByteCode::Store(i) => {
                            let name = self.id_idt_map.get(&i).unwrap();
                            let v = stack.peek_value();
                            print!(" {} {:?}", name, v)
                        },
                        ByteCode::JumpIfFalse(_) => print!(" {:?}", stack.peek_value()),
                        ByteCode::Next => print!(" {:?}", stack.peek_value()),
                        ByteCode::LoadConst(i) => print!(" {:?}", self.constants[*i as usize].clone()),
                        ByteCode::Ret(_) => print!(" {:?}", stack.peek_value()),
                        _ => {}
                    }

                    println!("");
                }

                match c {
                    ByteCode::Load(i) => {
                        match self.scope.lookup(stack.scope_id(), i) {
                            Some(v) => stack.push_value(StackValue::from(&*v)),
                            None => stack.push_value(StackValue::Undef(*i))
                        };
                    },
                    ByteCode::Store(i) => {
                        let v = Value::from(stack.pop_value().unwrap());
                        self.scope.store_named(stack.scope_id(), *i, v);
                    },
                    ByteCode::BinMul |
                    ByteCode::BinAdd |
                    ByteCode::BinMinus |
                    ByteCode::BinDivide => {
                        let tos = stack.pop_value().unwrap();
                        let tos1 = stack.pop_value().unwrap();

                        let v = match (tos1,tos) {
                            (StackValue::Int(a), StackValue::Int(b)) => {
                                match c {
                                    ByteCode::BinMul => StackValue::Int(a * b),
                                    ByteCode::BinAdd => StackValue::Int(a + b),
                                    ByteCode::BinMinus => StackValue::Int(a - b),
                                    ByteCode::BinDivide => StackValue::Int(a / b),
                                    _ => panic!("Invalid operation")
                                }
                            },
                            (StackValue::Float(a), StackValue::Float(b)) => {
                                match c {
                                    ByteCode::BinMul => StackValue::Float(a * b),
                                    ByteCode::BinAdd => StackValue::Float(a + b),
                                    ByteCode::BinMinus => StackValue::Float(a - b),
                                    ByteCode::BinDivide => StackValue::Float(a / b),
                                    _ => panic!("Invalid operation")
                                }
                            },
                            (StackValue::Float(a), StackValue::Int(b)) => {
                                match c {
                                    ByteCode::BinMul => StackValue::Float(a * b as f64),
                                    ByteCode::BinAdd => StackValue::Float(a + b as f64),
                                    ByteCode::BinMinus => StackValue::Float(a - b as f64),
                                    ByteCode::BinDivide => StackValue::Float(a / b as f64),
                                    _ => panic!("Invalid operation")
                                }
                            },
                            (StackValue::Int(a), StackValue::Float(b)) => {
                                match c {
                                    ByteCode::BinMul => StackValue::Float(a as f64 * b),
                                    ByteCode::BinAdd => StackValue::Float(a as f64 + b),
                                    ByteCode::BinMinus => StackValue::Float(a as f64 - b),
                                    ByteCode::BinDivide => StackValue::Float(a as f64 / b),
                                    _ => panic!("Invalid operation")
                                }
                            },
                            _ => panic!("Invalid operation")
                        };

                        stack.push_value(v);
                    },
                    ByteCode::Jump(indx) => stack.set_pc(*indx),
                    ByteCode::JumpIfFalse(inx) => {
                        let v = stack.pop_value().unwrap();

                        match v {
                            StackValue::Bool(b) => {
                                if !b {
                                    stack.set_pc(*inx);
                                }
                            },
                            StackValue::None => {
                                stack.set_pc(*inx);
                            },
                            StackValue::Int(i) => {
                                if i < 1 {
                                    stack.set_pc(*inx);
                                } 
                            },
                            _ => panic!("{:?}", v)
                        }
                    },
                    ByteCode::Call(arg_count) => {
                        if self.log > 1 {
                            println!("{:?}", stack);
                        }

                        let callee = stack.pop_value().unwrap();

                        let mut args = Vec::new();

                        for _ in 0..*arg_count {
                            let v = stack.pop_value().unwrap();
                            args.push(v);
                        }   

                        args.reverse();

                        if self.log > 1 {
                            println!("args: {:?}", args);
                        }

                        match callee {
                            StackValue::Fn(blk) => {
                                let scope_id = self.scope.create_child_scope(stack.scope_id());

                                stack.push(Call {
                                    blk,
                                    scope_id,
                                    values: args,
                                    pc: 0
                                });

                                curr_blk = blk;
                            },
                            StackValue::Undef(i) => {
                                let mut args = vec![];
                                for _ in 0..*arg_count {
                                    let v = stack.pop_value().unwrap();
                                    args.push(v);
                                }
                                args.reverse();
                                stack.push_value(StackValue::UndefCall {
                                    ident: i,
                                    args
                                });
                            },
                            StackValue::PropAccess { obj, prop } => {
                                let val = match self.scope.lookup(stack.scope_id(), &obj) {
                                    Some(v) => v,
                                    None => todo!()
                                };

                                match val {
                                    Value::List(l) => {
                                        for arg in args {
                                            match arg {
                                                StackValue::Int(i) => {
                                                    l.push(Value::Int(i));
                                                },
                                                _ => todo!("{:?}", arg)
                                            }
                                        }
                                    },
                                    _ => todo!("{:?}", val)
                                };
                            },
                            _ => todo!("invalid callee {:?}", callee)
                        }
                    },
                    ByteCode::Cmp => {
                        let tos = stack.pop_value().unwrap();
                        let tos1 = stack.pop_value().unwrap();
                        
                        let v = match (tos, tos1) {
                            (StackValue::Int(a), StackValue::Int(b)) => StackValue::Bool(a == b),
                            (StackValue::Float(a), StackValue::Float(b)) => StackValue::Bool(a == b),
                            (StackValue::Float(a), StackValue::Int(b)) => StackValue::Bool(a == b as f64),
                            (StackValue::Int(a), StackValue::Float(b)) => StackValue::Bool(a as f64 == b),
                            (StackValue::Bool(a), StackValue::Bool(b)) => StackValue::Bool(a == b),
                            (StackValue::Str(a), StackValue::Str(b)) => StackValue::Bool(a == b),
                            _ => panic!("Invalid operation")
                        };

                        stack.push_value(v);
                    },
                    ByteCode::BeginScope => {
                        let scope_id = self.scope.create_child_scope(stack.scope_id());
                        stack.set_scope_id(scope_id);
                    },
                    ByteCode::EndScope => {
                        let parent_scope = self.scope.get_parent_scope(stack.scope_id()).unwrap();
                        stack.set_scope_id(parent_scope);
                    },
                    ByteCode::LoadConst(a) => {
                        let v = self.constants[*a as usize].clone();
                        let v = match v {
                            Value::Int(i) => StackValue::Int(i),
                            Value::Float(f) => StackValue::Float(f),
                            Value::Str(s) => StackValue::Str(s),
                            Value::Bool(b) => StackValue::Bool(b),
                            _ => todo!("{:?}", v)
                        };
                        stack.push_value(v);
                    },
                    ByteCode::MakeStruct => todo!(),
                    ByteCode::MakeArray(len) => {
                        let mut items = vec![];
                        for _ in 0..*len {
                            let v = Value::from(stack.pop_value().unwrap());
                            items.push(v);
                        }
                        items.reverse();
                        let id = self.scope.store_unamed(Value::List(items));
                        stack.push_value(StackValue::Ref(id));
                    },
                    ByteCode::Assign => todo!(),
                    ByteCode::Ret(c) => {
                        return match stack.pop_value() {
                            Some(v) => RunResult::Value {
                                scope_id: stack.scope_id(),
                                value: Value::from(v)
                            },
                            None => RunResult::None
                        };
                    },
                    ByteCode::Fun(i) => stack.push_value(StackValue::Fn(*i)),
                    ByteCode::Next => {
                        let val = stack.peek_value().unwrap();
                        match val {
                            StackValue::Ref(r) => {
                                let val = match self.scope.lookup(stack.scope_id(), &(*r as u32)) {
                                    Some(v) => v,
                                    None => todo!()
                                };

                                let (id, inx) = match val {
                                    Value::ListIter { inx, id } => (*id, *inx),
                                    _ => todo!("{:?}", val)
                                };

                                let val = match self.scope.lookup(stack.scope_id(), &id) {
                                    Some(v) => v,
                                    None => todo!()
                                };

                                let val = match val {
                                    Value::List(arr) => {
                                        arr.get(inx as usize).unwrap()
                                    },
                                    _ => todo!("{:?}", val)
                                };

                                let val = StackValue::from(val);
                                stack.push_value(val);
                            },
                            _ => todo!("{:?}", val)
                        }
                    },
                    ByteCode::MakeIter => {
                        let val = stack.pop_value().unwrap();

                        match val {
                            StackValue::Ref(r) => {
                                let val = match self.scope.lookup(stack.scope_id(), &r) {
                                    Some(v) => v,
                                    None => todo!()
                                };

                                match val {
                                    Value::List(arr) => {
                                        let it_id = self.scope.store_unamed(Value::ListIter {
                                            inx: 0,
                                            id: r
                                        });
                                        stack.push_value(StackValue::Ref(it_id));
                                    },
                                    _ => todo!("{:?}", val)
                                };
                            },
                            _ => todo!("{:?}", val)
                        }
                    },
                    ByteCode::Await => {
                        let val = stack.pop_value().unwrap();
                        let val = match val {
                            StackValue::Int(i) => Value::Int(i),
                            StackValue::Float(f) => Value::Float(f),
                            StackValue::Str(s) => Value::Str(s),
                            StackValue::Bool(b) => Value::Bool(b),
                            StackValue::Undef(i) => Value::UndefIdent(i),
                            StackValue::UndefCall { ident, args } => {
                                Value::UndefCall {
                                    ident,
                                    args: vec![]
                                }
                            },
                            _ => todo!("{:?}", val)
                        };

                        return RunResult::Await {
                            stack_id,
                            value: val
                        };
                    },
                    ByteCode::Obj(arg_count) => {
                        let name = match stack.pop_value() {
                            Some(v) => match v {
                                StackValue::Str(s) => Some(s),
                                _ => todo!("{:?}", v)
                            }, 
                            None => None
                        };
                        
                        let mut obj = Obj {
                            name,
                            props: vec![]
                        };

                        for _ in 0..*arg_count {
                            let v = stack.pop_value().unwrap();
                            let k = stack.pop_value().unwrap();

                            let key = match k {
                                StackValue::Str(s) => s,
                                _ => todo!("{:?}", k)
                            };

                            let v = match v {
                                StackValue::Int(i) => Value::Int(i),
                                StackValue::Float(f) => Value::Float(f),
                                StackValue::Str(s) => Value::Str(s),
                                StackValue::Bool(b) => Value::Bool(b),
                                _ => todo!("{:?}", v)
                            };

                            obj.props.push(
                                ObjProp {
                                    name: key,
                                    value: v
                                }
                            );
                        }

                        let id = self.scope.store_unamed(Value::Obj(obj));
                        stack.push_value(StackValue::Ref(id));
                    },
                    ByteCode::AccessProp(a) => {
                        let val = stack.pop_value().unwrap();
                        match val {
                            StackValue::Ref(r) => {
                                stack.push_value(StackValue::PropAccess { obj: r, prop: *a });
                            },
                            _ => todo!("{:?}", val)
                        };
                    },
                    _ => todo!("{:?}", c)
                };
            }

            stack.pop();
            if self.log > 0 {
                println!("stack popped");
            }

            if self.log > 1 {
                println!("callstacks: {:?}", self.callstacks);
            } 
        }

        RunResult::None
    }

    pub fn run_code(&mut self, code: &str) -> RunResult {
        let ast = Parser::new(code).parse();
        let blk = self.compile_ast(&ast);

        if self.log > 0 {
            println!("compiled ast to blk: {}", blk);
        }

        self.run_blk(blk, Value::None)
    }

    pub fn run_blk(&mut self, blk: u32, args: Value) -> RunResult {
        if self.log > 0 {
            println!("run_blk blk: {} args: {:?}", blk, args);
        }

        let scope_id = self.scope.create_scope();

        let mut stack = Callstack::new();
        stack.log = self.log;
        stack.push(Call {
            blk: blk,
            scope_id,
            ..Default::default()
        });
        self.callstacks.push(stack);

        if self.log > 1 {
            println!("callstacks: {:?}", self.callstacks);
        }

        self.run_stack(self.callstacks.len() - 1)
    }

    pub fn store_const(&mut self, v: Value) -> u32 {
        self.constants.push(v);
        (self.constants.len() - 1) as u32
    }

    pub fn store_idt(&mut self, name: String) -> u32 {
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

    pub fn get_val(&mut self, scope_id: u32, id: u32) -> Option<&mut Value> {
        self.scope.lookup(scope_id, &id)
    }
}

#[cfg(test)]
mod tests {
    use crate::Obj;

    use super::*;

    #[test]
    fn test_return_number() {
        let mut vm = Vm::new();
        let res = vm.run_code("return 1");
        assert_eq!(res, RunResult::Value { value: Value::Int(1), scope_id: 0 });
    }

    #[test]
    fn simple_plus() {
        let mut vm = Vm::new();
        let res = vm.run_code("return 1 + 1");
        assert_eq!(res, RunResult::Value { value: Value::Int(2), scope_id: 0});
    }

    #[test]
    fn simple_sub() {
        let mut vm = Vm::new();
        let res = vm.run_code("return 1 - 1");
        assert_eq!(res, RunResult::Value { value: Value::Int(0), scope_id: 0});
    }

    #[test]
    fn add_sub() {
        let mut vm = Vm::new();
        let res = vm.run_code("return 1 + 1 - 1");
        assert_eq!(res, RunResult::Value { value: Value::Int(1), scope_id: 0});
    }

    #[test]
    fn simple_comparsion() {
        let mut vm = Vm::new();
        let res = vm.run_code("return 1 == 1");
        assert_eq!(res, RunResult::Value { value: Value::Bool(true), scope_id: 0});
    }

    #[test]
    fn simple_if_true() {
        let mut vm = Vm::new();
        let res = vm.run_code("if true { return 1 }");
        assert_eq!(res, RunResult::Value { value: Value::Int(1), scope_id: 0 });
    }

    #[test]
    fn simple_if_false() {
        let mut vm = Vm::new();
        let res = vm.run_code("if false { return 1 }");
        assert_eq!(res, RunResult::None);
    }

    #[test]
    fn assign_to_var() {
        let mut vm = Vm::new();
        let res = vm.run_code(r#"
        a = 1
        return a
        "#);
        assert_eq!(res, RunResult::Value { value: Value::Int(1), scope_id: 0 });
    }

    #[test]
    fn simple_array() {
        let mut vm = Vm::new();
        let res = vm.run_code("return [1,2,3]");

        match res {
            RunResult::Value { value, .. } => {
                let arr = match value {
                    Value::Ptr(p) => match vm.get_val(0, p) {
                        Some(Value::List(arr)) => arr,
                        _ => panic!("Invalid result")
                    },
                    _ => panic!("Invalid result")
                };
                assert_eq!(arr, &mut vec![
                    Value::Int(1),
                    Value::Int(2),
                    Value::Int(3)
                ]);
            },
            _ => panic!("Invalid result")
        }
    }

    #[test]
    fn function_calling() {
        let mut vm = Vm::new();
        vm.log = 2;
        let res = vm.run_code(r#"
        a = () => return 1
        return a()
        "#);
        println!("{:?}", vm.code_blocks);
        assert_eq!(res, RunResult::Value { value: Value::Int(1), scope_id: 1 });
    }

    #[test]
    fn function_call_with_args() {
        let mut vm = Vm::new();
        vm.log = 2;
        let res = vm.run_code(r#"
        a = (a) => return a
        return a(1)
        "#);
        println!("{:?}", vm.code_blocks);
        assert_eq!(res, RunResult::Value { value: Value::Int(1), scope_id: 1 });
    }

    #[test]
    fn simple_for() {
        let mut vm = Vm::new();
        let res = vm.run_code(r#"
        state = 0
        for a in [1,2,3] {
            state = state - a
        }
        return state
        "#);
        assert_eq!(res, RunResult::Value { value: Value::Int(-6), scope_id: 0 });
    }

    #[test]
    fn await_fun() {
        let mut vm = Vm::new();
        vm.log = 1;
        let res = vm.run_code("await(test())");

        assert_eq!(res, RunResult::Await {
            stack_id: 0,
            value: Value::UndefCall { 
                ident: 0, 
                args: vec![] 
            }
        });
    }

    #[test]
    fn await_fun_return_result() {
        let mut vm = Vm::new();
        let res = vm.run_code(r#"return await(test())"#);

        match res {
            RunResult::Await { stack_id, value } => {
                let res = vm.cont(stack_id, Value::Int(1));
                assert_eq!(res, RunResult::Value { value: Value::Int(1), scope_id: 0 });
            },
            _ => panic!("Invalid result")
        }
    }

    #[test]
    fn return_obj_instance() {
        let mut vm = Vm::new();
        let res = vm.run_code(r#"return H1 { text: "lol" }"#);

        match res {
            RunResult::Value { value, .. } => {
                let obj = match value {
                    Value::Ptr(p) => match vm.get_val(0, p) {
                        Some(Value::Obj(obj)) => obj,
                        _ => panic!("Invalid result")
                    },
                    _ => panic!("Invalid result")
                };
                assert_eq!(obj, &mut Obj {
                    name: Some("H1".to_string()),
                    props: vec![
                        ObjProp {
                            name: "text".to_string(),
                            value: Value::Str("lol".to_string())
                        }
                    ]
                });
            },
            _ => panic!("Invalid result")
        }
    }

    #[test]
    fn push_to_list() {
        let mut vm = Vm::new();
        vm.log = 2;
        let res = vm.run_code(r#"
        a = [1,2,3]
        a.push(4)
        return a
        "#);

        match res {
            RunResult::Value { value, .. } => {
                let arr = match value {
                    Value::Ptr(p) => match vm.get_val(0, p) {
                        Some(Value::List(arr)) => arr,
                        _ => panic!("Invalid result")
                    },
                    _ => panic!("Invalid result")
                };
                assert_eq!(arr, &mut vec![
                    Value::Int(1),
                    Value::Int(2),
                    Value::Int(3),
                    Value::Int(4)
                ]);
            },
            _ => panic!("Invalid result")
        }
    }
}