use std::collections::HashMap;
use crate::error::PalladError;
use crate::value::Value;
use crate::ir::Instr;

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
    /// Provide a short, human-readable name for the operation.
    ///
    /// The returned value is a `&'static str` describing the operation (e.g., `"add"`, `"power"`).
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::vm::Op;
    ///
    /// assert_eq!(Op::Add.name(), "add");
    /// assert_eq!(Op::Pow.name(), "power");
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
    /// - `UnaryTypeMismatch` for invalid unary operation type combinations (e.g., negating a string).
    /// - `NegationOverflow` when negating i64::MIN.
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
                Instr::LoadNone => self.stack.push(Value::None),
                Instr::LoadBool(b) => self.stack.push(Value::Bool(b)),
                Instr::LoadInt(n) => self.stack.push(Value::Int(n)),
                Instr::LoadFloat(f) => self.stack.push(Value::Float(f)),
                Instr::LoadStr(s) => self.stack.push(Value::Str(s)),
                Instr::LoadVar(name) => {
                    let val = self.globals.get(&name)
                        .cloned()
                        .ok_or(PalladError::UndefinedVariable { name: name.clone() })?;
                    self.stack.push(val);
                }
                Instr::StoreVar(name) => {
                    let val = self.stack.pop()
                        .ok_or(PalladError::StackUnderflow { operation: "store variable" })?;
                    self.globals.insert(name, val);
                }
                Instr::Add => {
                    self.execute_op(Op::Add)?;
                }
                Instr::Sub => {
                    self.execute_op(Op::Sub)?;
                }
                Instr::Mul => {
                    self.execute_op(Op::Mul)?;
                }
                Instr::Div => {
                    self.execute_op(Op::Div)?;
                }
                Instr::IntDiv => {
                    self.execute_op(Op::IntDiv)?;
                }
                Instr::Mod => {
                    self.execute_op(Op::Mod)?;
                }
                Instr::Pow => {
                    self.execute_op(Op::Pow)?;
                }
                Instr::And => {
                    self.execute_op(Op::And)?;
                }
                Instr::Or => {
                    self.execute_op(Op::Or)?;
                }
                Instr::Neg => {
                    self.execute_op(Op::Neg)?;
                }
                Instr::Not => {
                    self.execute_op(Op::Not)?;
                }
                Instr::CallBuiltin { name, argc } => {
                    if name == "print" {
                        let mut args = Vec::with_capacity(argc);
                        for _ in 0..argc {
                            args.push(self.stack.pop()
                                .ok_or(PalladError::StackUnderflow { operation: "print" })?);
                        }
                        for arg in args.into_iter().rev() {
                            match arg {
                                Value::None => println!("<none>"),
                                Value::Bool(b) => println!("{}", b),
                                Value::Int(n) => println!("{}", n),
                                Value::Float(f) => println!("{}", f),
                                Value::Str(s) => println!("{}", s),
                            }
                        }
                    } else {
                        return Err(PalladError::UnknownBuiltin { name });
                    }
                }
                Instr::Pop => {
                    self.stack.pop()
                        .ok_or(PalladError::StackUnderflow { operation: "Pop" })?;
                }
            }
        }
        Ok(())
    }

    fn execute_op(&mut self, op: Op) -> Result<(), PalladError> {
        let result = if matches!(op, Op::Neg | Op::Not) {
            self.pop_one_operand(op)?
        } else {
            self.pop_two_operands(op)?
        };
        self.stack.push(result);
        Ok(())
    }

    fn pop_one_operand(&mut self, op: Op) -> Result<Value, PalladError> {
        let v = self.stack.pop()
            .ok_or(PalladError::StackUnderflow { operation: op.name() })?;

        Ok(match (&v, &op) {
            // Valid operations:
            // Neg: int, float
            // Not: any (uses truthiness)
            
            // negative (-)
            (Value::Int(v), Op::Neg) => {
                v.checked_neg()
                    .map(Value::Int)
                    .ok_or(PalladError::NegationOverflow)?
            }
            (Value::Float(v), Op::Neg) => Value::Float(-v),

            // not (not)
            (v, Op::Not) => Value::Bool(!Self::value_is_true(v)),

            _ => return Err(PalladError::UnaryTypeMismatch {
                value: v,
                operation: op.name()
            }),
        })

    }

    /// Pop two values from the VM stack and compute the specified binary operation.
    ///
    /// The top of the stack is the right operand and the second-to-top is the left operand.
    /// Supported operations include addition, subtraction, multiplication, division, integer division, modulus, exponentiation, logical `and`, and logical `or`.
    ///
    /// # Parameters
    ///
    /// - `op`: The binary operation to apply to the two popped operands.
    ///
    /// # Returns
    ///
    /// `Ok(Value)` containing the result of applying `op` to the left and right operands, or an `Err(PalladError)` for stack underflow, division-by-zero, invalid type combinations, or other operation-specific errors.
    ///
    /// # Examples
    ///
    /// ```
    /// // Push 2 then 3 so left=2, right=3 for Add -> 5
    /// let mut vm = VM::new();
    /// vm.stack.push(Value::Int(2));
    /// vm.stack.push(Value::Int(3));
    /// let res = vm.pop_two_operands(Op::Add).expect("operation failed");
    /// assert_eq!(res, Value::Int(5));
    /// ```
    fn pop_two_operands(&mut self, op: Op) -> Result<Value, PalladError> {
        let b = self.stack.pop()
            .ok_or(PalladError::StackUnderflow { operation: op.name() })?;
        let a = self.stack.pop()
            .ok_or(PalladError::StackUnderflow { operation: op.name() })?;

        // Check for division by zero
        if matches!(op, Op::Div | Op::IntDiv | Op::Mod) {
            let is_zero = match &b {
                Value::Int(n) => *n == 0,
                Value::Float(f) => *f == 0.0,
                _ => false, // Others raise PalladError::TypeMismatch
            };
            if is_zero {
                return Err(PalladError::DivisionByZero { operation: op.name() });
            }
        }
        // Check for 0 ** 0
        if matches!(op, Op::Pow) {
            let left_is_zero = match &a {
                Value::Int(n) => *n == 0,
                Value::Float(f) => *f == 0.0,
                _ => false, // Others raise PalladError::TypeMismatch
            };
            let right_is_zero = match &b {
                Value::Int(n) => *n == 0,
                Value::Float(f) => *f == 0.0,
                _ => false, // Others raise PalladError::TypeMismatch
            };
            if left_is_zero && right_is_zero {
                return Err(PalladError::ZeroPowerZero);
            }
        }

        Ok(match (&a, &b, &op) {
            // 'none' is invalid in '+ - * / // % **' operations.
            // Other invalid operations:
            // string - any         any - string        int * string        float * string
            // string * float       string / any        any / string        string // any
            // any // string        string % any        any % string
            
            // add (+)
            // int
            (Value::Int(a), Value::Int(b), Op::Add) => Value::Int(a + b),
            (Value::Int(a), Value::Float(b), Op::Add) => Value::Float(*a as f64 + b),
            (Value::Int(a), Value::Str(b), Op::Add) => Value::Str(a.to_string() + b),
            // float
            (Value::Float(a), Value::Int(b), Op::Add) => Value::Float(a + *b as f64),
            (Value::Float(a), Value::Float(b), Op::Add) => Value::Float(a + b),
            (Value::Float(a), Value::Str(b), Op::Add) => Value::Str(a.to_string() + b),
            // string
            (Value::Str(a), Value::Int(b), Op::Add) => Value::Str(a.clone() + &b.to_string()),
            (Value::Str(a), Value::Float(b), Op::Add) => Value::Str(a.clone() + &b.to_string()),
            (Value::Str(a), Value::Str(b), Op::Add) => Value::Str(a.clone() + b),

            // subtract (-)
            // int
            (Value::Int(a), Value::Int(b), Op::Sub) => Value::Int(a - b),
            (Value::Int(a), Value::Float(b), Op::Sub) => Value::Float(*a as f64 - b),
            // float
            (Value::Float(a), Value::Int(b), Op::Sub) => Value::Float(a - *b as f64),
            (Value::Float(a), Value::Float(b), Op::Sub) => Value::Float(a - b),

            // multiply (*)
            // int
            (Value::Int(a), Value::Int(b), Op::Mul) => Value::Int(a * b),
            (Value::Int(a), Value::Float(b), Op::Mul) => Value::Float(*a as f64 * b),
            // float
            (Value::Float(a), Value::Int(b), Op::Mul) => Value::Float(a * *b as f64),
            (Value::Float(a), Value::Float(b), Op::Mul) => Value::Float(a * b),
            // string
            (Value::Str(a), Value::Int(b), Op::Mul) => {
                if *b < 0 {
                    return Err(PalladError::NegativeRepeat);
                }
                let count = *b as usize;
                // Check for overflow before allocating
                a.len()
                    .checked_mul(count)
                    .ok_or(PalladError::RepeatOverflow)?;
                Value::Str(a.repeat(count))
            },

            // divide (/)
            // int
            (Value::Int(a), Value::Int(b), Op::Div) => Value::Float(*a as f64 / *b as f64),
            (Value::Int(a), Value::Float(b), Op::Div) => Value::Float(*a as f64 / b),
            // float
            (Value::Float(a), Value::Int(b), Op::Div) => Value::Float(a / *b as f64),
            (Value::Float(a), Value::Float(b), Op::Div) => Value::Float(a / b),

            // integer-divide (//)
            // int
            (Value::Int(a), Value::Int(b), Op::IntDiv) => {
                a.checked_div(*b)
                    .map(Value::Int)
                    .ok_or(PalladError::IntDivOverflow)?
            }
            (Value::Int(a), Value::Float(b), Op::IntDiv) => {
                let result = (*a as f64 / b).floor();
                if result.is_finite() && result >= i64::MIN as f64 && result <= i64::MAX as f64 {
                    Value::Int(result as i64)
                } else {
                    return Err(PalladError::IntDivOverflow);
                }
            }
            // float
            (Value::Float(a), Value::Int(b), Op::IntDiv) => {
                let result = (a / *b as f64).floor();
                if result.is_finite() && result >= i64::MIN as f64 && result <= i64::MAX as f64 {
                    Value::Int(result as i64)
                } else {
                    return Err(PalladError::IntDivOverflow);
                }
            }
            (Value::Float(a), Value::Float(b), Op::IntDiv) => {
                let result = (a / b).floor();
                if result.is_finite() && result >= i64::MIN as f64 && result <= i64::MAX as f64 {
                    Value::Int(result as i64)
                } else {
                    return Err(PalladError::IntDivOverflow);
                }
            }

            // mod (%)
            // int
            (Value::Int(a), Value::Int(b), Op::Mod) => Value::Int(a % b),
            (Value::Int(a), Value::Float(b), Op::Mod) => Value::Float(*a as f64 % b),
            // float
            (Value::Float(a), Value::Int(b), Op::Mod) => Value::Float(a % *b as f64),
            (Value::Float(a), Value::Float(b), Op::Mod) => Value::Float(a % b),

            // power (**)
            // int
            (Value::Int(a), Value::Int(b), Op::Pow) => Value::Int(a.pow(*b as u32)),
            (Value::Int(a), Value::Float(b), Op::Pow) => Value::Float((*a as f64).powf(*b)),
            // float
            (Value::Float(a), Value::Int(b), Op::Pow) => Value::Float(a.powf(*b as f64)),
            (Value::Float(a), Value::Float(b), Op::Pow) => Value::Float(a.powf(*b)),

            // and (and)
            (a, b, Op::And) => Value::Bool(Self::value_is_true(a) && Self::value_is_true(b)),

            // or (or)
            (a, b, Op::Or) => Value::Bool(Self::value_is_true(a) || Self::value_is_true(b)),

            _ => return Err(PalladError::TypeMismatch {
                left: a,
                right: b,
                operation: op.name()
            }),
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
}