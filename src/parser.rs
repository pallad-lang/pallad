use crate::ast::{Expr, Stmt, BinOp};
use crate::lexer::Token;
use crate::error::PalladError;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    line: usize,
}

impl Parser {
    /// Create a new `Parser` for the given token stream.
    ///
    /// Initializes the parser with the provided tokens, sets the current position to 0, and starts the line counter at 1 for error reporting.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut parser = Parser::new(vec![]);
    /// // empty input produces no statements
    /// assert_eq!(parser.parse().unwrap().len(), 0);
    /// ```
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0, line: 1 }
    }

    /// Get a reference to the token at the parser's current position, if one exists.
    ///
    /// # Returns
    ///
    /// `Some(&Token)` with the current token, or `None` if the parser position is past the end of the token stream.
    ///
    /// # Examples
    ///
    /// ```
    /// let parser = Parser::new(vec![]);
    /// assert!(parser.current().is_none());
    /// ```
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    /// Advance the parser to the next token, incrementing `line` when the current token is `Token::Eol`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut p = Parser::new(vec![Token::Eol, Token::Int(1)]);
    /// assert_eq!(p.line, 1);
    /// p.advance();
    /// assert_eq!(p.line, 2);
    /// assert_eq!(p.current(), Some(&Token::Int(1)));
    /// ```
    fn advance(&mut self) {
        if let Some(Token::Eol) = self.current() {
            self.line += 1;
        }
        self.pos += 1;
    }

    /// Parses the parser's token stream into an abstract syntax tree of statements.
    ///
    /// The parser consumes tokens until the end of input and produces a vector of `Stmt`:
    /// 
    /// - `var <ident> = <expr>` produces `Stmt::Let { name, expr }`
    /// - `print(...)` produces `Stmt::Expr(Expr::Call { name: "print", args })`
    /// 
    /// Empty lines (Eol) are skipped. Syntax errors and premature end-of-input produce `PalladError`.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `Vec<Stmt>` on success, or a `PalladError` describing the syntax error and line on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::lexer::Token;
    /// use crate::parser::Parser;
    ///
    /// // tokens for: var x = 42
    /// let tokens = vec![Token::Var, Token::Ident("x".to_string()), Token::Eq, Token::Int(42), Token::Eol];
    /// let mut parser = Parser::new(tokens);
    /// let stmts = parser.parse().unwrap();
    /// assert_eq!(stmts.len(), 1);
    /// ```
    pub fn parse(&mut self) -> Result<Vec<Stmt>, PalladError> {
        let mut stmts = vec![];

        while let Some(tok) = self.current() {
            match tok {
                Token::Var => {
                    self.advance();
                    let var_name = match self.current() {
                        Some(Token::Ident(name)) => {
                            let n = name.clone();
                            self.advance();
                            n
                        }
                        Some(other) => {
                            return Err(PalladError::UnexpectedToken {
                                got: format!("{:?}", other),
                                expected: "identifier".to_string(),
                                line: self.line,
                            });
                        }
                        None => {
                            return Err(PalladError::EndOfInput {
                                expected: "identifier".to_string(),
                                line: self.line,
                            });
                        }
                    };

                    match self.current() {
                        Some(Token::Eq) => self.advance(),
                        Some(other) => {
                            return Err(PalladError::UnexpectedToken {
                                got: format!("{:?}", other),
                                expected: "'='".to_string(),
                                line: self.line,
                            });
                        }
                        None => {
                            return Err(PalladError::EndOfInput {
                                expected: "'='".to_string(),
                                line: self.line,
                            });
                        }
                    }

                    let expr = self.parse_expr()?;
                    stmts.push(Stmt::Let { name: var_name, expr });
                }

                Token::Print => {
                    self.advance();
                    match self.current() {
                        Some(Token::LParen) => self.advance(),
                        Some(other) => {
                            return Err(PalladError::UnexpectedToken {
                                got: format!("{:?}", other),
                                expected: "'('".to_string(),
                                line: self.line,
                            });
                        }
                        None => {
                            return Err(PalladError::EndOfInput {
                                expected: "'('".to_string(),
                                line: self.line,
                            });
                        }
                    }

                    let mut args = vec![];
                    if let Some(Token::RParen) = self.current() {
                        self.advance();
                    } else {
                        loop {
                            if let Some(Token::RParen) = self.current() {
                                self.advance();
                                break;
                            }
                            args.push(self.parse_expr()?);
                            match self.current() {
                                Some(Token::Comma) => { self.advance(); }
                                Some(Token::RParen) => { self.advance(); break; }
                                Some(other) => {
                                    return Err(PalladError::UnexpectedToken {
                                        got: format!("{:?}", other),
                                        expected: "',' or ')'".to_string(),
                                        line: self.line,
                                    });
                                }
                                None => {
                                    return Err(PalladError::EndOfInput {
                                        expected: "',' or ')'".to_string(),
                                        line: self.line,
                                    });
                                }
                            }
                        }
                    }

                    stmts.push(Stmt::Expr(Expr::Call { name: "print".to_string(), args }));
                }

                Token::Eol => { self.advance(); }

                other => {
                    return Err(PalladError::UnexpectedToken {
                        got: format!("{:?}", other),
                        expected: "'var', 'print', or end of line".to_string(),
                        line: self.line,
                    });
                }
            }
        }

        Ok(stmts)
    }

    /// Parses an expression starting at the parser's current token and returns its AST node.
    ///
    /// # Returns
    ///
    /// `Ok(Expr)` containing the parsed expression on success, or `Err(PalladError)` if parsing fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut parser = Parser::new(vec![Token::Int(42)]);
    /// let expr = parser.parse_expr().unwrap();
    /// assert_eq!(expr, Expr::Int(42));
    /// ```
    pub fn parse_expr(&mut self) -> Result<Expr, PalladError> {
        self.parse_add_sub()
    }

    /// Parses a left-associative chain of addition and subtraction expressions.
    ///
    /// Continues consuming `+` and `-` operators and their right-hand multiplicative operands until a non-additive token is reached.
    /// Returns the parsed `Expr` representing the combined expression (e.g., parsing `1 + 2 - 3` yields an expression equivalent to `((1 + 2) - 3)`).
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::parser::Parser;
    /// use crate::lexer::Token;
    ///
    /// let tokens = vec![Token::Int(1), Token::Plus, Token::Int(2), Token::Minus, Token::Int(3)];
    /// let mut parser = Parser::new(tokens);
    /// let expr = parser.parse_add_sub().unwrap();
    /// // `expr` now represents ((1 + 2) - 3)
    /// ```
    fn parse_add_sub(&mut self) -> Result<Expr, PalladError> {
        let mut left = self.parse_mul_div()?;

        while let Some(tok) = self.current() {
            left = match tok {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_mul_div()?;
                    Expr::Binary { left: Box::new(left), op: BinOp::Add, right: Box::new(right) }
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_mul_div()?;
                    Expr::Binary { left: Box::new(left), op: BinOp::Sub, right: Box::new(right) }
                }
                _ => break,
            }
        }

        Ok(left)
    }

    /// Parses multiplicative operations (`*`, `/`, `//`, `%`) with left-to-right associativity and returns the resulting expression.
    ///
    /// This consumes a leading factor and then repeatedly consumes any consecutive multiplicative operator and factor,
    /// building left-associative `Expr::Binary` nodes until a non-multiplicative token is encountered.
    ///
    /// # Examples
    ///
    /// ```
    /// // Parse `2 * 3`
    /// let mut parser = Parser::new(vec![Token::Int(2), Token::Star, Token::Int(3)]);
    /// let expr = parser.parse_mul_div().unwrap();
    /// match expr {
    ///     Expr::Binary { op: BinOp::Mul, .. } => (),
    ///     _ => panic!("expected multiplication binary expression"),
    /// }
    /// ```
    fn parse_mul_div(&mut self) -> Result<Expr, PalladError> {
        let mut left = self.parse_factor()?;

        while let Some(tok) = self.current() {
            left = match tok {
                Token::Star => {
                    self.advance();
                    let right = self.parse_factor()?;
                    Expr::Binary { left: Box::new(left), op: BinOp::Mul, right: Box::new(right) }
                }
                Token::Slash => {
                    self.advance();
                    let right = self.parse_factor()?;
                    Expr::Binary { left: Box::new(left), op: BinOp::Div, right: Box::new(right) }
                }
                Token::IntDiv => {
                    self.advance();
                    let right = self.parse_factor()?;
                    Expr::Binary { left: Box::new(left), op: BinOp::IntDiv, right: Box::new(right) }
                }
                Token::Mod => {
                    self.advance();
                    let right = self.parse_factor()?;
                    Expr::Binary { left: Box::new(left), op: BinOp::Mod, right: Box::new(right) }
                }
                _ => break,
            }
        }

        Ok(left)
    }

    /// Parses and returns a single factor: an integer, float, identifier, or a parenthesized expression.
    ///
    /// This handles one atomic expression unit used by higher-precedence parsing (numbers, variables, or `(expr)`).
    ///
    /// # Returns
    ///
    /// `Ok(Expr)` containing the parsed factor, or `Err(PalladError)` if the current token is unexpected or the input ends prematurely.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::parser::Parser;
    /// use crate::lexer::Token;
    /// use crate::ast::Expr;
    ///
    /// let mut p = Parser::new(vec![Token::Int(42)]);
    /// let expr = p.parse_expr().unwrap();
    /// assert!(matches!(expr, Expr::Int(42)));
    /// ```
    fn parse_factor(&mut self) -> Result<Expr, PalladError> {
        match self.current().cloned() {
            Some(Token::Minus) => {
                self.advance();
                let operand = self.parse_factor()?;
                Ok(Expr::Binary {
                    left: Box::new(Expr::Int(0)),
                    op: BinOp::Sub,
                    right: Box::new(operand),
                })
            }
            Some(Token::Int(n)) => { self.advance(); Ok(Expr::Int(n)) }
            Some(Token::Float(f)) => { self.advance(); Ok(Expr::Float(f)) }
            Some(Token::Str(s)) => { self.advance(); Ok(Expr::Str(s)) }
            Some(Token::Ident(name)) => { self.advance(); Ok(Expr::Var(name)) }
            Some(Token::LParen) => {
                self.advance();
                let expr = self.parse_expr()?;
                match self.current() {
                    Some(Token::RParen) => { self.advance(); Ok(expr) }
                    Some(other) => Err(PalladError::UnexpectedToken {
                        got: format!("{:?}", other),
                        expected: "')'".to_string(),
                        line: self.line,
                    }),
                    None => Err(PalladError::EndOfInput {
                        expected: "')'".to_string(),
                        line: self.line,
                    }),
                }
            }
            Some(tok) => Err(PalladError::UnexpectedToken {
                got: format!("{:?}", tok),
                expected: "integer, float, variable, or '('".to_string(),
                line: self.line,
            }),
            None => Err(PalladError::EndOfInput {
                expected: "integer, float, variable, or '('".to_string(),
                line: self.line,
            }),
        }
    }
}