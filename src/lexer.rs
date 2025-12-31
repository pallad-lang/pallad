use crate::error::PalladError;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Var,          // 'var'
    None,         // 'none'
    Print,        // 'print'
    Ident(String),// variable names
    Int(i64),     // int numbers
    Float(f64),   // float numbers
    Str(String),  // strings
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

/// Convert source text into a sequence of lexical tokens for the language.
///
/// Processes the input line-by-line, stripping `#` comments and emitting tokens for
/// identifiers, reserved keywords, integer and floating numeric literals, string literals
/// (with escape sequences: \n, \t, \r, \", \\, \'), operators (`+`, `-`, `*`, `/`, `//`, 
/// `%`, `=`), parentheses, commas, and an end-of-line `Eol` token after each non-empty line.
///
/// # Returns
///
/// `Ok(Vec<Token>)` with the token stream on success, or `Err(PalladError)` if a lexical
/// error is encountered (for example `InvalidNumber` for malformed numeric literals or
/// `UnknownCharacter` for unexpected characters), with the error carrying the line number.
///
/// # Examples
///
/// ```
/// let src = "var x = 42\nprint x\n";
/// let tokens = tokenize(src).unwrap();
/// // starts with: Var, Ident("x"), Eq, Int(42), Eol, Print, Ident("x"), Eol
/// assert!(matches!(tokens.get(0), Some(Token::Var)));
/// assert!(matches!(tokens.get(3), Some(Token::Int(42))));
/// ```
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
                        "none" => tokens.push(Token::None),
                        "print" => tokens.push(Token::Print),
                        _ => tokens.push(Token::Ident(ident)),
                    }
                }
                '"' => {
                    chars.next(); // consume opening "
                    let s = parse_string(&mut chars, '"', line_no)?;
                    tokens.push(Token::Str(s));
                }
                '\'' => {
                    chars.next(); // consume opening '
                    let s = parse_string(&mut chars, '\'', line_no)?;
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

fn parse_string(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    quote: char,
    line_no: usize,
) -> Result<String, PalladError> {
    let mut s = String::new();
    let mut closed = false;

    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                let escaped = match chars.next() {
                    Some('n') => '\n',
                    Some('t') => '\t',
                    Some('r') => '\r',
                    Some(q) if q == quote => q,
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
            c if c == quote => {
                closed = true;
                break;
            }
            other => s.push(other),
        }
    }

    if !closed {
        return Err(PalladError::UnterminatedString { line: line_no + 1 });
    }

    Ok(s)
}