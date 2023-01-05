use crate::token::*;
use std::{error::Error, fmt::Display, iter::Peekable};

/* 
 * mexpr grammar
 * --------------------------------------------------------
 * atom = (number | identifier | "+" | "-" | "*" | "/" | "%" | "^" | "(" expr ")") ("(" expr? ("," expr)* ")")*
 * primary = atom ("^" factor)?
 * factor = (("+" | "-") factor) | primary
 * term = (factor (("*" | "/" | "%") factor)*)
 * expr = (term (("+" | "-") term)*)
 * --------------------------------------------------------
 */

#[derive(Debug, Clone)]
pub struct ParseError<'a> {
    pub expected: &'static [TokenKind],
    pub found: Token<'a>,
}
impl<'a> Display for ParseError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.expected.len() > 0 {
            let mut iter = self.expected.iter();
            write!(f, "expected a token of type ")?;
            write!(f, "{}", iter.next().unwrap())?;
            for token_kind in iter {
                write!(f, " or {}", token_kind)?;
            }
            write!(f, " but ")?;
        }
        write!(f, "found {}", self.found)
    }
}
impl<'a> Error for ParseError<'a> {}

#[derive(Debug, Clone)]
pub enum ParseTree {
    Number(f64),
    Identifier(String),
    FunctionCall(Box<ParseTree>, Vec<ParseTree>),
}
impl<'a> TryFrom<&'a str> for ParseTree {
    type Error = ParseError<'a>;

    fn try_from(string: &'a str) -> Result<Self, Self::Error> {
        Parser::from(string).parse()
    }
}

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    iter: Peekable<Tokens<'a>>,
}
impl<'a> Parser<'a> {
    fn atom(&mut self) -> Result<ParseTree, ParseError<'a>> {
        let token = self.iter.next().unwrap();
        let mut primary = match token.kind {
            TokenKind::Number => Ok(ParseTree::Number(token.lexeme.parse().unwrap())),
            TokenKind::Identifier => Ok(ParseTree::Identifier(token.lexeme.to_owned())),
            TokenKind::Plus => Ok(ParseTree::Identifier("+".to_owned())),
            TokenKind::Minus => Ok(ParseTree::Identifier("-".to_owned())),
            TokenKind::Star => Ok(ParseTree::Identifier("*".to_owned())),
            TokenKind::Slash => Ok(ParseTree::Identifier("/".to_owned())),
            TokenKind::Percent => Ok(ParseTree::Identifier("%".to_owned())),
            TokenKind::Caret => Ok(ParseTree::Identifier("^".to_owned())),
            TokenKind::LeftParenthesis => {
                let primary = self.expr()?;
                let token = self.iter.next().unwrap();
                if let TokenKind::RightParenthesis = token.kind {
                    Ok(primary)
                } else {
                    Err(ParseError {
                        expected: &[TokenKind::RightParenthesis],
                        found: token,
                    })
                }
            },
            _ => {
                Err(ParseError {
                    expected: &[
                        TokenKind::Number,
                        TokenKind::Identifier,
                        TokenKind::Plus,
                        TokenKind::Minus,
                        TokenKind::LeftParenthesis
                    ],
                    found: token,
                })
            }
        }?;
        while let TokenKind::LeftParenthesis = self.iter.peek().unwrap().kind {
            self.iter.next().unwrap();
            let mut args = Vec::new();
            let mut parser = self.clone();
            if let Ok(arg) = parser.expr() {
                *self = parser;
                args.push(arg);
                while let TokenKind::Comma = self.iter.peek().unwrap().kind {
                    self.iter.next().unwrap();
                    args.push(self.expr()?);
                }
            }
            let token = self.iter.next().unwrap();
            primary = if let TokenKind::RightParenthesis = token.kind {
                Ok(ParseTree::FunctionCall(Box::new(primary), args))
            } else {
                Err(ParseError {
                    expected: &[TokenKind::RightParenthesis],
                    found: token,
                })
            }?
        }
        Ok(primary)
    }
    fn primary(&mut self) -> Result<ParseTree, ParseError<'a>> {
        let mut primary = self.atom()?;
        if let TokenKind::Caret = self.iter.peek().unwrap().kind {
            self.iter.next();
            primary = ParseTree::FunctionCall(Box::new(ParseTree::Identifier("^".to_owned())), vec![primary, self.factor()?])
        }
        Ok(primary)
    }
    fn factor(&mut self) -> Result<ParseTree, ParseError<'a>> {
        if let TokenKind::Plus | TokenKind::Minus = self.iter.peek().unwrap().kind {
            let mut parser = self.clone();
            let token = parser.iter.next().unwrap();
            if let TokenKind::Star | TokenKind::Slash | TokenKind::Percent | TokenKind::Caret = parser.iter.peek().unwrap().kind {
                let mut parser = parser.clone();
                if let (Ok(_), Ok(_)) = (parser.factor(), parser.factor()) {
                    return self.primary();
                }
            }
            if let Ok(factor) = parser.factor() {
                *self = parser;
                match token.kind {
                    TokenKind::Plus => Ok(ParseTree::FunctionCall(Box::new(ParseTree::Identifier("+".to_owned())), vec![factor])),
                    _ => Ok(ParseTree::FunctionCall(Box::new(ParseTree::Identifier("-".to_owned())), vec![factor])),
                }
            } else {
                self.primary()
            }
        } else {
            self.primary()
        }
    }
    fn term(&mut self) -> Result<ParseTree, ParseError<'a>> {
        let mut term = self.factor()?;
        while let TokenKind::Star | TokenKind::Slash | TokenKind::Percent = self.iter.peek().unwrap().kind {
            match self.iter.next().unwrap().kind {
                TokenKind::Star => term = ParseTree::FunctionCall(Box::new(ParseTree::Identifier("*".to_owned())), vec![term, self.factor()?]),
                TokenKind::Slash => term = ParseTree::FunctionCall(Box::new(ParseTree::Identifier("/".to_owned())), vec![term, self.factor()?]),
                _ => term = ParseTree::FunctionCall(Box::new(ParseTree::Identifier("%".to_owned())), vec![term, self.factor()?]),
            }
        }
        Ok(term)
    }
    fn expr(&mut self) -> Result<ParseTree, ParseError<'a>> {
        let mut expr = self.term()?;
        while let TokenKind::Plus | TokenKind::Minus = self.iter.peek().unwrap().kind {
            match self.iter.next().unwrap().kind {
                TokenKind::Plus => expr = ParseTree::FunctionCall(Box::new(ParseTree::Identifier("+".to_owned())), vec![expr, self.term()?]),
                _ => expr = ParseTree::FunctionCall(Box::new(ParseTree::Identifier("-".to_owned())), vec![expr, self.term()?]),
            }
        }
        Ok(expr)
    }
    pub fn parse(&mut self) -> Result<ParseTree, ParseError<'a>> {    
        let tree = self.expr()?;
        let token = self.iter.next().unwrap();
        if let TokenKind::EndOfFile = token.kind {
            Ok(tree)
        } else {
            Err(ParseError {
                expected: &[TokenKind::EndOfFile],
                found: token,
            })
        }
    }
}
impl<'a> From<&'a str> for Parser<'a> {
    fn from(string: &'a str) -> Self {
        Self {
            iter: Tokens::from(string).peekable(),
        }
    }
}