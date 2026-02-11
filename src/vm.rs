use crate::error::PalladError;
use crate::ir::Instr;
use crate::value::Value;
use std::collections::HashMap;

const MAX_INT_EXPONENT: i64 = u32::MAX as i64;

enum Op {
    Add,
    Sub,
    Mul,
    Div,
    IntDiv,
    Mod,
    Pow,
    And,
    Or,
    Neg,
    Not,
}

impl Op {
    pub fn name(&self) -> &'static str {
        match self {
            Op::Add => "add",
            Op::Sub => "subtract",
            Op::Mul => "multiply",
            Op::Div => "divide",
            Op::IntDiv => "integer-divide",
            Op::Mod => "mod",
            Op::Pow => "power",
            Op::And => "and",
            Op::Or => "or",
            Op::Neg => "negate",
            Op::Not => "not",
        }
    }
}

pub struct VM {
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            globals: HashMap::new(),
        }
    }

    pub fn run(&mut self, program: Vec<Instr>) -> Result<(), PalladError> {
        for instr in program {
            match instr {
                Instr::LoadNone { line: _line } => self.stack.push(Value::None),
                Instr::LoadBool { value, line: _line } => self.stack.push(Value::Bool(value)),
                Instr::LoadInt { value, line: _line } => self.stack.push(Value::Int(value)),
                Instr::LoadFloat { value, line: _line } => self.stack.push(Value::Float(value)),
                Instr::LoadStr { value, line: _line } => self.stack.push(Value::Str(value)),
                Instr::LoadVar { name, line } => {
                    let val =
                        self.globals
                            .get(&name)
                            .cloned()
                            .ok_or(PalladError::UndefinedVariable {
                                name: name.clone(),
                                line,
                            })?;
                    self.stack.push(val);
                }
                Instr::StoreVar { name, line } => {
                    let val = self.stack.pop().ok_or(PalladError::StackUnderflow {
                        operation: "store variable",
                        line,
                    })?;
                    if self.globals.contains_key(&name) {
                        return Err(PalladError::DuplicateVariable {
                            name: name.clone(),
                            line,
                        });
                    }
                    self.globals.insert(name, val);
                }
                Instr::SetVar { name, line } => {
                    let val = self.stack.pop().ok_or(PalladError::StackUnderflow {
                        operation: "set variable",
                        line,
                    })?;
                    if !self.globals.contains_key(&name) {
                        return Err(PalladError::UndefinedVariable {
                            name: name.clone(),
                            line,
                        });
                    }
                    self.globals.insert(name, val);
                }
                Instr::Add { line } => self.execute_op(Op::Add, line)?,
                Instr::Sub { line } => self.execute_op(Op::Sub, line)?,
                Instr::Mul { line } => self.execute_op(Op::Mul, line)?,
                Instr::Div { line } => self.execute_op(Op::Div, line)?,
                Instr::IntDiv { line } => self.execute_op(Op::IntDiv, line)?,
                Instr::Mod { line } => self.execute_op(Op::Mod, line)?,
                Instr::Pow { line } => self.execute_op(Op::Pow, line)?,
                Instr::And { line } => self.execute_op(Op::And, line)?,
                Instr::Or { line } => self.execute_op(Op::Or, line)?,
                Instr::Neg { line } => self.execute_op(Op::Neg, line)?,
                Instr::Not { line } => self.execute_op(Op::Not, line)?,
                Instr::CallBuiltin { name, argc, line } => self.call_builtin(&name, argc, line)?,
                Instr::Pop { line } => {
                    self.stack.pop().ok_or(PalladError::StackUnderflow {
                        operation: "pop",
                        line,
                    })?;
                }
            }
        }
        Ok(())
    }

    fn execute_op(&mut self, op: Op, line: usize) -> Result<(), PalladError> {
        let result = if matches!(op, Op::Neg | Op::Not) {
            self.pop_one_operand(op, line)?
        } else {
            self.pop_two_operands(op, line)?
        };
        self.stack.push(result);
        Ok(())
    }

    fn pop_one_operand(&mut self, op: Op, line: usize) -> Result<Value, PalladError> {
        let v = self.stack.pop().ok_or(PalladError::StackUnderflow {
            operation: op.name(),
            line,
        })?;

        Ok(match (&v, &op) {
            (Value::Int(v), Op::Neg) => {
                v.checked_neg()
                    .map(Value::Int)
                    .ok_or(PalladError::IntegerOverflow {
                        operation: format!("- {v}"),
                        line,
                    })?
            }
            (Value::Float(v), Op::Neg) => Value::Float(-v),
            (v, Op::Not) => Value::Bool(!Self::value_is_true(v)),
            _ => {
                return Err(PalladError::UnaryTypeMismatch {
                    value: v,
                    operation: op.name(),
                    line,
                });
            }
        })
    }

    fn pop_two_operands(&mut self, op: Op, line: usize) -> Result<Value, PalladError> {
        let b = self.stack.pop().ok_or(PalladError::StackUnderflow {
            operation: op.name(),
            line,
        })?;
        let a = self.stack.pop().ok_or(PalladError::StackUnderflow {
            operation: op.name(),
            line,
        })?;

        if matches!(op, Op::Div | Op::IntDiv | Op::Mod) {
            let is_zero = match &b {
                Value::Int(n) => *n == 0,
                Value::Float(f) => *f == 0.0,
                _ => false,
            };
            if is_zero {
                return Err(PalladError::DivisionByZero {
                    operation: op.name(),
                    line,
                });
            }
        }

        if matches!(op, Op::Pow) {
            let left_is_zero = match &a {
                Value::Int(n) => *n == 0,
                Value::Float(f) => *f == 0.0,
                _ => false,
            };
            let right_is_zero = match &b {
                Value::Int(n) => *n == 0,
                Value::Float(f) => *f == 0.0,
                _ => false,
            };
            if left_is_zero && right_is_zero {
                return Err(PalladError::ZeroPowerZero { line });
            }
        }

        Ok(match (&a, &b, &op) {
            (Value::Int(a), Value::Int(b), Op::Add) => {
                a.checked_add(*b)
                    .map(Value::Int)
                    .ok_or(PalladError::IntegerOverflow {
                        operation: format!("{a} + {b}"),
                        line,
                    })?
            }
            (Value::Int(a), Value::Float(b), Op::Add) => Value::Float(*a as f64 + b),
            (Value::Int(a), Value::Str(b), Op::Add) => Value::Str(a.to_string() + b),
            (Value::Float(a), Value::Int(b), Op::Add) => Value::Float(a + *b as f64),
            (Value::Float(a), Value::Float(b), Op::Add) => Value::Float(a + b),
            (Value::Float(a), Value::Str(b), Op::Add) => Value::Str(a.to_string() + b),
            (Value::Str(a), Value::Int(b), Op::Add) => Value::Str(a.clone() + &b.to_string()),
            (Value::Str(a), Value::Float(b), Op::Add) => Value::Str(a.clone() + &b.to_string()),
            (Value::Str(a), Value::Str(b), Op::Add) => Value::Str(a.clone() + b),

            (Value::Int(a), Value::Int(b), Op::Sub) => {
                a.checked_sub(*b)
                    .map(Value::Int)
                    .ok_or(PalladError::IntegerOverflow {
                        operation: format!("{a} - {b}"),
                        line,
                    })?
            }
            (Value::Int(a), Value::Float(b), Op::Sub) => Value::Float(*a as f64 - b),
            (Value::Float(a), Value::Int(b), Op::Sub) => Value::Float(a - *b as f64),
            (Value::Float(a), Value::Float(b), Op::Sub) => Value::Float(a - b),

            (Value::Int(a), Value::Int(b), Op::Mul) => {
                a.checked_mul(*b)
                    .map(Value::Int)
                    .ok_or(PalladError::IntegerOverflow {
                        operation: format!("{a} * {b}"),
                        line,
                    })?
            }
            (Value::Int(a), Value::Float(b), Op::Mul) => Value::Float(*a as f64 * b),
            (Value::Float(a), Value::Int(b), Op::Mul) => Value::Float(a * *b as f64),
            (Value::Float(a), Value::Float(b), Op::Mul) => Value::Float(a * b),
            (Value::Str(a), Value::Int(b), Op::Mul) => {
                if *b < 0 {
                    return Err(PalladError::NegativeRepeat { line });
                }
                let count =
                    usize::try_from(*b).map_err(|_| PalladError::RepeatOverflow { line })?;
                a.len()
                    .checked_mul(count)
                    .ok_or(PalladError::RepeatOverflow { line })?;
                Value::Str(a.repeat(count))
            }

            (Value::Int(a), Value::Int(b), Op::Div) => Value::Float(*a as f64 / *b as f64),
            (Value::Int(a), Value::Float(b), Op::Div) => Value::Float(*a as f64 / b),
            (Value::Float(a), Value::Int(b), Op::Div) => Value::Float(a / *b as f64),
            (Value::Float(a), Value::Float(b), Op::Div) => Value::Float(a / b),

            (Value::Int(a), Value::Int(b), Op::IntDiv) => {
                a.checked_div(*b)
                    .map(Value::Int)
                    .ok_or(PalladError::IntegerOverflow {
                        operation: format!("{a} // {b}"),
                        line,
                    })?
            }
            (Value::Int(a), Value::Float(b), Op::IntDiv) => {
                let result = (*a as f64 / b).floor();
                if result.is_finite() && result >= i64::MIN as f64 && result <= i64::MAX as f64 {
                    Value::Int(result as i64)
                } else {
                    return Err(PalladError::IntegerOverflow {
                        operation: format!("{a} // {b}"),
                        line,
                    });
                }
            }
            (Value::Float(a), Value::Int(b), Op::IntDiv) => {
                let result = (a / *b as f64).floor();
                if result.is_finite() && result >= i64::MIN as f64 && result <= i64::MAX as f64 {
                    Value::Int(result as i64)
                } else {
                    return Err(PalladError::IntegerOverflow {
                        operation: format!("{a} // {b}"),
                        line,
                    });
                }
            }
            (Value::Float(a), Value::Float(b), Op::IntDiv) => {
                let result = (a / b).floor();
                if result.is_finite() && result >= i64::MIN as f64 && result <= i64::MAX as f64 {
                    Value::Int(result as i64)
                } else {
                    return Err(PalladError::IntegerOverflow {
                        operation: format!("{a} // {b}"),
                        line,
                    });
                }
            }

            (Value::Int(a), Value::Int(b), Op::Mod) => {
                a.checked_rem(*b)
                    .map(Value::Int)
                    .ok_or(PalladError::IntegerOverflow {
                        operation: format!("{a} % {b}"),
                        line,
                    })?
            }
            (Value::Int(a), Value::Float(b), Op::Mod) => Value::Float(*a as f64 % b),
            (Value::Float(a), Value::Int(b), Op::Mod) => Value::Float(a % *b as f64),
            (Value::Float(a), Value::Float(b), Op::Mod) => Value::Float(a % b),

            (Value::Int(a), Value::Int(b), Op::Pow) => {
                if *b < 0 || *b > MAX_INT_EXPONENT {
                    Value::Float((*a as f64).powf(*b as f64))
                } else {
                    a.checked_pow(*b as u32)
                        .map(Value::Int)
                        .unwrap_or_else(|| Value::Float((*a as f64).powf(*b as f64)))
                }
            }
            (Value::Int(a), Value::Float(b), Op::Pow) => Value::Float((*a as f64).powf(*b)),
            (Value::Float(a), Value::Int(b), Op::Pow) => Value::Float(a.powf(*b as f64)),
            (Value::Float(a), Value::Float(b), Op::Pow) => Value::Float(a.powf(*b)),

            (a, b, Op::And) => Value::Bool(Self::value_is_true(a) && Self::value_is_true(b)),
            (a, b, Op::Or) => Value::Bool(Self::value_is_true(a) || Self::value_is_true(b)),

            _ => {
                return Err(PalladError::TypeMismatch {
                    left: a,
                    right: b,
                    operation: op.name(),
                    line,
                });
            }
        })
    }

    fn value_is_true(value: &Value) -> bool {
        match value {
            Value::None => false,
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::Str(s) => !s.is_empty(),
        }
    }

    fn call_builtin(&mut self, name: &str, argc: usize, line: usize) -> Result<(), PalladError> {
        match name {
            "print" => {
                if self.stack.len() < argc {
                    return Err(PalladError::StackUnderflow {
                        operation: "print",
                        line,
                    });
                }
                let start = self.stack.len() - argc;
                for i in start..self.stack.len() {
                    match &self.stack[i] {
                        Value::None => println!("<none>"),
                        Value::Bool(b) => println!("{}", b),
                        Value::Int(n) => println!("{}", n),
                        Value::Float(f) => println!("{}", f),
                        Value::Str(s) => println!("{}", s),
                    }
                }
                self.stack.truncate(start);
            }
            _ => {
                return Err(PalladError::UnknownBuiltin {
                    name: name.to_string(),
                    line,
                });
            }
        }
        Ok(())
    }
}
