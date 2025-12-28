#[derive(Debug)]
pub enum PalladError {
    UnexpectedToken { got: String, expected: String, line: usize },
    EndOfInput { expected: String, line: usize },
    UnknownCharacter { got: String, line: usize },
    UnknownBuiltin { name: String },
    UndefinedVariable { name: String },
    StackUnderflow { operation: String },
    TypeMismatch { operation: String },
    InvalidNumber { value: String, line: usize },
    DivisionByZero { operation: String },
}

impl std::fmt::Display for PalladError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PalladError::UnexpectedToken { got, expected, line } =>
                write!(f, "Line {}: Expected {}, got {}", line, expected, got),
            PalladError::EndOfInput { expected, line } =>
                write!(f, "Line {}: Expected {}, got end of input", line, expected),
            PalladError::UnknownCharacter { got, line } =>
                write!(f, "Line {}: Unknown character: {}", line, got),
            PalladError::InvalidNumber { value, line } => 
                write!(f, "Line {}: Invalid number: {}", line, value),
            PalladError::UnknownBuiltin { name } =>
                write!(f, "Unknown builtin: {}", name),
            PalladError::UndefinedVariable { name } => 
                write!(f, "Undefined variable: {}", name),
            PalladError::StackUnderflow { operation } =>
                write!(f, "Stack underflow: {}", operation),
            PalladError::TypeMismatch { operation } =>
                write!(f, "Type mismatch: {}", operation),
            PalladError::DivisionByZero { operation } =>
                write!(f, "Division by zero at {} operation is not valid", operation)
        }
    }
}

impl std::error::Error for PalladError {}
