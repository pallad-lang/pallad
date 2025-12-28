use crate::ast::{Stmt, Expr, BinOp};
use crate::ir::Instr;

pub fn compile(stmts: Vec<Stmt>) -> Vec<Instr> {
    let mut program = vec![];

    for stmt in stmts {
        match stmt {
            Stmt::Let { name, expr } => {
                compile_expr(expr, &mut program);
                program.push(Instr::StoreVar(name));
            }
            Stmt::Expr(Expr::Call { name, args }) => {
                for arg in &args {
                    compile_expr(arg.clone(), &mut program);
                }
                program.push(Instr::CallBuiltin { name, argc: args.len() });
            }
            Stmt::Expr(expr) => {
                compile_expr(expr, &mut program);
                program.push(Instr::Pop);
            }
        }
    }

    program
}

fn compile_expr(expr: Expr, program: &mut Vec<Instr>) {
    match expr {
        Expr::Int(n) => program.push(Instr::LoadInt(n)),
        Expr::Float(f) => program.push(Instr::LoadFloat(f)),
        Expr::Var(name) => program.push(Instr::LoadVar(name)),
        Expr::Binary { left, op, right } => {
            compile_expr(*left, program);
            compile_expr(*right, program);
            match op {
                BinOp::Add => program.push(Instr::Add),
                BinOp::Sub => program.push(Instr::Sub),
                BinOp::Mul => program.push(Instr::Mul),
                BinOp::Div => program.push(Instr::Div),
                BinOp::IntDiv => program.push(Instr::IntDiv),
                BinOp::Mod => program.push(Instr::Mod),
            }
        }
        Expr::Call { name, args } => {
            let argc = args.len();
            for arg in args {
                compile_expr(arg, program);
            }
            program.push(Instr::CallBuiltin { name, argc });
        }
    }
}
