use crate::ast::{BinOp, Expr, Stmt, UnOp};
use crate::error::PalladError;
use crate::ir::Instr;

/// Compiles a sequence of AST statements into a flat list of IR instructions.
///
/// This function translates each top-level `Stmt` into one or more `Instr` values,
/// preserving per-node source `line` information in emitted instructions.
/// It handles variable declarations (`Let` -> `StoreVar`), assignments (`Set` -> `SetVar`),
/// expression statements (compiled then `Pop`), and direct calls to builtins
/// (arguments compiled followed by `CallBuiltin`).
///
/// # Returns
///
/// `Ok(Vec<Instr>)` containing the compiled instruction stream on success, or
/// `Err(PalladError)` if compilation fails.
///
/// # Examples
///
/// ```
/// use crate::{compiler::compile, ast::{Stmt, Expr}, ir::Instr};
///
/// let stmts = vec![Stmt::Let {
///     name: "x".into(),
///     expr: Expr::Int { value: 42, line: 1 },
///     line: 1,
/// }];
///
/// let program = compile(stmts).expect("compile");
/// assert!(!program.is_empty());
/// match &program[0] {
///     Instr::LoadInt { value, line } => {
///         assert_eq!(*value, 42);
///         assert_eq!(*line, 1);
///     }
///     _ => panic!("expected LoadInt first"),
/// }
/// ```
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

/// Compiles an AST expression into IR instructions and appends them to `program`.
///
/// # Examples
///
/// ```
/// use crate::ast::Expr;
/// use crate::ir::Instr;
///
/// let expr = Expr::Int { value: 42, line: 1 };
/// let mut program: Vec<Instr> = Vec::new();
/// crate::compiler::compile_expr(expr, &mut program);
/// assert_eq!(program.len(), 1);
/// // program[0] is expected to be Instr::LoadInt { value: 42, line: 1 }
/// ```
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