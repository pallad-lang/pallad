use crate::error::PalladError;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Var,
    None,
    Print,
    Ident(String),
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Plus,
    Minus,
    Star,
    Slash,
    IntDiv,
    Mod,
    Pow,
    Eq,
    LParen,
    RParen,
    Comma,
    And,
    Or,
    Not,
    Eol,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
}

/// Converts source text into a sequence of lexer tokens, each annotated with the line on which it appears.
///
/// The tokenizer recognizes integers, floats, identifiers, keywords (e.g., `var`, `print`, `none`, `true`, `false`, `and`, `or`, `not`),
/// string literals (single-line, empty single-line, and multiline with escapes), comments, punctuation, and operators.
/// On success returns a vector of `Token` values in lexical order. On failure returns a `PalladError` describing the first
/// encountered lexical error (invalid number, unknown escape, unterminated string, or unknown character).
///
/// # Returns
///
/// `Ok(Vec<Token>)` with the lexical tokens in order, or `Err(PalladError)` if a lexical error is encountered.
///
/// # Examples
///
/// ```
/// let src = r#"var x = 42
/// print(x)
/// "#;
/// let tokens = tokenize(src).unwrap();
/// assert!(matches!(tokens.first().unwrap().kind, TokenKind::Var));
/// assert!(matches!(tokens.last().unwrap().kind, TokenKind::Eol));
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
                    tokens.push(Token {
                        kind: TokenKind::Eol,
                        line: line_no,
                    });
                }
                line_has_tokens = false;
                line_no += 1;
            }
            '#' => {
                chars.next();
                for c in &mut chars {
                    if c == '\n' {
                        if paren_depth == 0 && line_has_tokens {
                            tokens.push(Token {
                                kind: TokenKind::Eol,
                                line: line_no,
                            });
                        }
                        line_has_tokens = false;
                        line_no += 1;
                        break;
                    }
                }
            }
            '0'..='9' => {
                let token_line = line_no;
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
                                line: token_line,
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
                    tokens.push(Token {
                        kind: TokenKind::Float(number_string.parse().map_err(|_| {
                            PalladError::InvalidNumber {
                                value: number_string.clone(),
                                line: token_line,
                            }
                        })?),
                        line: token_line,
                    });
                } else {
                    tokens.push(Token {
                        kind: TokenKind::Int(number_string.parse().map_err(|_| {
                            PalladError::InvalidNumber {
                                value: number_string.clone(),
                                line: token_line,
                            }
                        })?),
                        line: token_line,
                    });
                }
                line_has_tokens = true;
            }
            '_' | 'a'..='z' | 'A'..='Z' => {
                let token_line = line_no;
                let mut identifier = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        identifier.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let kind = match identifier.as_str() {
                    "var" => TokenKind::Var,
                    "none" => TokenKind::None,
                    "print" => TokenKind::Print,
                    "true" => TokenKind::Bool(true),
                    "false" => TokenKind::Bool(false),
                    "and" => TokenKind::And,
                    "or" => TokenKind::Or,
                    "not" => TokenKind::Not,
                    _ => TokenKind::Ident(identifier),
                };
                tokens.push(Token {
                    kind,
                    line: token_line,
                });
                line_has_tokens = true;
            }
            '"' => {
                let token_line = line_no;
                chars.next();
                let mut is_multiline = false;
                let mut is_empty_single_line = false;
                if let Some('"') = chars.peek() {
                    chars.next();
                    if let Some('"') = chars.peek() {
                        chars.next();
                        is_multiline = true;
                    } else {
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
                                        return Err(PalladError::UnterminatedString {
                                            line: token_line,
                                        });
                                    }
                                };
                                string_content.push(escaped_char);
                            }
                            '\n' if !is_multiline => {
                                return Err(PalladError::UnterminatedString { line: token_line });
                            }
                            '\n' => {
                                string_content.push('\n');
                                line_no += 1;
                            }
                            '"' if is_multiline => {
                                if let Some('"') = chars.peek() {
                                    chars.next();
                                    if let Some('"') = chars.peek() {
                                        chars.next();
                                        closed = true;
                                        break;
                                    }
                                    string_content.push('"');
                                }
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
                    return Err(PalladError::UnterminatedString { line: token_line });
                }

                tokens.push(Token {
                    kind: TokenKind::Str(string_content),
                    line: token_line,
                });
                line_has_tokens = true;
            }
            '/' => {
                let token_line = line_no;
                chars.next();
                if let Some(&'/') = chars.peek() {
                    chars.next();
                    tokens.push(Token {
                        kind: TokenKind::IntDiv,
                        line: token_line,
                    });
                } else {
                    tokens.push(Token {
                        kind: TokenKind::Slash,
                        line: token_line,
                    });
                }
                line_has_tokens = true;
            }
            '*' => {
                let token_line = line_no;
                chars.next();
                if let Some(&'*') = chars.peek() {
                    chars.next();
                    tokens.push(Token {
                        kind: TokenKind::Pow,
                        line: token_line,
                    });
                } else {
                    tokens.push(Token {
                        kind: TokenKind::Star,
                        line: token_line,
                    });
                }
                line_has_tokens = true;
            }
            '+' => {
                chars.next();
                tokens.push(Token {
                    kind: TokenKind::Plus,
                    line: line_no,
                });
                line_has_tokens = true;
            }
            '-' => {
                chars.next();
                tokens.push(Token {
                    kind: TokenKind::Minus,
                    line: line_no,
                });
                line_has_tokens = true;
            }
            '%' => {
                chars.next();
                tokens.push(Token {
                    kind: TokenKind::Mod,
                    line: line_no,
                });
                line_has_tokens = true;
            }
            '=' => {
                chars.next();
                tokens.push(Token {
                    kind: TokenKind::Eq,
                    line: line_no,
                });
                line_has_tokens = true;
            }
            '(' => {
                chars.next();
                paren_depth = paren_depth.saturating_add(1);
                tokens.push(Token {
                    kind: TokenKind::LParen,
                    line: line_no,
                });
                line_has_tokens = true;
            }
            ')' => {
                chars.next();
                paren_depth = paren_depth.saturating_sub(1);
                tokens.push(Token {
                    kind: TokenKind::RParen,
                    line: line_no,
                });
                line_has_tokens = true;
            }
            ',' => {
                chars.next();
                tokens.push(Token {
                    kind: TokenKind::Comma,
                    line: line_no,
                });
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
        tokens.push(Token {
            kind: TokenKind::Eol,
            line: line_no,
        });
    }

    Ok(tokens)
}