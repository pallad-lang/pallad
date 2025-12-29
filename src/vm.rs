use std::collections::HashMap;
use crate::error::PalladError;
use crate::value::Value;
use crate::ir::Instr;

pub struct VM {
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
}

impl VM {
    /// Constructs a new VM with an empty operand stack and an empty global variable store.
    ///
    /// # Examples
    ///
    /// ```
    /// let _vm = VM::new();
    /// ```
    pub fn new() -> Self {
        Self {
            stack: vec![],
            globals: HashMap::new(),
        }
    }

    /// Executes a sequence of bytecode-like instructions on the virtual machine, updating the stack and globals.
    ///
    /// The VM processes each `Instr` in order, manipulating the operand stack and global variable store,
    /// performing arithmetic, variable access, built-in calls (currently `print`), and stack operations.
    ///
    /// # Errors
    ///
    /// Returns a `PalladError` when execution fails, including but not limited to:
    /// - `UndefinedVariable` if a `LoadVar` references a missing global.
    /// - `StackUnderflow` when an instruction requires more stack values than available.
    /// - `UnknownBuiltin` if `CallBuiltin` targets an unrecognized builtin.
    /// - `DivisionByZero` for division/modulo by zero.
    /// - `TypeMismatch` for unsupported operand type combinations (e.g., invalid types for `IntDiv`).
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::{VM, Instr, Value, PalladError};
    ///
    /// let mut vm = VM::new();
    /// let program = vec![
    ///     Instr::LoadInt(2),
    ///     Instr::LoadInt(3),
    ///     Instr::Add,
    ///     Instr::CallBuiltin { name: "print".to_string(), argc: 1 },
    /// ];
    ///
    /// assert!(vm.run(program).is_ok());
    /// ```
    pub fn run(&mut self, program: Vec<Instr>) -> Result<(), PalladError> {
        for instr in program {
            match instr {
                Instr::LoadInt(n) => self.stack.push(Value::Int(n)),
                Instr::LoadFloat(f) => self.stack.push(Value::Float(f)),
                Instr::LoadVar(name) => {
                    let val = self.globals.get(&name)
                        .cloned()
                        .ok_or_else(|| PalladError::UndefinedVariable { name: name.clone() })?;
                    self.stack.push(val);
                }
                Instr::StoreVar(name) => {
                    let val = self.stack.pop()
                        .ok_or_else(|| PalladError::StackUnderflow { operation: "StoreVar".to_string() })?;
                    self.globals.insert(name, val);
                }
                Instr::Add => {
                    self.execute_arithmetic("Add")?;
                }
                Instr::Sub => {
                    self.execute_arithmetic("Sub")?;
                }
                Instr::Mul => {
                    self.execute_arithmetic("Mul")?;
                }
                Instr::Div => {
                    self.execute_arithmetic("Div")?;
                }
                Instr::IntDiv => {
                    self.execute_arithmetic("IntDiv")?;
                }
                Instr::Mod => {
                    self.execute_arithmetic("Mod")?;
                }
                Instr::CallBuiltin { name, argc } => {
                    if name == "print" {
                        let mut args = Vec::with_capacity(argc);
                        for _ in 0..argc {
                            args.push(self.stack.pop()
                                .ok_or_else(|| PalladError::StackUnderflow { operation: "print".to_string() })?);
                        }
                        for arg in args.into_iter().rev() {
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

    /// Executes a binary arithmetic operation by popping two operands and
    /// pushing the resulting value back onto the stack.
    ///
    /// The `op_name` parameter is used for error reporting to indicate which
    /// operation caused a stack underflow in `pop_two_operands`.
    fn execute_arithmetic(&mut self, op_name: &str) -> Result<(), PalladError> {
        let result = self.pop_two_operands(op_name)?;
        self.stack.push(result);
        Ok(())
    }

    /// Pop two values from the VM stack and compute the binary operation identified by `op_name`.
    ///
    /// Supported operation names: `"Add"`, `"Sub"`, `"Mul"`, `"Div"`, `"IntDiv"`, and `"Mod"`.
    /// On success returns the resulting `Value` produced by applying the operation to the second-to-top
    /// stack value (left operand) and the top stack value (right operand).
    ///
    /// # Parameters
    ///
    /// - `op_name`: The operation to perform; must be one of the supported names above.
    ///
    /// # Returns
    ///
    /// The resulting `Value` for the performed operation, or an error for stack underflow, division by
    /// zero (for `Div`, `IntDiv`, `Mod`), or a type mismatch when operands are incompatible.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// // Minimal VM and Value setup for the example
    /// #[derive(Debug, PartialEq)]
    /// enum Value { Int(i64), Float(f64) }
    /// struct VM { stack: Vec<Value>, globals: HashMap<String, Value> }
    /// impl VM {
    ///     fn new() -> Self { VM { stack: Vec::new(), globals: HashMap::new() } }
    ///     fn pop_two_operands(&mut self, op_name: &str) -> Result<Value, String> {
    ///         let b = self.stack.pop().ok_or("underflow")?;
    ///         let a = self.stack.pop().ok_or("underflow")?;
    ///         match (a, b, op_name) {
    ///             (Value::Int(a), Value::Int(b), "Add") => Ok(Value::Int(a + b)),
    ///             (Value::Float(a), Value::Float(b), "Add") => Ok(Value::Float(a + b)),
    ///             _ => Err("type mismatch".to_string())
    ///         }
    ///     }
    /// }
    ///
    /// let mut vm = VM::new();
    /// vm.stack.push(Value::Int(2));
    /// vm.stack.push(Value::Int(3));
    /// let res = vm.pop_two_operands("Add").expect("operation failed");
    /// assert_eq!(res, Value::Int(5));
    /// ```
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