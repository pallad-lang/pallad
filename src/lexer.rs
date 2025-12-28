use crate::error::PalladError;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Var,          // 'var'
    Print,        // 'print'
    Ident(String),// variable names
    Int(i64),     // int numbers
    Float(f64),   // float numbers
    Plus,         // '+'
    Minus,        // '-'
    Star,         // '*'
    Slash,        // '/'
    IntDiv,       // '//'
    Mod,          // '%'
    Eq,           // '='
    LParen,       // '('
    RParen,       // ')'
    Comma,        // ',' 
    Eol,          // end of line
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, PalladError> {
    let mut tokens = Vec::new();

    for (line_no, line) in input.lines().enumerate() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() { continue; }

        let mut chars = line.chars().peekable();

        while let Some(&ch) = chars.peek() {
            match ch {
                ' ' | '\t' => { chars.next(); }
                '0'..='9' => {
                    let mut num = String::new();
                    let mut is_float = false;
                    let mut dot_count = 0;
                    while let Some(&c) = chars.peek() {
                        if c.is_numeric() {
                            num.push(c);
                            chars.next();
                        } else if c == '.' {
                            dot_count += 1;
                            if dot_count > 1 {
                                return Err(PalladError::InvalidNumber {
                                    value: num + ".",
                                    line: line_no + 1,
                                });
                            }
                            is_float = true;
                            num.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    if is_float {
                        tokens.push(Token::Float(num.parse().map_err(|_| {
                            PalladError::InvalidNumber { value: num.clone(), line: line_no + 1 }
                        })?));
                    } else {
                        tokens.push(Token::Int(num.parse().map_err(|_| {
                            PalladError::InvalidNumber { value: num.clone(), line: line_no + 1 }
                        })?));
                    }
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut ident = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() || c == '_' {
                            ident.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    match ident.as_str() {
                        "var" => tokens.push(Token::Var),
                        "print" => tokens.push(Token::Print),
                        _ => tokens.push(Token::Ident(ident)),
                    }
                }
                '/' => {
                    chars.next();
                    if let Some(&'/') = chars.peek() {
                        chars.next();
                        tokens.push(Token::IntDiv);
                    } else {
                        tokens.push(Token::Slash);
                    }
                }
                '+' => { chars.next(); tokens.push(Token::Plus); }
                '-' => { chars.next(); tokens.push(Token::Minus); }
                '*' => { chars.next(); tokens.push(Token::Star); }
                '%' => { chars.next(); tokens.push(Token::Mod); }
                '=' => { chars.next(); tokens.push(Token::Eq); }
                '(' => { chars.next(); tokens.push(Token::LParen); }
                ')' => { chars.next(); tokens.push(Token::RParen); }
                ',' => { chars.next(); tokens.push(Token::Comma); }
                _ => {
                    return Err(PalladError::UnknownCharacter {
                        got: ch.to_string(),
                        line: line_no + 1,
                    });
                },
            }
        }
        tokens.push(Token::Eol);
    } 

    Ok(tokens)
}
