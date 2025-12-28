use std::env;
mod ast;
mod lexer;
mod parser;
mod ir;
mod vm;
mod value;
mod compiler;
pub mod error;

use std::fs;
use crate::lexer::tokenize;
use crate::parser::Parser;
use crate::compiler::compile;
use crate::vm::VM;

/// Entry point for the Pallad toolchain: reads a source file, tokenizes and parses it, compiles the AST, and executes the resulting program on the VM while printing any errors to standard error.
///
/// On success this runs the compiled program; on failure it prints a descriptive error message to stderr and exits early for that stage (file read, tokenization, parsing, or compilation). The default input path is "examples/example.pd" when no command-line argument is provided.
///
/// # Examples
///
/// ```no_run
/// // Run with the default example file:
/// // $ cargo run --release
///
/// // Run against a specific source file:
/// // $ cargo run --release -- path/to/program.pd
/// ```
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).map(|s| s.as_str()).unwrap_or("examples/example.pd");

    let code = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read the Pallad source file '{}': {}", filename, e);
            return;
        }
    };

    let tokens = match tokenize(&code) {
        Ok(toks) => toks,
        Err(err) => {
            eprintln!("Tokenizer error: {}", err);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let stmts = match parser.parse() {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Parse error: {}", err);
            return;
        }
    };

    let program = match compile(stmts) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Compile error: {}", err);
            return;
        }
    };

    let mut vm = VM::new();
    if let Err(err) = vm.run(program) {
        eprintln!("Runtime error: {}", err);
    }
}