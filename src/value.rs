use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
}

impl fmt::Display for Value {
    /// Formats the `Value` as its variant type name (e.g., "none", "bool", "integer", "float", "string").
    ///
    /// # Examples
    ///
    /// ```
    /// let v = Value::Int(5);
    /// assert_eq!(format!("{}", v), "integer");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Value::None => "none",
            Value::Bool(_b) => "bool",
            Value::Int(_i) => "integer",
            Value::Float(_f) => "float",
            Value::Str(_s) => "string",
        };
        write!(f, "{name}")
    }
}