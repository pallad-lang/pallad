use crate::error::PalladError;
use crate::ir::Instr;
use crate::value::Value;
use std::collections::HashMap;

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
    /// Get the mnemonic name for this operation.
    ///
    /// The returned string is a short identifier for the operation (for example, `"add"` or `"not"`).
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::Op;
    ///
    /// assert_eq!(Op::Add.name(), "add");
    /// assert_eq!(Op::Not.name(), "not");
    /// ```
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
    /// Creates a new virtual machine instance.
    ///
    /// The VM is initialized with an empty operand stack and an empty globals map.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vm = VM::new();
    /// vm.run(vec![]).unwrap();
    /// ```
    pub fn new() -> Self {
        Self {
            stack: vec![],
            globals: HashMap::new(),
        }
    }

    /// Execute a sequence of instructions on the virtual machine, mutating its operand stack and global variables.
    ///
    /// Runs the provided `program` (a `Vec<Instr>`) instruction by instruction. Each instruction updates the VM's
    /// internal `stack` and/or `globals` as defined by the instruction semantics. The method terminates early and
    /// returns an error if any instruction causes a runtime failure.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the program completed without runtime errors. `Err(PalladError)` for runtime failures such as:
    /// `UndefinedVariable`, `StackUnderflow`, `DuplicateVariable`, `UnaryTypeMismatch`, `DivisionByZero`,
    /// `NegativeExponentOnInteger`, `IntegerOverflow`, `RepeatOverflow`, `TypeMismatch`, and `UnknownBuiltin`.
    /// Errors include the source line number for diagnostic context.
    ///
    /// # Examples
    ///
    /// ```
    /// use pallad_vm::{VM, Instr, Value};
    ///
    /// let mut vm = VM::new();
    /// let program = vec![
    ///     Instr::LoadInt { value: 1, line: 1 },
    ///     Instr::LoadInt { value: 2, line: 1 },
    ///     Instr::Add { line: 1 },
    /// ];
    ///
    /// vm.run(program).unwrap();
    /// assert_eq!(vm.stack.pop(), Some(Value::Int(3)));
    /// ```
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

    /// Execute a unary or binary operation and push its result onto the VM stack.
    ///
    /// The operation `op` determines whether one or two operands are popped from the stack;
    /// the computed result is pushed back onto the stack. The `line` is the source-line
    /// number attached to any error produced while evaluating the operation.
    ///
    /// # Parameters
    ///
    /// - `op`: the operation to execute.
    /// - `line`: source-line number for error reporting.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the operation succeeded and its result was pushed onto the stack, or
    /// a `PalladError` describing the failure (e.g., stack underflow, type mismatch,
    /// division-by-zero, overflow) with the given `line` context.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::{VM, Op, Value};
    ///
    /// let mut vm = VM::new();
    /// vm.stack.push(Value::Int(2));
    /// vm.stack.push(Value::Int(3));
    /// vm.execute_op(Op::Add, 10).unwrap();
    /// assert_eq!(vm.stack.pop(), Some(Value::Int(5)));
    /// ```
    fn execute_op(&mut self, op: Op, line: usize) -> Result<(), PalladError> {
        let result = if matches!(op, Op::Neg | Op::Not) {
            self.pop_one_operand(op, line)?
        } else {
            self.pop_two_operands(op, line)?
        };
        self.stack.push(result);
        Ok(())
    }

    /// Pops a single operand from the VM stack and applies a unary operation.
    ///
    /// The `line` parameter is used to annotate any error produced with source-line context.
    ///
    /// Returns the resulting `Value` for supported unary operations:
    /// - `Neg` negates `Int` or `Float` (integer negation reports `IntegerOverflow` on overflow).
    /// - `Not` returns a `Bool` whose value is the logical negation of the operand's truthiness.
    ///
    /// Errors:
    /// - `PalladError::StackUnderflow` if the stack is empty.
    /// - `PalladError::UnaryTypeMismatch` if the operand's type is incompatible with `op`.
    /// - `PalladError::IntegerOverflow` for integer negation overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vm = VM::new();
    /// vm.stack.push(Value::Int(42));
    /// let res = vm.pop_one_operand(Op::Neg, 1).unwrap();
    /// assert_eq!(res, Value::Int(-42));
    ///
    /// vm.stack.push(Value::Bool(true));
    /// let res = vm.pop_one_operand(Op::Not, 2).unwrap();
    /// assert_eq!(res, Value::Bool(false));
    /// ```
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

    /// Pops two operands from the VM stack, applies the binary `op`, and returns the resulting `Value`.
    ///
    /// The `line` argument is attached to any error produced to indicate the source location.
    ///
    /// # Returns
    ///
    /// The `Value` produced by applying `op` to the two top-most stack values (left operand is the value
    /// that was pushed earlier).
    ///
    /// # Errors
    ///
    /// Returns a `PalladError` with `line` context for the following conditions:
    /// - `StackUnderflow` if there are fewer than two values on the stack.
    /// - `DivisionByZero` for `Div`, `IntDiv`, or `Mod` when the right operand is zero.
    /// - `IntegerOverflow` when an integer arithmetic operation overflows or an integer result cannot be represented.
    /// - `NegativeRepeat` when repeating a string by a negative integer.
    /// - `RepeatOverflow` when repeating a string would overflow memory or conversion to `usize` fails.
    /// - `TypeMismatch` when the operand types are not compatible with the requested operation.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// // Push two integers and apply addition; the VM will produce the summed value.
    /// let mut vm = VM::new();
    /// // (in actual code, push values into the VM stack and call the binary operation)
    /// // vm.stack.push(Value::Int(1));
    /// // vm.stack.push(Value::Int(2));
    /// // let result = vm.pop_two_operands(Op::Add, 1).unwrap();
    /// // assert_eq!(result, Value::Int(3));
    /// ```
    fn pop_two_operands(&mut self, op: Op, line: usize) -> Result<Value, PalladError> {
        let b = self.stack.pop().ok_or(PalladError::StackUnderflow {
            operation: op.name(),
            line,
        })?;
        let a = self.stack.pop().ok_or(PalladError::StackUnderflow {
            operation: op.name(),
            line,
        })?;

        let both_numeric = matches!((&a, &b),
            (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_))
        );
        if both_numeric && matches!(op, Op::Div | Op::IntDiv | Op::Mod) {
            let is_zero = match &b {
                Value::Int(n) => *n == 0,
                Value::Float(f) => *f == 0.0,
                _ => false,
            };
            if is_zero {
                let left = match a {
                    Value::Int(i) => i.to_string(),
                    Value::Float(f) => f.to_string(),
                    _ => String::new(), // Other types filtered above
                };
                let right = match b {
                    Value::Int(i) => i.to_string(),
                    Value::Float(f) => f.to_string(),
                    _ => String::new(), // Other types filtered above
                };
                let operand = match op {
                    Op::Div => "/",
                    Op::IntDiv => "//",
                    Op::Mod => "%",
                    _ => "", // Other operands filtered above
                };
                return Err(PalladError::DivisionByZero {
                    operation: format!("{left} {} {right}", operand),
                    line,
                })
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
                let q = a.checked_div(*b)
                    .ok_or(PalladError::IntegerOverflow {
                        operation: format!("{a} // {b}"),
                        line,
                    })?;
                let r = a.checked_rem(*b)
                    .ok_or(PalladError::IntegerOverflow {
                        operation: format!("{a} // {b}"),
                        line,
                    })?;
                if (r != 0) && ((r > 0) != (*b > 0)) {
                    Value::Int(q - 1)
                } else {
                    Value::Int(q)
                }
            }
            (Value::Int(a), Value::Float(b), Op::IntDiv) => {
                let result = (*a as f64 / b).floor();
                if result.is_finite() && result >= (i64::MIN as f64) && result < ((i64::MAX as f64) + 1.0) {
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
                if result.is_finite() && result >= (i64::MIN as f64) && result < ((i64::MAX as f64) + 1.0) {
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
                if result.is_finite() && result >= (i64::MIN as f64) && result < ((i64::MAX as f64) + 1.0) {
                    Value::Int(result as i64)
                } else {
                    return Err(PalladError::IntegerOverflow {
                        operation: format!("{a} // {b}"),
                        line,
                    });
                }
            }

            (Value::Int(a), Value::Int(b), Op::Mod) => {
                let r = a.checked_rem(*b)
                    .ok_or(PalladError::IntegerOverflow {
                        operation: format!("{a} % {b}"),
                        line,
                    })?;
                let result = if (r != 0) && ((r > 0) != (*b > 0)) {
                    r + b
                } else {
                    r
                };
                Value::Int(result)
            }
            (Value::Int(a), Value::Float(b), Op::Mod) => Value::Float(((*a as f64 % b) + b) % b),
            (Value::Float(a), Value::Int(b), Op::Mod) => {
                Value::Float(((a % *b as f64) + *b as f64) % *b as f64)
            }
            (Value::Float(a), Value::Float(b), Op::Mod) => Value::Float(((a % b) + b) % b),

            (Value::Int(a), Value::Int(b), Op::Pow) => {
                if *b < 0 {
                    return Err(PalladError::NegativeExponentOnInteger {
                        operation: format!("{a} ** {b}"),
                        line
                    })
                }
                let exp = u32::try_from(*b).map_err(|_| PalladError::IntegerOverflow {
                    operation: format!("{a} ** {b}"),
                    line,
                })?;
                a.checked_pow(exp)
                    .map(Value::Int)
                    .ok_or(PalladError::IntegerOverflow {
                        operation: format!("{a} ** {b}"),
                        line,
                    })?
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

    /// Determines whether a `Value` is truthy.
    ///
    /// A `Value` is considered truthy when:
    /// - `Bool(true)`,
    /// - `Int` not equal to zero,
    /// - `Float` not equal to 0.0,
    /// - `Str` not empty.
    /// 
    /// `None` and the falsy variants above are considered false.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::Value;
    ///
    /// assert!(!value_is_true(&Value::None));
    /// assert!(value_is_true(&Value::Bool(true)));
    /// assert!(!value_is_true(&Value::Int(0)));
    /// assert!(value_is_true(&Value::Float(0.1)));
    /// assert!(value_is_true(&Value::Str(String::from("hi"))));
    /// ```
    fn value_is_true(value: &Value) -> bool {
        match value {
            Value::None => false,
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::Str(s) => !s.is_empty(),
        }
    }

    /// Calls a built-in function by name using arguments taken from the VM stack.
    ///
    /// Currently supports the `"print"` builtin, which prints the top `argc` values
    /// (from oldest to newest) to stdout and removes them from the stack.
    ///
    /// On success this returns `Ok(())`. Errors:
    /// - `PalladError::StackUnderflow { operation: "print", line }` if the stack
    ///   contains fewer than `argc` values.
    /// - `PalladError::UnknownBuiltin { name, line }` if `name` is not a recognized
    ///   builtin.
    ///
    /// The `line` parameter is used to attach source-line context to returned errors.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// # use crate::{VM, Value};
    /// let mut vm = VM::new();
    /// // push values to be printed
    /// vm.stack.push(Value::Int(42));
    /// vm.stack.push(Value::Str("hello".into()));
    /// // print two values (prints "42" then "hello") and removes them from the stack
    /// vm.call_builtin("print", 2, 1).unwrap();
    /// assert!(vm.stack.is_empty());
    /// ```
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