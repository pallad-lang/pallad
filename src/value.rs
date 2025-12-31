use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    None,
    Int(i64),
    Float(f64),
    Str(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Value::None => "none",
            Value::Int(_i) => "integer",
            Value::Float(_f) => "float",
            Value::Str(_s) => "string",
        };
        write!(f, "{name}")
    }
}