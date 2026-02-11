use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum PalladError {
    /// Tokenizer error: Got an unknown character.
    UnknownCharacter {
        /// What got (dynamic)
        got: String,
        /// Line number
        line: usize,
    },
    /// Tokenizer error: Got invalid number.
    InvalidNumber {
        /// Given value (dynamic)
        value: String,
        /// Line number
        line: usize,
    },
    /// Tokenizer error: Invalid escaped char in string.
    InvalidEscape {
        /// Escaped char
        char: char,
        /// Line number
        line: usize,
    },
    /// Tokenizer error: Unterminated string.
    UnterminatedString {
        /// Line number
        line: usize,
    },
    /// Parse error: Got an unexpected token.
    UnexpectedToken {
        /// What got (dynamic)
        got: String,
        /// What expected (static)
        expected: &'static str,
        /// Line number
        line: usize,
    },
    /// Parse error: Unexpected end of input.
    EndOfInput {
        /// What expected (static)
        expected: &'static str,
        /// Line number
        line: usize,
    },
    /// Runtime error: An unknown builtin called.
    UnknownBuiltin {
        /// Name of builtin (dynamic)
        name: String,
        /// Line number
        line: usize,
    },
    /// Runtime error: An undefined variable used.
    UndefinedVariable {
        /// Name of variable (dynamic)
        name: String,
        /// Line number
        line: usize,
    },
    /// Runtime error: Unexpected end of stack.
    StackUnderflow {
        /// Current operation (static)
        operation: &'static str,
        /// Line number
        line: usize,
    },
    /// Runtime error: Invalid value types for binary operation.
    TypeMismatch {
        /// Left value in operation
        left: Value,
        /// Right value in operation
        right: Value,
        /// Operation name (static)
        operation: &'static str,
        /// Line number
        line: usize,
    },
    /// Runtime error: Invalid value type for unary operation.
    UnaryTypeMismatch {
        /// Value in operation
        value: Value,
        /// Operation name (static)
        operation: &'static str,
        /// Line number
        line: usize,
    },
    /// Runtime error: Division by zero is not allowed.
    DivisionByZero {
        /// Complete operation (dynamic)
        operation: String,
        /// Line number
        line: usize,
    },
    /// Runtime error: Overflow on integer operation.
    IntegerOverflow {
        /// Complete operation (dynamic)
        operation: String,
        /// Line number
        line: usize,
    },
    /// Runtime error: Too big repeat for string. (string * int)
    RepeatOverflow {
        /// Line number
        line: usize,
    },
    /// Runtime error: Negative value for string repeat. (string * int)
    NegativeRepeat {
        /// Line number
        line: usize,
    },
    /// Runtime error: Duplicate variable declared.
    DuplicateVariable {
        /// Variable name
        name: String,
        /// Line number
        line: usize,
    },
    /// Runtime error: Negative exponent on int.
    NegativeExponentOnInteger {
        /// Complete operation (dynamic)
        operation: String,
        /// Line number
        line: usize,
    }
}

impl std::error::Error for PalladError {}
impl std::fmt::Display for PalladError {
    /// Formats a `PalladError` into a concise, human-readable message that includes the source line number.
    ///
    /// The message produced varies by error variant and always prefixes the description with `Line {line}:`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use pallad::error::PalladError;
    ///
    /// let err = PalladError::UndefinedVariable { name: "x".into(), line: 3 };
    /// assert_eq!(format!("{}", err), "Line 3: Undefined variable: x");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PalladError::UnknownCharacter { got, line } => {
                write!(f, "Line {}: Unknown character: {}", line, got)
            }
            PalladError::InvalidNumber { value, line } => {
                write!(f, "Line {}: Invalid number: {}", line, value)
            }
            PalladError::InvalidEscape { line, char } => {
                write!(f, "Line {}: Invalid escaped character: {}", line, char)
            }
            PalladError::UnterminatedString { line } => {
                write!(f, "Line {}: Unterminated string", line)
            }
            PalladError::UnexpectedToken { got, expected, line } => {
                write!(f, "Line {}: Expected {}, got {}", line, expected, got)
            }
            PalladError::EndOfInput { expected, line } => {
                write!(f, "Line {}: Expected {}, got end of input", line, expected)
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
            PalladError::TypeMismatch { left, right, operation, line } => {
                write!(f, "Line {}: Cannot {} '{}' and '{}'", line, operation, left, right)
            }
            PalladError::UnaryTypeMismatch { value, operation, line } => {
                write!(f, "Line {}: Cannot {} '{}'", line, operation, value)
            }
            PalladError::DivisionByZero { operation, line } => {
                write!(f, "Line {}: Division by zero: {}", line, operation)
            }
            PalladError::IntegerOverflow { operation, line } => {
                write!(f, "Line {}: Integer overflow at: {}", line, operation)
            }
            PalladError::RepeatOverflow { line } => {
                write!(f, "Line {}: String repeat overflow", line)
            }
            PalladError::NegativeRepeat { line } => {
                write!(f, "Line {}: String repeat count can't be negative", line)
            }
            PalladError::DuplicateVariable { name, line } => {
                write!(f, "Line {}: Variable '{}' already defined", line, name)
            }
            PalladError::NegativeExponentOnInteger { operation, line } => {
                write!(f, "Line {}: Negative exponent on integer: '{}', convert at least one value to float", line, operation)
            }
        }
    }
}