use crate::ast::{Expr, Stmt, BinOp};
use crate::lexer::Token;
use crate::error::PalladError;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    line: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0, line: 1 }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        if let Some(Token::EOL) = self.current() {
            self.line += 1;
        }
        self.pos += 1;
    }

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
                    loop {
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

                    stmts.push(Stmt::Expr(Expr::Call { name: "print".to_string(), args }));
                }

                Token::EOL => { self.advance(); }

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

    pub fn parse_expr(&mut self) -> Result<Expr, PalladError> {
        self.parse_add_sub()
    }

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

    fn parse_factor(&mut self) -> Result<Expr, PalladError> {
        match self.current().cloned() {
            Some(Token::Int(n)) => { self.advance(); Ok(Expr::Int(n)) }
            Some(Token::Float(f)) => { self.advance(); Ok(Expr::Float(f)) }
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
