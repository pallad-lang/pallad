use std::env;
mod ast;
mod lexer;
mod parser;
mod ir;
mod vm;
mod value;
mod compiler;
pub mod error;

use std::io::Error;
use std::fs;
use crate::lexer::tokenize;
use crate::parser::Parser;
use crate::compiler::compile;
use crate::vm::VM;

const FALLBACK_CODE: &str = include_str!("../examples/example.pd");

fn read_source_file(source_path: &str) -> Result<String, Error> {
    Ok(
        match source_path {
            file if file.ends_with(".pd") => fs::read_to_string(file)?,
            "" => FALLBACK_CODE.to_string(),
            other => {
                eprintln!("Warning: '{}' is not a .pd file, using fallback example...", other);
                FALLBACK_CODE.to_string()
            },
        }
    )
}

/// Entry point for the Pallad toolchain: reads a source file, tokenizes and parses it, compiles the AST, and executes the resulting program on the VM while printing any errors to standard error.
///
/// On success this runs the compiled program; on failure it prints a descriptive error message to stderr and exits early for that stage (file read, tokenization, parsing, or compilation). The default input path is "examples/example.pd" when no command-line argument is provided or given file isn't as .pd file.
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
    let input_path = args.get(1).map(|s| s.as_str()).unwrap_or("");

    let code = match read_source_file(input_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read the Pallad source file '{}': {}", input_path, e);
            return
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
        Ok(parsed_statements) => parsed_statements,
        Err(err) => {
            eprintln!("Parse error: {}", err);
            return;
        }
    };

    let bytecode = match compile(stmts) {
        Ok(compiled_program) => compiled_program,
        Err(err) => {
            eprintln!("Compile error: {}", err);
            return;
        }
    };

    let mut vm = VM::new();
    if let Err(err) = vm.run(bytecode) {
        eprintln!("Runtime error: {}", err);
    }
}