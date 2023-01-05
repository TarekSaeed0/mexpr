use std::{fmt::Display, str::Chars};

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    pub lexeme: &'a str,
    pub kind: TokenKind,
}
impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)?;
        if let TokenKind::Unknown | TokenKind::Number | TokenKind::Identifier = self.kind {
            write!(f, "({})", self.lexeme)?;
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Unknown,
    EndOfFile,
    Number,
    Identifier,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Comma,
    LeftParenthesis,
    RightParenthesis,
}
impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Tokens<'a> {
    iter: Chars<'a>,
}
impl<'a> From<&'a str> for Tokens<'a> {
    fn from(string: &'a str) -> Self {
        Self {
            iter: string.chars(),
        }
    }
}
impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let string = self.iter.as_str();
        match self.iter.next() {
            Some(ch) => match ch {
                ch if ch.is_whitespace() => {
                    while self.iter.clone().next().map_or(false, |ch| ch.is_whitespace()) {
                        self.iter.next();
                    }
                    self.next()
                },
                ch if ch.is_digit(10) => {
                    while self.iter.clone().next().map_or(false, |ch| ch.is_digit(10)) {
                        self.iter.next();
                    }
                    if let Some('.') = self.iter.clone().next() {
                        self.iter.next();
                        while self.iter.clone().next().map_or(false, |ch| ch.is_digit(10)) {
                            self.iter.next();
                        }
                    }
                    let mut iter = self.iter.clone();
                    if let Some('e') | Some('E') = iter.next() {
                        if let Some('+') | Some('-') = iter.clone().next() {
                            iter.next();
                        }
                        if iter.next().map_or(false, |ch| ch.is_digit(10)) {
                            self.iter = iter;
                            while self.iter.clone().next().map_or(false, |ch| ch.is_digit(10)) {
                                self.iter.next();
                            }
                        }
                    }
                    Some(Token {
                        lexeme: &string[..string.len() - self.iter.as_str().len()],
                        kind: TokenKind::Number,
                    })
                },
                '.' if self.iter.clone().next().map_or(false, |ch| ch.is_digit(10)) => {
                    self.iter.next();
                    while self.iter.clone().next().map_or(false, |ch| ch.is_digit(10)) {
                        self.iter.next();
                    }
                    let mut iter = self.iter.clone();
                    if let Some('e') | Some('E') = iter.next() {
                        if let Some('+') | Some('-') = iter.clone().next() {
                            iter.next();
                        }
                        if iter.next().map_or(false, |ch| ch.is_digit(10)) {
                            self.iter = iter;
                            while self.iter.clone().next().map_or(false, |ch| ch.is_digit(10)) {
                                self.iter.next();
                            }
                        }
                    }
                    Some(Token {
                        lexeme: &string[..string.len() - self.iter.as_str().len()],
                        kind: TokenKind::Number,
                    })
                },
                ch if ch.is_alphabetic() || ch == '_' => {
                    while self.iter.clone().next().map_or(false, |ch| ch.is_alphanumeric() || ch == '_') {
                        self.iter.next();
                    }
                    Some(Token {
                        lexeme: &string[..string.len() - self.iter.as_str().len()],
                        kind: TokenKind::Identifier,
                    })
                },
                '+' => Some(Token {
                    lexeme: &string[..1],
                    kind: TokenKind::Plus,
                }),
                '-' => Some(Token {
                    lexeme: &string[..1],
                    kind: TokenKind::Minus,
                }),
                '*' => Some(Token {
                    lexeme: &string[..1],
                    kind: TokenKind::Star,
                }),
                '/' => Some(Token {
                    lexeme: &string[..1],
                    kind: TokenKind::Slash,
                }),
                '%' => Some(Token {
                    lexeme: &string[..1],
                    kind: TokenKind::Percent,
                }),
                '^' => Some(Token {
                    lexeme: &string[..1],
                    kind: TokenKind::Caret,
                }),
                ',' => Some(Token {
                    lexeme: &string[..1],
                    kind: TokenKind::Comma,
                }),
                '(' => Some(Token {
                    lexeme: &string[..1],
                    kind: TokenKind::LeftParenthesis,
                }),
                ')' => Some(Token {
                    lexeme: &string[..1],
                    kind: TokenKind::RightParenthesis,
                }),
                _ => {
                    while self.iter.clone().next().map_or(false, |ch| {
                        !(
                            ch.is_whitespace()
                            || ch.is_digit(10)
                            || ch.is_alphabetic()
                            || (ch == '.' && self.iter.clone().nth(1).map_or(false, |ch| ch.is_digit(10)))
                            || matches!(ch, '+' | '-' | '*' | '/' | '%' | '^' | ',' | '(' | ')')
                        )
                    }) {
                        self.iter.next();
                    }
                    Some(Token {
                        lexeme: &string[..string.len() - self.iter.as_str().len()],
                        kind: TokenKind::Unknown,
                    })
                }
            },
            None => Some(Token {
                lexeme: string,
                kind: TokenKind::EndOfFile,
            }),
        }
    }
}