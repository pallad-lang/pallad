use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Value::Int(_i) => "Integer",
            Value::Float(_f) => "Float",
            Value::Str(_s) => "String",
        };
        write!(f, "{name}")
    }
}