use crate::ast::{BinOp, Expr, Stmt, UnOp};
use crate::error::PalladError;
use crate::ir::Instr;

pub fn compile(stmts: Vec<Stmt>) -> Result<Vec<Instr>, PalladError> {
    let mut program = vec![];

    for stmt in stmts {
        match stmt {
            Stmt::Let { name, expr, line } => {
                compile_expr(expr, &mut program);
                program.push(Instr::StoreVar { name, line });
            }
            Stmt::Set { name, expr, line } => {
                compile_expr(expr, &mut program);
                program.push(Instr::SetVar { name, line });
            }
            Stmt::Expr {
                expr: Expr::Call { name, args, .. },
                line,
            } => {
                let argc = args.len();
                for arg in args {
                    compile_expr(arg, &mut program);
                }
                program.push(Instr::CallBuiltin { name, argc, line });
            }
            Stmt::Expr { expr, line } => {
                compile_expr(expr, &mut program);
                program.push(Instr::Pop { line });
            }
        }
    }

    Ok(program)
}

fn compile_expr(expr: Expr, program: &mut Vec<Instr>) {
    match expr {
        Expr::None { line } => program.push(Instr::LoadNone { line }),
        Expr::Bool { value, line } => program.push(Instr::LoadBool { value, line }),
        Expr::Int { value, line } => program.push(Instr::LoadInt { value, line }),
        Expr::Float { value, line } => program.push(Instr::LoadFloat { value, line }),
        Expr::Str { value, line } => program.push(Instr::LoadStr { value, line }),
        Expr::Var { name, line } => program.push(Instr::LoadVar { name, line }),
        Expr::Binary {
            left,
            op,
            right,
            line,
        } => {
            compile_expr(*left, program);
            compile_expr(*right, program);
            match op {
                BinOp::Add => program.push(Instr::Add { line }),
                BinOp::Sub => program.push(Instr::Sub { line }),
                BinOp::Mul => program.push(Instr::Mul { line }),
                BinOp::Div => program.push(Instr::Div { line }),
                BinOp::IntDiv => program.push(Instr::IntDiv { line }),
                BinOp::Mod => program.push(Instr::Mod { line }),
                BinOp::Pow => program.push(Instr::Pow { line }),
                BinOp::And => program.push(Instr::And { line }),
                BinOp::Or => program.push(Instr::Or { line }),
            }
        }
        Expr::Unary { op, expr, line } => {
            compile_expr(*expr, program);
            match op {
                UnOp::Neg => program.push(Instr::Neg { line }),
                UnOp::Not => program.push(Instr::Not { line }),
            }
        }
        Expr::Call { name, args, line } => {
            let argc = args.len();
            for arg in args {
                compile_expr(arg, program);
            }
            program.push(Instr::CallBuiltin { name, argc, line });
        }
    }
}
