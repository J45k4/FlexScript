use crate::Assign;
use crate::Ast;
use crate::BinOP;
use crate::Body;
use crate::Call;
use crate::Expr;
use crate::ForExpr;
use crate::Func;
use crate::If;
use crate::IfBranch;
use crate::MatchExpr;
use crate::ObjExpr;
use crate::Operator;
use crate::PropAccess;
use crate::RangeExpr;
use crate::Struct;

use crate::vm_types::ADD_OP;
use crate::vm_types::ByteCode;
use crate::vm_types::DIV_OP;
use crate::vm_types::EQUAL_TO_OP;
use crate::vm_types::GREATER_THAN_EQUAL_TO_OP;
use crate::vm_types::GREATER_THAN_OP;
use crate::vm_types::Ins;
use crate::vm_types::LOGICAL_AND;
use crate::vm_types::LOGICAL_OR;
use crate::vm_types::MOD_OP;
use crate::vm_types::MUL_OP;
use crate::vm_types::NOT_EQUAL_TO_OP;
use crate::vm_types::SMALLER_THAN_EQUAL_TO_OP;
use crate::vm_types::SMALLER_THAN_OP;
use crate::vm_types::SUB_OP;
use crate::vm_types::Value;
use crate::vm::Vm;

fn compile_op(vm: &mut Vm, op: &Operator) {
    let op = match op {
        Operator::Add => ADD_OP,
        Operator::Sub => SUB_OP,
        Operator::Mul => MUL_OP,
        Operator::Div => DIV_OP,
        Operator::LogicalAnd => LOGICAL_AND,
        Operator::LogicalOr => LOGICAL_OR,
        Operator::Modulus => MOD_OP,
        Operator::Pow => todo!(),
        Operator::And => LOGICAL_AND,
        Operator::Or => LOGICAL_OR,
        Operator::Eq => EQUAL_TO_OP,
        Operator::Lt => SMALLER_THAN_OP,
        Operator::Lte => SMALLER_THAN_EQUAL_TO_OP,
        Operator::Gt => GREATER_THAN_OP,
        Operator::Gte => GREATER_THAN_EQUAL_TO_OP,
        Operator::Neq => NOT_EQUAL_TO_OP,
    };

    let ins = Ins {
        code: ByteCode::BinOP,
        arg: op
    };

    vm.add_ins(ins);
}

fn compine_binop(vm: &mut Vm, binop: &BinOP) {
    compile_expr(vm, &binop.left);
    compile_op(vm, &binop.op);
    compile_expr(vm, &binop.right);
}

fn compile_branch(vm: &mut Vm, branch: &IfBranch) {
    compile_expr(vm, &branch.condition);
    compile_body(vm, &branch.body);
}

fn compile_if(vm: &mut Vm, if_stmt: &If) {
    for branch in &if_stmt.branches {
        compile_branch(vm, branch);
    }
}

fn compile_obj_expr(vm: &mut Vm, obj_expr: &ObjExpr) {
    vm.add_ins(Ins {
        code: ByteCode::Obj,
        arg: obj_expr.fields.len()
    });

    for field in &obj_expr.fields {
        let i = vm.store_const(Value::String(field.target.clone()));

        vm.add_ins(Ins {
            code: ByteCode::LoadConst,
            arg: i
        });

        compile_expr(vm, &field.value);
    }

    vm.add_ins(Ins {
        code: ByteCode::Store,
        arg: 0
    });
}

fn compile_func(vm: &mut Vm, func: &Func) {
    // compile_body(&mut func_vm, &func.body);

    // let i = vm.store_const(Value::Func(func_vm));

    // vm.add_ins(Ins {
    //     code: ByteCode::LoadConst,
    //     arg: i
    // });
}

fn compile_for(vm: &mut Vm, for_expr: &ForExpr) {
    todo!()
}

fn compile_call(vm: &mut Vm, call: &Call) {
    for arg in &call.args {
        compile_expr(vm, arg);
    }

    compile_expr(vm, &call.callee);

    vm.add_ins(Ins {
        code: ByteCode::Call,
        arg: call.args.len()
    });
}

fn compile_range(vm: &mut Vm, range: &RangeExpr) {
    todo!()
}

fn compile_match_expr(vm: &mut Vm, match_expr: &MatchExpr) {
    todo!()
}

fn compile_prop_access(vm: &mut Vm, prop_access: &PropAccess) {
    todo!()
}

fn compile_assign(vm: &mut Vm, assign: Assign) {
    compile_expr(vm, &assign.value);

    vm.add_ins(Ins {
        code: ByteCode::StoreName,
        arg: 0
    });
}

fn compile_struct(vm: &mut Vm, s: &Struct) {
    
}

fn compile_array(vm: &mut Vm, array: &Vec<Expr>) {
    for expr in array {
        compile_expr(vm, expr);
    }

    vm.add_ins(Ins {
        code: ByteCode::MakeArray,
        arg: array.len()
    });
}

fn compile_xml(vm: &mut Vm, xml: &Vec<Expr>) {
    todo!()
}

fn compile_cost(vm: &mut Vm, cost: &Vec<Expr>) {
    todo!()
}

fn compile_expr(vm: &mut Vm, expr: &Expr) {
    match expr {
        Expr::BinOP(binop) => {
            compine_binop(vm, binop);
        },
        Expr::ObjExpr(ob) => {
            compile_obj_expr(vm, ob);
        },
        _ => {

        }
    }
}



fn compile_ident(vm: &mut Vm, ident: String) {

}

fn compile_body(vm: &mut Vm, body: &Body) {

}

pub fn compile(vm: &mut Vm, ast: Ast) {

}