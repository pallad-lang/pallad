use crate::error::PalladError;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Var,          // 'var'
    None,         // 'none'
    Print,        // 'print'
    Ident(String),// variable names
    Bool(bool),   // 'true' or 'false'
    Int(i64),     // int numbers
    Float(f64),   // float numbers
    Str(String),  // strings
    Plus,         // '+'
    Minus,        // '-'
    Star,         // '*'
    Slash,        // '/'
    IntDiv,       // '//'
    Mod,          // '%'
    Pow,          // '**'
    Eq,           // '='
    LParen,       // '('
    RParen,       // ')'
    Comma,        // ','
    And,          // 'and'
    Or,           // 'or'
    Not,          // 'not'
    Eol,          // end of line
}

/// Converts source text into a sequence of lexical tokens for the language.
///
/// Processes input line-by-line, strips `#` comments, and emits tokens for identifiers,
/// reserved keywords, integer and floating numeric literals, string literals (supports
/// `\n`, `\t`, `\r`, `\"`, `\\`), operators (`+`, `-`, `*`, `**`, `/`, `//`, `%`, `=`),
/// parentheses, commas, and an end-of-line `Eol` token after each non-empty line
/// that is not inside parentheses.
///
/// # Returns
///
/// `Ok(Vec<Token>)` containing the token stream on success, or `Err(PalladError)` with the
/// source line number for the first lexical error encountered (for example `InvalidNumber`,
/// `InvalidEscape`, `UnterminatedString`, or `UnknownCharacter`).
///
/// # Examples
///
/// ```
/// let src = r#"
/// var x = 42
/// print x
/// "#;
/// let tokens = tokenize(src).unwrap();
/// assert!(matches!(tokens.get(0), Some(Token::Var)));
/// assert!(matches!(tokens.get(3), Some(Token::Int(42))));
/// ```
pub fn tokenize(input: &str) -> Result<Vec<Token>, PalladError> {
    let mut tokens = Vec::new();
    let mut paren_depth: usize = 0;

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
                '_' | 'a'..='z' | 'A'..='Z' => {
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
                        "none" => tokens.push(Token::None),
                        "print" => tokens.push(Token::Print),
                        "true" => tokens.push(Token::Bool(true)),
                        "false" => tokens.push(Token::Bool(false)),
                        "and" => tokens.push(Token::And),
                        "or" => tokens.push(Token::Or),
                        "not" => tokens.push(Token::Not),
                        _ => tokens.push(Token::Ident(ident)),
                    }
                }
                '"' => {
                    chars.next(); // consume opening "

                    let mut s = String::new();
                    let mut closed = false;

                    while let Some(c) = chars.next() {
                        match c {
                            '\\' => {
                                let escaped = match chars.next() {
                                    Some('n') => '\n',
                                    Some('t') => '\t',
                                    Some('r') => '\r',
                                    Some('"') => '"',
                                    Some('\\') => '\\',
                                    Some(other) => {
                                        return Err(PalladError::InvalidEscape {
                                            char: other,
                                            line: line_no + 1,
                                        });
                                    }
                                    None => {
                                        return Err(PalladError::UnterminatedString { line: line_no + 1 });
                                    }
                                };
                                s.push(escaped);
                            }
                            '"' => {
                                closed = true;
                                break;
                            }
                            other => s.push(other),
                        }
                    }

                    if !closed {
                        return Err(PalladError::UnterminatedString { line: line_no + 1 });
                    }

                    tokens.push(Token::Str(s));
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
                '*' => {
                    chars.next();
                    if let Some(&'*') = chars.peek() {
                        chars.next();
                        tokens.push(Token::Pow);
                    } else {
                        tokens.push(Token::Star);
                    }
                }
                '+' => { chars.next(); tokens.push(Token::Plus); }
                '-' => { chars.next(); tokens.push(Token::Minus); }
                '%' => { chars.next(); tokens.push(Token::Mod); }
                '=' => { chars.next(); tokens.push(Token::Eq); }
                '(' => {
                    chars.next();
                    paren_depth = paren_depth.saturating_add(1);
                    tokens.push(Token::LParen);
                }
                ')' => {
                    chars.next();
                    paren_depth = paren_depth.saturating_sub(1);
                    tokens.push(Token::RParen);
                }
                ',' => { chars.next(); tokens.push(Token::Comma); }
                _ => {
                    return Err(PalladError::UnknownCharacter {
                        got: ch.to_string(),
                        line: line_no + 1,
                    });
                },
            }
        }
        if paren_depth == 0 {
            tokens.push(Token::Eol);
        }
    } 

    Ok(tokens)
}
