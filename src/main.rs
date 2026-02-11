use std::env;
mod ast;
mod compiler;
mod error;
mod ir;
mod lexer;
mod parser;
mod value;
mod vm;

use crate::compiler::compile;
use crate::lexer::tokenize;
use crate::parser::Parser;
use crate::vm::VM;
use std::fs;
use std::io::Error;

const FALLBACK_CODE: &str = include_str!("../examples/example.pd");

/// Reads Pallad source from `source_path`, with fallback behavior for empty or non-`.pd` paths.
///
/// If `source_path` ends with `.pd`, the file is read and its contents are returned.
/// If `source_path` is an empty string, the built-in `FALLBACK_CODE` is returned.
/// If `source_path` is any other non-empty value, a warning is printed to stderr and `FALLBACK_CODE` is returned.
///
/// # Errors
///
/// Propagates I/O errors that occur when reading a `.pd` file.
///
/// # Examples
///
/// ```
/// use std::fs;
/// let path = "tmp_example.pd";
/// fs::write(path, "print 1;").unwrap();
/// let src = read_source_file(path).unwrap();
/// assert_eq!(src, "print 1;");
/// fs::remove_file(path).unwrap();
/// ```
fn read_source_file(source_path: &str) -> Result<String, Error> {
    Ok(match source_path {
        file if file.ends_with(".pd") => fs::read_to_string(file)?,
        "" => FALLBACK_CODE.to_string(),
        other => {
            eprintln!(
                "Warning: '{}' is not a .pd file, using fallback example...",
                other
            );
            FALLBACK_CODE.to_string()
        }
    })
}

/// Entry point for the Pallad toolchain: reads a source file, tokenizes and parses it, compiles the AST, and executes the resulting program on the VM while printing any errors to standard error.
///
/// On success this runs the compiled program; on failure it prints a descriptive error message to stderr and exits early for that stage (file read, tokenization, parsing, or compilation). The default input path is "examples/example.pd" when no command-line argument is provided or the given file isn't a .pd file.
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
    let source_path_arg = args.get(1).map(|s| s.as_str()).unwrap_or("");

    let code = match read_source_file(source_path_arg) {
        Ok(source_code) => source_code,
        Err(e) => {
            eprintln!(
                "Failed to read the Pallad source file '{}': {}",
                source_path_arg, e
            );
            return;
        }
    };

    let tokens = match tokenize(&code) {
        Ok(tokens) => tokens,
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