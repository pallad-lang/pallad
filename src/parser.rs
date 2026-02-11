use crate::ast::{BinOp, Expr, Stmt, UnOp};
use crate::error::PalladError;
use crate::lexer::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    current_pos: usize,
}

impl Parser {
    /// Creates a new Parser initialized with the provided token stream.
    ///
    /// The parser's position is set to the start of the token vector (index 0).
    ///
    /// # Examples
    ///
    /// ```
    /// let tokens = Vec::<crate::lexer::Token>::new();
    /// let _parser = crate::parser::Parser::new(tokens);
    /// ```
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_pos: 0,
        }
    }

    /// Get the token at the parser's current position.
    ///
    /// # Returns
    ///
    /// `Some(&Token)` for the token at the current position, or `None` if the position is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// // assuming `tokens` is a Vec<Token> and `parser` is created with Parser::new(tokens)
    /// if let Some(tok) = parser.current() {
    ///     // inspect tok
    /// }
    /// ```
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.current_pos)
    }

    /// Get the line number of the current token, falling back to the last token's line or `1` if no tokens exist.
    ///
    /// # Returns
    ///
    /// `usize` — the line number associated with the current token, or the last token's line, or `1` when there are no tokens.
    ///
    /// # Examples
    ///
    /// ```
    /// let p = Parser::new(Vec::new());
    /// assert_eq!(p.current_line(), 1);
    /// ```
    fn current_line(&self) -> usize {
        self.current()
            .map(|t| t.line)
            .unwrap_or_else(|| self.tokens.last().map(|t| t.line).unwrap_or(1))
    }

    /// Advances the parser's position to consume the current token.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut p = Parser::new(Vec::new());
    /// let before = p.current_pos;
    /// p.advance();
    /// assert_eq!(p.current_pos, before + 1);
    /// ```
    fn advance(&mut self) {
        self.current_pos += 1;
    }

    /// Parses the token stream into a sequence of statements (AST).
    ///
    /// Returns a vector of parsed `Stmt` values or a `PalladError` if parsing fails,
    /// with each AST node annotated with its source line for error reporting.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut parser = Parser::new(vec![]);
    /// let stmts = parser.parse().unwrap();
    /// assert!(stmts.is_empty());
    /// ```
    pub fn parse(&mut self) -> Result<Vec<Stmt>, PalladError> {
        let mut stmts = vec![];

        while let Some(tok) = self.current() {
            match &tok.kind {
                TokenKind::Var | TokenKind::Ident(_) => {
                    let stmt_line = tok.line;
                    let is_let = matches!(&tok.kind, TokenKind::Var);
                    if is_let {
                        self.advance();
                    }

                    let var_name = match self.current() {
                        Some(Token {
                            kind: TokenKind::Ident(name),
                            ..
                        }) => {
                            let n = name.clone();
                            self.advance();
                            n
                        }
                        Some(other) => {
                            return Err(PalladError::UnexpectedToken {
                                got: format!("{:?}", other.kind),
                                expected: "identifier".to_string(),
                                line: other.line,
                            });
                        }
                        None => {
                            return Err(PalladError::EndOfInput {
                                expected: "identifier".to_string(),
                                line: stmt_line,
                            });
                        }
                    };

                    let expr = match self.current() {
                        Some(Token {
                            kind: TokenKind::Eq,
                            ..
                        }) => {
                            self.advance();
                            self.parse_expr()?
                        }
                        Some(Token {
                            kind: TokenKind::Eol,
                            ..
                        }) if is_let => Expr::None { line: stmt_line },
                        Some(other) => {
                            return Err(PalladError::UnexpectedToken {
                                got: format!("{:?}", other.kind),
                                expected: if is_let { "'=' or end of line" } else { "'='" }
                                    .to_string(),
                                line: other.line,
                            });
                        }
                        None => {
                            if is_let {
                                Expr::None { line: stmt_line }
                            } else {
                                return Err(PalladError::EndOfInput {
                                    expected: "'='".to_string(),
                                    line: stmt_line,
                                });
                            }
                        }
                    };

                    if is_let {
                        stmts.push(Stmt::Let {
                            name: var_name,
                            expr,
                            line: stmt_line,
                        });
                    } else {
                        stmts.push(Stmt::Set {
                            name: var_name,
                            expr,
                            line: stmt_line,
                        });
                    }
                }
                TokenKind::Print => {
                    let stmt_line = tok.line;
                    self.advance();
                    match self.current() {
                        Some(Token {
                            kind: TokenKind::LParen,
                            ..
                        }) => self.advance(),
                        Some(other) => {
                            return Err(PalladError::UnexpectedToken {
                                got: format!("{:?}", other.kind),
                                expected: "'('".to_string(),
                                line: other.line,
                            });
                        }
                        None => {
                            return Err(PalladError::EndOfInput {
                                expected: "'('".to_string(),
                                line: stmt_line,
                            });
                        }
                    }

                    let mut args = vec![];
                    if let Some(Token {
                        kind: TokenKind::RParen,
                        ..
                    }) = self.current()
                    {
                        self.advance();
                    } else {
                        loop {
                            if let Some(Token {
                                kind: TokenKind::RParen,
                                ..
                            }) = self.current()
                            {
                                self.advance();
                                break;
                            }
                            args.push(self.parse_expr()?);
                            match self.current() {
                                Some(Token {
                                    kind: TokenKind::Comma,
                                    ..
                                }) => self.advance(),
                                Some(Token {
                                    kind: TokenKind::RParen,
                                    ..
                                }) => {
                                    self.advance();
                                    break;
                                }
                                Some(other) => {
                                    return Err(PalladError::UnexpectedToken {
                                        got: format!("{:?}", other.kind),
                                        expected: "',' or ')'".to_string(),
                                        line: other.line,
                                    });
                                }
                                None => {
                                    return Err(PalladError::EndOfInput {
                                        expected: "',' or ')'".to_string(),
                                        line: stmt_line,
                                    });
                                }
                            }
                        }
                    }

                    let call = Expr::Call {
                        name: "print".to_string(),
                        args,
                        line: stmt_line,
                    };
                    stmts.push(Stmt::Expr {
                        expr: call,
                        line: stmt_line,
                    });
                }
                TokenKind::Str(_) => {
                    self.advance();
                    match self.current() {
                        Some(Token {
                            kind: TokenKind::Eol,
                            ..
                        }) => self.advance(),
                        None => {}
                        Some(other) => {
                            return Err(PalladError::UnexpectedToken {
                                got: format!("{:?}", other.kind),
                                expected: "end of line".to_string(),
                                line: other.line,
                            });
                        }
                    }
                }
                TokenKind::Eol => self.advance(),
                other => {
                    return Err(PalladError::UnexpectedToken {
                        got: format!("{:?}", other),
                        expected:
                            "'var', identifier, 'print', string literal comment, or end of line"
                                .to_string(),
                        line: tok.line,
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
    /// `Expr` representing the parsed expression on success, or a `PalladError` describing the parse failure.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crate::parser::Parser;
    /// use crate::lexer::{Token, TokenKind};
    ///
    /// // Construct a parser with tokens for a simple expression (example only).
    /// let tokens = vec![Token { kind: TokenKind::Int(42), line: 1 }];
    /// let mut parser = Parser::new(tokens);
    /// let expr = parser.parse_expr().expect("failed to parse expression");
    /// ```
    pub fn parse_expr(&mut self) -> Result<Expr, PalladError> {
        self.parse_or()
    }

    /// Parses a left-associative chain of logical OR expressions and returns the resulting expression AST.
    ///
    /// Continues consuming `Or` tokens and combines parsed subexpressions into nested `Expr::Binary` nodes with `BinOp::Or`.
    ///
    /// # Returns
    ///
    /// `Expr` representing the parsed OR expression.
    ///
    /// # Examples
    ///
    /// ```
    /// // assuming `tokens` is a Vec<Token> representing the expression `a or b or c`
    /// let mut parser = Parser::new(tokens);
    /// let expr = parser.parse_or().unwrap();
    /// assert!(matches!(expr, Expr::Binary { op: BinOp::Or, .. }));
    /// ```
    fn parse_or(&mut self) -> Result<Expr, PalladError> {
        let mut left = self.parse_and()?;
        while matches!(self.current().map(|t| &t.kind), Some(TokenKind::Or)) {
            let line = self.current_line();
            self.advance();
            let right = self.parse_and()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinOp::Or,
                right: Box::new(right),
                line,
            };
        }
        Ok(left)
    }

    /// Parses a left-associative sequence of logical AND expressions into an `Expr`.
    ///
    /// Repeatedly consumes `&&`-style `And` tokens and combines the parsed subexpressions
    /// into `Expr::Binary` nodes using `BinOp::And`, preserving the line number from the
    /// operator token for each combined node.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Assuming `parser` is a `Parser` positioned at the start of an AND expression:
    /// let expr = parser.parse_and().unwrap();
    /// ```
    fn parse_and(&mut self) -> Result<Expr, PalladError> {
        let mut left = self.parse_not()?;
        while matches!(self.current().map(|t| &t.kind), Some(TokenKind::And)) {
            let line = self.current_line();
            self.advance();
            let right = self.parse_not()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinOp::And,
                right: Box::new(right),
                line,
            };
        }
        Ok(left)
    }

    /// Parses a logical NOT prefix expression or the next higher-precedence expression.
    ///
    /// If the current token is a `not`, returns an `Expr::Unary` with `UnOp::Not`,
    /// the operand parsed recursively, and the node's `line` set to the `not` token's line.
    /// Otherwise returns the next parsed expression.
    ///
    /// # Returns
    ///
    /// `Expr::Unary` with `UnOp::Not` and the parsed operand when a leading `not` is present; otherwise the next parsed `Expr`.
    ///
    /// # Examples
    ///
    /// ```
    /// // Given a tokenizer that produces tokens for the input "not true"
    /// // let tokens = crate::lexer::tokenize("not true");
    /// // let mut parser = crate::parser::Parser::new(tokens);
    /// // let expr = parser.parse_not().unwrap();
    /// // matches!(expr, crate::ast::Expr::Unary { .. });
    /// ```
    fn parse_not(&mut self) -> Result<Expr, PalladError> {
        if matches!(self.current().map(|t| &t.kind), Some(TokenKind::Not)) {
            let line = self.current_line();
            self.advance();
            let expr = self.parse_not()?;
            Ok(Expr::Unary {
                op: UnOp::Not,
                expr: Box::new(expr),
                line,
            })
        } else {
            self.parse_add_sub()
        }
    }

    /// Parses a left-associative sequence of addition and subtraction operations.
    ///
    /// Starts by parsing a multiplicative expression and then consumes any following `+` or `-`
    /// operators, combining them into `Expr::Binary` nodes with `BinOp::Add` or `BinOp::Sub`.
    ///
    /// # Returns
    ///
    /// The parsed `Expr` representing the combined addition/subtraction expression.
    fn parse_add_sub(&mut self) -> Result<Expr, PalladError> {
        let mut left = self.parse_mul_div()?;
        while let Some(tok) = self.current() {
            left = match tok.kind {
                TokenKind::Plus => {
                    let line = tok.line;
                    self.advance();
                    let right = self.parse_mul_div()?;
                    Expr::Binary {
                        left: Box::new(left),
                        op: BinOp::Add,
                        right: Box::new(right),
                        line,
                    }
                }
                TokenKind::Minus => {
                    let line = tok.line;
                    self.advance();
                    let right = self.parse_mul_div()?;
                    Expr::Binary {
                        left: Box::new(left),
                        op: BinOp::Sub,
                        right: Box::new(right),
                        line,
                    }
                }
                _ => break,
            }
        }
        Ok(left)
    }

    /// Parses left-associative multiplication, division, integer-division, and modulus expressions.
    ///
    /// This consumes any sequence of `*`, `/`, `//` (IntDiv), and `%` (Mod) operators and their
    /// right-hand operands, producing nested `Expr::Binary` nodes with the operator's token line
    /// recorded on each binary node.
    ///
    /// # Returns
    ///
    /// The parsed `Expr` representing the (possibly nested) binary expression.
    ///
    /// # Examples
    ///
    /// ```
    /// // Tokens for the expression `2 * 3` (illustrative; types come from the crate's lexer)
    /// use crate::{Parser, Token, TokenKind};
    ///
    /// let tokens = vec![
    ///     Token::new_int(2, 1),
    ///     Token::new_kind(TokenKind::Star, 1),
    ///     Token::new_int(3, 1),
    /// ];
    ///
    /// let mut parser = Parser::new(tokens);
    /// let expr = parser.parse_mul_div().unwrap();
    /// // `expr` is an Expr::Binary representing `2 * 3`
    /// ```
    fn parse_mul_div(&mut self) -> Result<Expr, PalladError> {
        let mut left = self.parse_unary()?;
        while let Some(tok) = self.current() {
            left = match tok.kind {
                TokenKind::Star => {
                    let line = tok.line;
                    self.advance();
                    let right = self.parse_unary()?;
                    Expr::Binary {
                        left: Box::new(left),
                        op: BinOp::Mul,
                        right: Box::new(right),
                        line,
                    }
                }
                TokenKind::Slash => {
                    let line = tok.line;
                    self.advance();
                    let right = self.parse_unary()?;
                    Expr::Binary {
                        left: Box::new(left),
                        op: BinOp::Div,
                        right: Box::new(right),
                        line,
                    }
                }
                TokenKind::IntDiv => {
                    let line = tok.line;
                    self.advance();
                    let right = self.parse_unary()?;
                    Expr::Binary {
                        left: Box::new(left),
                        op: BinOp::IntDiv,
                        right: Box::new(right),
                        line,
                    }
                }
                TokenKind::Mod => {
                    let line = tok.line;
                    self.advance();
                    let right = self.parse_unary()?;
                    Expr::Binary {
                        left: Box::new(left),
                        op: BinOp::Mod,
                        right: Box::new(right),
                        line,
                    }
                }
                _ => break,
            }
        }
        Ok(left)
    }

    /// Parse a unary expression, handling a leading unary minus as numeric negation.
    ///
    /// If the current token is a minus, consumes it and returns an `Expr::Unary` with `UnOp::Neg`
    /// applied to the recursively parsed operand; otherwise delegates to parsing exponentiation.
    ///
    /// # Returns
    ///
    /// The parsed `Expr` for the unary or power expression.
    ///
    /// # Examples
    ///
    /// ```
    /// // Example (illustrative): parsing "-42" yields a unary negation node.
    /// # use crate::parser::Parser;
    /// # use crate::lexer::{Token, TokenKind};
    /// # use crate::ast::{Expr, UnOp};
    /// # let tokens = vec![Token::new(TokenKind::Minus, 1), Token::new(TokenKind::Int(42), 1)];
    /// # let mut p = Parser::new(tokens);
    /// let expr = p.parse_unary().unwrap();
    /// // `expr` will be `Expr::Unary { op: UnOp::Neg, expr: Box::new(Expr::Int { value: 42, line: 1 }), line: 1 }`
    /// ```
    fn parse_unary(&mut self) -> Result<Expr, PalladError> {
        if matches!(self.current().map(|t| &t.kind), Some(TokenKind::Minus)) {
            let line = self.current_line();
            self.advance();
            let expr = self.parse_unary()?;
            Ok(Expr::Unary {
                op: UnOp::Neg,
                expr: Box::new(expr),
                line,
            })
        } else {
            self.parse_pow()
        }
    }

    /// Parses an exponentiation expression, treating the `^` operator as right-associative.
    ///
    /// Parses a base factor and, if a `Pow` token (`^`) follows, parses the right-hand side
    /// recursively to produce a right-associative binary `Expr::Binary` with `BinOp::Pow`.
    ///
    /// # Returns
    ///
    /// The parsed `Expr` with line information, or a `PalladError` if parsing fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Right-associative: a ^ b ^ c -> a ^ (b ^ c)
    /// let tokens = vec![ /* tokens representing "a ^ b ^ c" */ ];
    /// let mut parser = Parser::new(tokens);
    /// let expr = parser.parse_pow().unwrap();
    /// match expr {
    ///     Expr::Binary { op: BinOp::Pow, right, .. } => {
    ///         // `right` should be another `Expr::Binary` for the nested exponentiation
    ///     }
    ///     _ => panic!("expected exponentiation"),
    /// }
    /// ```
    fn parse_pow(&mut self) -> Result<Expr, PalladError> {
        let left = self.parse_factor()?;
        if matches!(self.current().map(|t| &t.kind), Some(TokenKind::Pow)) {
            let line = self.current_line();
            self.advance();
            let right = self.parse_pow()?;
            Ok(Expr::Binary {
                left: Box::new(left),
                op: BinOp::Pow,
                right: Box::new(right),
                line,
            })
        } else {
            Ok(left)
        }
    }

    /// Parses a primary factor: literals, identifiers, or a parenthesized expression.
    ///
    /// Recognizes `None`, boolean, integer, float, and string literals, identifier references,
    /// and grouped expressions `( ... )`. Advances the parser past the consumed tokens and
    /// attaches the source line to the produced `Expr`.
    ///
    /// # Returns
    ///
    /// An `Expr` representing the parsed literal, variable, or grouped expression on success.
    ///
    /// # Errors
    ///
    /// Returns `PalladError::UnexpectedToken` if the current token is not a valid factor,
    /// or `PalladError::EndOfInput` if the input ends where a value or closing parenthesis is expected.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::lexer::{Token, TokenKind};
    /// use crate::parser::Parser;
    /// use crate::ast::Expr;
    ///
    /// let tokens = vec![Token { kind: TokenKind::Int(42), line: 1 }];
    /// let mut parser = Parser::new(tokens);
    /// let expr = parser.parse_factor().unwrap();
    /// assert!(matches!(expr, Expr::Int { value: 42, line: 1 }));
    /// ```
    fn parse_factor(&mut self) -> Result<Expr, PalladError> {
        match self.current().cloned() {
            Some(Token {
                kind: TokenKind::None,
                line,
            }) => {
                self.advance();
                Ok(Expr::None { line })
            }
            Some(Token {
                kind: TokenKind::Bool(value),
                line,
            }) => {
                self.advance();
                Ok(Expr::Bool { value, line })
            }
            Some(Token {
                kind: TokenKind::Int(value),
                line,
            }) => {
                self.advance();
                Ok(Expr::Int { value, line })
            }
            Some(Token {
                kind: TokenKind::Float(value),
                line,
            }) => {
                self.advance();
                Ok(Expr::Float { value, line })
            }
            Some(Token {
                kind: TokenKind::Str(value),
                line,
            }) => {
                self.advance();
                Ok(Expr::Str { value, line })
            }
            Some(Token {
                kind: TokenKind::Ident(name),
                line,
            }) => {
                self.advance();
                Ok(Expr::Var { name, line })
            }
            Some(Token {
                kind: TokenKind::LParen,
                ..
            }) => {
                self.advance();
                let expr = self.parse_expr()?;
                match self.current() {
                    Some(Token {
                        kind: TokenKind::RParen,
                        ..
                    }) => {
                        self.advance();
                        Ok(expr)
                    }
                    Some(other) => Err(PalladError::UnexpectedToken {
                        got: format!("{:?}", other.kind),
                        expected: "')'".to_string(),
                        line: other.line,
                    }),
                    None => Err(PalladError::EndOfInput {
                        expected: "')'".to_string(),
                        line: self.current_line(),
                    }),
                }
            }
            Some(tok) => Err(PalladError::UnexpectedToken {
                got: format!("{:?}", tok.kind),
                expected: "value, variable, or '('".to_string(),
                line: tok.line,
            }),
            None => Err(PalladError::EndOfInput {
                expected: "value, variable, or '('".to_string(),
                line: self.current_line(),
            }),
        }
    }
}