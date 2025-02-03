use std::{
    collections::LinkedList,
    error::Error,
    fmt::Display,
    iter::{Iterator, Peekable},
    vec::IntoIter,
};

type TokenStream = Peekable<IntoIter<Token>>;
type ExprResult = Result<Expr, ParseError>;

use crate::{
    ast::{Expr, Module, Stmt},
    lex::{SourceRange, Token, TokenKind},
};

pub fn parse_module(name: String, tokens: Vec<Token>) -> Module {
    Module {
        name,
        exprs: parse_tokens(tokens),
    }
}

fn parse_tokens(tokens: Vec<Token>) -> Vec<Expr> {
    let mut iter = tokens.into_iter().peekable();
    let mut exprs = LinkedList::new();

    while iter.peek_token().is_ok() {
        let expr = parse_expr(&mut iter).unwrap();
        exprs.push_back(expr);
    }

    exprs.into_iter().collect::<Vec<Expr>>()
}

fn parse_expr(stream: &mut TokenStream) -> ExprResult {
    let token = stream.peek_token()?;

    match token.kind {
        TokenKind::Int => parse_int(stream),
        TokenKind::Nop => Err(ParseError::new(token, "Unexpected token")),
    }
}

fn parse_int(stream: &mut TokenStream) -> ExprResult {
    let token = stream.next_token()?;

    token
        .value
        .parse::<i32>()
        .map(|n| Expr::Int(n))
        .map_err(|e| ParseError::new(token, format!("{}", e).as_str()))
}

trait TokenStreamTrait {
    fn end_range(&mut self) -> SourceRange;
    fn next_token(&mut self) -> Result<Token, ParseError>;
    fn peek_token(&mut self) -> Result<Token, ParseError>;
}

impl TokenStreamTrait for TokenStream {
    fn end_range(&mut self) -> SourceRange {
        if let Some(t) = self.peek() {
            t.range
        } else {
            SourceRange::empty()
        }
    }

    fn next_token(&mut self) -> Result<Token, ParseError> {
        if let Some(token) = self.next() {
            Ok(token)
        } else {
            Err(ParseError::new(
                Token::empty(self.end_range()),
                "Unexpected end of file",
            ))
        }
    }

    fn peek_token(&mut self) -> Result<Token, ParseError> {
        if let Some(token) = self.peek() {
            Ok(token.clone())
        } else {
            Err(ParseError::new(
                Token::empty(self.end_range()),
                "Unexpected end of file",
            ))
        }
    }
}

#[derive(Debug)]
struct ParseError {
    msg: String,
    range: SourceRange,
}

impl ParseError {
    fn new(token: Token, msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
            range: token.range,
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError: {} \n {}", self.msg, self.range.start)
    }
}

impl Error for ParseError {}

#[cfg(test)]
mod tests {
    use crate::{
        ast::Expr,
        lex::{lex, CharStream},
        parser::parse_expr,
    };

    use super::parse_tokens;

    #[test]
    fn parse_int_test() {
        let result = parse_tokens(lex("256").unwrap());

        let expected = vec![Expr::Int(256)];

        assert_eq!(expected, result)
    }
}
