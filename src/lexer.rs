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
    EOL,          // end of line
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
                    while let Some(&c) = chars.peek() {
                        if c.is_numeric() {
                            num.push(c);
                            chars.next();
                        } else if c == '.' {
                            is_float = true;
                            num.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    if is_float {
                        tokens.push(Token::Float(num.parse().unwrap()));
                    } else {
                        tokens.push(Token::Int(num.parse().unwrap()));
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
        tokens.push(Token::EOL);
    } 

    Ok(tokens)
}
