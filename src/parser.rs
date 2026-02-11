use crate::ast::{BinOp, Expr, Stmt, UnOp};
use crate::error::PalladError;
use crate::lexer::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    current_pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_pos: 0,
        }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.current_pos)
    }

    fn current_line(&self) -> usize {
        self.current()
            .map(|t| t.line)
            .unwrap_or_else(|| self.tokens.last().map(|t| t.line).unwrap_or(1))
    }

    fn advance(&mut self) {
        self.current_pos += 1;
    }

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

    pub fn parse_expr(&mut self) -> Result<Expr, PalladError> {
        self.parse_or()
    }

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
