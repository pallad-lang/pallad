#[derive(Debug, Clone, PartialEq)]
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
    /// Formats `PalladError` values into human-readable error messages.
    ///
    /// Each error variant is rendered with a concise description; variants that include a line number
    /// include it in the message.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crate::error::PalladError;
    ///
    /// let e = PalladError::UnexpectedToken { got: "}".into(), expected: "identifier".into(), line: 3 };
    /// assert_eq!(format!("{}", e), "Line 3: Expected identifier, got }");
    ///
    /// let e2 = PalladError::DivisionByZero { operation: "divide".into() };
    /// assert_eq!(format!("{}", e2), "Division by zero at divide operation is not valid");
    /// ```
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