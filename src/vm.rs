use std::collections::HashMap;
use crate::error::PalladError;
use crate::value::Value;
use crate::ir::Instr;

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
                Instr::LoadInt(n) => self.stack.push(Value::Int(n)),
                Instr::LoadFloat(f) => self.stack.push(Value::Float(f)),
                Instr::LoadVar(name) => {
                    let val = *self.globals.get(&name)
                        .ok_or_else(|| PalladError::UndefinedVariable { name: name.clone() })?;
                    self.stack.push(val);
                }
                Instr::StoreVar(name) => {
                    let val = self.stack.pop()
                        .ok_or_else(|| PalladError::StackUnderflow { operation: "StoreVar".to_string() })?;
                    self.globals.insert(name, val);
                }
                Instr::Add => {
                    let result = self.pop_two_operands("Add")?;
                    self.stack.push(result);
                }
                Instr::Sub => {
                    let result = self.pop_two_operands("Sub")?;
                    self.stack.push(result);
                }
                Instr::Mul => {
                    let result = self.pop_two_operands("Mul")?;
                    self.stack.push(result);
                }
                Instr::Div => {
                    let result = self.pop_two_operands("Div")?;
                    self.stack.push(result);
                }
                Instr::IntDiv => {
                    let result = self.pop_two_operands("IntDiv")?;
                    self.stack.push(result);
                }
                Instr::Mod => {
                    let result = self.pop_two_operands("Mod")?;
                    self.stack.push(result);
                }
                Instr::CallBuiltin { name, argc } => {
                    if name == "print" {
                        let mut args = Vec::with_capacity(argc);
                        for _ in 0..argc {
                            args.push(self.stack.pop()
                                .ok_or_else(|| PalladError::StackUnderflow { operation: "print".to_string() })?);
                        }
                        args.reverse();
                        for arg in args {
                            match arg {
                                Value::Int(n) => println!("{}", n),
                                Value::Float(f) => println!("{}", f),
                            }
                        }
                    } else {
                        return Err(PalladError::UnknownBuiltin { name });
                    }
                }
                Instr::Pop => {
                    self.stack.pop()
                        .ok_or_else(|| PalladError::StackUnderflow { operation: "Pop".to_string() })?;
                }
            }
        }
        Ok(())
    }

    fn pop_two_operands(&mut self, op_name: &str) -> Result<Value, PalladError> {
        let b = self.stack.pop()
            .ok_or_else(|| PalladError::StackUnderflow { operation: op_name.to_string() })?;
        let a = self.stack.pop()
            .ok_or_else(|| PalladError::StackUnderflow { operation: op_name.to_string() })?;

        // Check for division by zero
        if matches!(op_name, "Div" | "IntDiv" | "Mod") {
            let is_zero = match &b {
                Value::Int(n) => *n == 0,
                Value::Float(f) => *f == 0.0,
            };
            if is_zero {
                return Err(PalladError::DivisionByZero { operation: op_name.to_string() });
            }
        }

        Ok(match (a, b, op_name) {
            (Value::Int(a), Value::Int(b), "Add") => Value::Int(a + b),
            (Value::Int(a), Value::Float(b), "Add") => Value::Float(a as f64 + b),
            (Value::Float(a), Value::Int(b), "Add") => Value::Float(a + b as f64),
            (Value::Float(a), Value::Float(b), "Add") => Value::Float(a + b),

            (Value::Int(a), Value::Int(b), "Sub") => Value::Int(a - b),
            (Value::Int(a), Value::Float(b), "Sub") => Value::Float(a as f64 - b),
            (Value::Float(a), Value::Int(b), "Sub") => Value::Float(a - b as f64),
            (Value::Float(a), Value::Float(b), "Sub") => Value::Float(a - b),

            (Value::Int(a), Value::Int(b), "Mul") => Value::Int(a * b),
            (Value::Int(a), Value::Float(b), "Mul") => Value::Float(a as f64 * b),
            (Value::Float(a), Value::Int(b), "Mul") => Value::Float(a * b as f64),
            (Value::Float(a), Value::Float(b), "Mul") => Value::Float(a * b),

            (Value::Int(a), Value::Int(b), "Div") => Value::Float(a as f64 / b as f64),
            (Value::Int(a), Value::Float(b), "Div") => Value::Float(a as f64 / b),
            (Value::Float(a), Value::Int(b), "Div") => Value::Float(a / b as f64),
            (Value::Float(a), Value::Float(b), "Div") => Value::Float(a / b),

            (Value::Int(a), Value::Int(b), "IntDiv") => Value::Int(a / b),
            (Value::Int(a), Value::Float(b), "IntDiv") => {
                let result = (a as f64 / b).floor();
                if result.is_finite() && result >= i64::MIN as f64 && result <= i64::MAX as f64 {
                    Value::Int(result as i64)
                } else {
                    return Err(PalladError::TypeMismatch { operation: "IntDiv".to_string() });
                }
            }
            (Value::Float(a), Value::Int(b), "IntDiv") => {
                let result = (a / b as f64).floor();
                if result.is_finite() && result >= i64::MIN as f64 && result <= i64::MAX as f64 {
                    Value::Int(result as i64)
                } else {
                    return Err(PalladError::TypeMismatch { operation: "IntDiv".to_string() });
                }
            }
            (Value::Float(a), Value::Float(b), "IntDiv") => {
                let result = (a / b).floor();
                if result.is_finite() && result >= i64::MIN as f64 && result <= i64::MAX as f64 {
                    Value::Int(result as i64)
                } else {
                    return Err(PalladError::TypeMismatch { operation: "IntDiv".to_string() });
                }
            }

            (Value::Int(a), Value::Int(b), "Mod") => Value::Int(a % b),
            (Value::Int(a), Value::Float(b), "Mod") => Value::Float(a as f64 % b),
            (Value::Float(a), Value::Int(b), "Mod") => Value::Float(a % b as f64),
            (Value::Float(a), Value::Float(b), "Mod") => Value::Float(a % b),

            _ => return Err(PalladError::TypeMismatch { operation: op_name.to_string() }),
        })
    }
}
