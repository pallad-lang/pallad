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
/// Processes input, strips `#` comments outside of string literals, and emits tokens for
/// identifiers, reserved keywords, integer and floating numeric literals, string literals
/// (supports `\n`, `\t`, `\r`, `\"`, `\\` and multiline strings with `"""`), operators
/// (`+`, `-`, `*`, `**`, `/`, `//`, `%`, `=`), parentheses, commas, and an end-of-line
/// `Eol` token after each non-empty line that is not inside parentheses.
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
    let mut line_no: usize = 1;
    let mut line_has_tokens = false;

    let mut chars = input.chars().peekable();
    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\r' => {
                chars.next();
            }
            '\n' => {
                chars.next();
                if paren_depth == 0 && line_has_tokens {
                    tokens.push(Token::Eol);
                }
                line_has_tokens = false;
                line_no += 1;
            }
            '#' => {
                chars.next();
                for c in &mut chars {
                    if c == '\n' {
                        if paren_depth == 0 && line_has_tokens {
                            tokens.push(Token::Eol);
                        }
                        line_has_tokens = false;
                        line_no += 1;
                        break;
                    }
                }
            }
            '0'..='9' => {
                let mut number_string = String::new();
                let mut is_float = false;
                let mut dot_count = 0;
                while let Some(&c) = chars.peek() {
                    if c.is_numeric() {
                        number_string.push(c);
                        chars.next();
                    } else if c == '.' {
                        dot_count += 1;
                        if dot_count > 1 {
                            return Err(PalladError::InvalidNumber {
                                value: number_string + ".",
                                line: line_no,
                            });
                        }
                        is_float = true;
                        number_string.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if is_float {
                    tokens.push(Token::Float(number_string.parse().map_err(|_| {
                        PalladError::InvalidNumber { value: number_string.clone(), line: line_no }
                    })?));
                } else {
                    tokens.push(Token::Int(number_string.parse().map_err(|_| {
                        PalladError::InvalidNumber { value: number_string.clone(), line: line_no }
                    })?));
                }
                line_has_tokens = true;
            }
            '_' | 'a'..='z' | 'A'..='Z' => {
                let mut identifier = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        identifier.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match identifier.as_str() {
                    "var" => tokens.push(Token::Var),
                    "none" => tokens.push(Token::None),
                    "print" => tokens.push(Token::Print),
                    "true" => tokens.push(Token::Bool(true)),
                    "false" => tokens.push(Token::Bool(false)),
                    "and" => tokens.push(Token::And),
                    "or" => tokens.push(Token::Or),
                    "not" => tokens.push(Token::Not),
                    _ => tokens.push(Token::Ident(identifier)),
                }
                line_has_tokens = true;
            }
            '"' => {
                chars.next(); // consume opening "
                let mut is_multiline = false;
                let mut is_empty_single_line = false;
                if let Some('"') = chars.peek() {
                    chars.next(); // consume second "
                    if let Some('"') = chars.peek() { // triple quote
                        chars.next();
                        is_multiline = true;
                    } else { // just two quote: empty single-line string
                        is_empty_single_line = true;
                    }
                }

                let mut string_content = String::new();
                let mut closed = false;

                if is_empty_single_line {
                    closed = true;
                } else {
                    while let Some(c) = chars.next() {
                        match c {
                            '\\' => {
                                let escaped_char = match chars.next() {
                                    Some('n') => '\n',
                                    Some('t') => '\t',
                                    Some('r') => '\r',
                                    Some('"') => '"',
                                    Some('\\') => '\\',
                                    Some(other) => {
                                        return Err(PalladError::InvalidEscape {
                                            char: other,
                                            line: line_no,
                                        });
                                    }
                                    None => {
                                        return Err(PalladError::UnterminatedString { line: line_no });
                                    }
                                };
                                string_content.push(escaped_char);
                            }
                            '\n' if !is_multiline => {
                                return Err(PalladError::UnterminatedString { line: line_no });
                            }
                            '\n' => {
                                string_content.push('\n');
                                line_no += 1;
                            }
                            '"' if is_multiline => {
                                if let Some('"') = chars.peek() {
                                    chars.next(); // consume second "
                                    if let Some('"') = chars.peek() { // triple quote
                                        chars.next();
                                        closed = true;
                                        break;
                                    }
                                    // not a triple quote: push second " (as first)
                                    string_content.push('"');
                                }
                                // one or two quotes (not three): push current "
                                string_content.push('"');
                            }
                            '"' => {
                                closed = true;
                                break;
                            }
                            other => string_content.push(other),
                        }
                    }
                }

                if !closed {
                    return Err(PalladError::UnterminatedString { line: line_no });
                }

                tokens.push(Token::Str(string_content));
                line_has_tokens = true;
            }
            '/' => {
                chars.next();
                if let Some(&'/') = chars.peek() {
                    chars.next();
                    tokens.push(Token::IntDiv);
                } else {
                    tokens.push(Token::Slash);
                }
                line_has_tokens = true;
            }
            '*' => {
                chars.next();
                if let Some(&'*') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Pow);
                } else {
                    tokens.push(Token::Star);
                }
                line_has_tokens = true;
            }
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
                line_has_tokens = true;
            }
            '-' => {
                chars.next();
                tokens.push(Token::Minus);
                line_has_tokens = true;
            }
            '%' => {
                chars.next();
                tokens.push(Token::Mod);
                line_has_tokens = true;
            }
            '=' => {
                chars.next();
                tokens.push(Token::Eq);
                line_has_tokens = true;
            }
            '(' => {
                chars.next();
                paren_depth = paren_depth.saturating_add(1);
                tokens.push(Token::LParen);
                line_has_tokens = true;
            }
            ')' => {
                chars.next();
                paren_depth = paren_depth.saturating_sub(1);
                tokens.push(Token::RParen);
                line_has_tokens = true;
            }
            ',' => {
                chars.next();
                tokens.push(Token::Comma);
                line_has_tokens = true;
            }
            _ => {
                return Err(PalladError::UnknownCharacter {
                    got: ch.to_string(),
                    line: line_no,
                });
            }
        }
    }

    if paren_depth == 0 && line_has_tokens {
        tokens.push(Token::Eol);
    }

    Ok(tokens)
}
