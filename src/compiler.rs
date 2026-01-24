use crate::ast::{BinOp, Expr, Stmt, UnOp};
use crate::error::PalladError;
use crate::ir::Instr;

/// Compile a sequence of AST statements into a vector of IR instructions.
///
/// The function traverses the provided statements in order and emits the corresponding
/// low-level instructions for each statement (e.g., evaluating expressions, storing
/// variables, calling builtins, and popping expression results).
///
/// # Examples
///
/// ```
/// use crate::{compile, ast::{Stmt, Expr}};
///
/// let stmts = vec![Stmt::Expr(Expr::Int(42))];
/// let program = compile(stmts).unwrap();
/// assert!(!program.is_empty());
/// ```
///
/// # Returns
/// 
/// `Ok(Vec<Instr>)` containing the compiled IR program on success, or `Err(PalladError)` if compilation fails.
pub fn compile(stmts: Vec<Stmt>) -> Result<Vec<Instr>, PalladError> {
    let mut program = vec![];

    for stmt in stmts {
        match stmt {
            Stmt::Let { name, expr } => {
                compile_expr(expr, &mut program);
                program.push(Instr::StoreVar(name));
            }
            Stmt::Expr(Expr::Call { name, args }) => {
                let argc = args.len();
                for arg in args {
                    compile_expr(arg, &mut program);
                }
                program.push(Instr::CallBuiltin { name, argc });
            }
            Stmt::Expr(expr) => {
                compile_expr(expr, &mut program);
                program.push(Instr::Pop);
            }
        }
    }

    Ok(program)
}

/// Compiles an AST expression into IR by appending the corresponding instructions to the given program buffer.
///
/// This includes literals (none, bool, int, float, string), variable loads, unary operations (operand then operator), binary operations (left then right then operator), and builtin function calls (arguments compiled in order).
///
/// # Examples
///
/// ```
/// let mut program = Vec::new();
/// compile_expr(Expr::Int(42), &mut program);
/// assert_eq!(program, vec![Instr::LoadInt(42)]);
/// ```
fn compile_expr(expr: Expr, program: &mut Vec<Instr>) {
    match expr {
        Expr::None => program.push(Instr::LoadNone),
        Expr::Bool(b) => program.push(Instr::LoadBool(b)),
        Expr::Int(n) => program.push(Instr::LoadInt(n)),
        Expr::Float(f) => program.push(Instr::LoadFloat(f)),
        Expr::Str(s) => program.push(Instr::LoadStr(s)),
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
                BinOp::Pow => program.push(Instr::Pow),
                BinOp::And => program.push(Instr::And),
                BinOp::Or => program.push(Instr::Or),
            }
        }
        Expr::Unary { op, expr } => {
            compile_expr(*expr, program);
            match op {
                UnOp::Neg => program.push(Instr::Neg),
                UnOp::Not => program.push(Instr::Not),
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