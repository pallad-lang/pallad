use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum PalladError {
    UnexpectedToken {
        got: String,
        expected: String,
        line: usize,
    },
    EndOfInput {
        expected: String,
        line: usize,
    },
    UnknownCharacter {
        got: String,
        line: usize,
    },
    UnknownBuiltin {
        name: String,
        line: usize,
    },
    UndefinedVariable {
        name: String,
        line: usize,
    },
    StackUnderflow {
        operation: &'static str,
        line: usize,
    },
    TypeMismatch {
        left: Value,
        right: Value,
        operation: &'static str,
        line: usize,
    },
    UnaryTypeMismatch {
        value: Value,
        operation: &'static str,
        line: usize,
    },
    InvalidNumber {
        value: String,
        line: usize,
    },
    DivisionByZero {
        operation: &'static str,
        line: usize,
    },
    IntegerOverflow {
        operation: String,
        line: usize,
    },
    RepeatOverflow {
        line: usize,
    },
    NegativeRepeat {
        line: usize,
    },
    InvalidEscape {
        char: char,
        line: usize,
    },
    UnterminatedString {
        line: usize,
    },
    ZeroPowerZero {
        line: usize,
    },
    DuplicateVariable {
        name: String,
        line: usize,
    },
}

impl std::fmt::Display for PalladError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PalladError::UnexpectedToken {
                got,
                expected,
                line,
            } => write!(f, "Line {}: Expected {}, got {}", line, expected, got),
            PalladError::EndOfInput { expected, line } => {
                write!(f, "Line {}: Expected {}, got end of input", line, expected)
            }
            PalladError::UnknownCharacter { got, line } => {
                write!(f, "Line {}: Unknown character: {}", line, got)
            }
            PalladError::InvalidNumber { value, line } => {
                write!(f, "Line {}: Invalid number: {}", line, value)
            }
            PalladError::UnknownBuiltin { name, line } => {
                write!(f, "Line {}: Unknown builtin: {}", line, name)
            }
            PalladError::UndefinedVariable { name, line } => {
                write!(f, "Line {}: Undefined variable: {}", line, name)
            }
            PalladError::StackUnderflow { operation, line } => {
                write!(f, "Line {}: Stack underflow: {}", line, operation)
            }
            PalladError::TypeMismatch {
                left,
                right,
                operation,
                line,
            } => write!(
                f,
                "Line {}: Cannot {} '{}' and '{}'",
                line, operation, left, right
            ),
            PalladError::UnaryTypeMismatch {
                value,
                operation,
                line,
            } => write!(f, "Line {}: Cannot {} '{}'", line, operation, value),
            PalladError::DivisionByZero { operation, line } => write!(
                f,
                "Line {}: Division by zero at {} operation is not valid",
                line, operation
            ),
            PalladError::IntegerOverflow { operation, line } => {
                write!(f, "Line {}: Integer overflow at: {}", line, operation)
            }
            PalladError::RepeatOverflow { line } => {
                write!(f, "Line {}: String repeat overflow", line)
            }
            PalladError::NegativeRepeat { line } => {
                write!(f, "Line {}: String repeat count can't be negative", line)
            }
            PalladError::InvalidEscape { line, char } => {
                write!(f, "Line {}: Invalid escaped character: {}", line, char)
            }
            PalladError::UnterminatedString { line } => {
                write!(f, "Line {}: Unterminated string", line)
            }
            PalladError::ZeroPowerZero { line } => write!(f, "Line {}: 0 ** 0 not allowed", line),
            PalladError::DuplicateVariable { name, line } => {
                write!(f, "Line {}: Variable '{}' already defined", line, name)
            }
        }
    }
}

impl std::error::Error for PalladError {}
