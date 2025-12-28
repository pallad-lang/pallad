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
