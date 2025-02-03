use std::char;
use std::collections::LinkedList;
use std::fmt::Display;

type LexResult = Result<Vec<Token>, LexError>;
type TokenResult = Result<Token, LexError>;

pub fn lex(content: &str) -> LexResult {
    let mut stream = CharStream::from_string(content.to_string());

    let mut result = LinkedList::new();

    loop {
        if let Some(c) = stream.peek() {
            let token = match c {
                _ if c.is_numeric() => parse_num(&mut stream),
                _ => return Err(stream.make_error("Unexcpexted token")),
            }?;

            result.push_back(token);

            continue;
        }

        break;
    }

    Ok(result.into_iter().collect::<Vec<Token>>())
}

fn parse_num(stream: &mut CharStream) -> TokenResult {
    let start = stream.get_offset();
    let res = stream.take_while(|c| c.is_numeric()).collect::<String>();

    let range = SourceRange {
        start,
        end: stream.get_offset(),
    };

    Ok(Token::int(res, range))
}

pub(crate) struct CharStream {
    source: String,
    current: Option<char>,
    index: usize,
    line: usize,
    column: usize,
}

impl CharStream {
    pub fn from_string(string: String) -> Self {
        Self {
            source: string,
            current: None,
            index: 0,
            line: 0,
            column: 0,
        }
    }

    fn peek(&mut self) -> Option<char> {

            self.source.chars().nth(self.index)
    }

    fn make_error(&self, msg: &str) -> LexError {
        LexError {
            msg: msg.to_string(),
            line: self.line,
            column: self.column,
        }
    }

    fn get_offset(&self) -> SourceOffset {
        SourceOffset {
            offset: self.index,
            line: self.line,
            column: self.column,
        }
    }
}

impl Iterator for CharStream {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.current {
            self.current = None;
            println!("Reurn peeked char");
            dbg!(c);
            return Some(c);
        }

        let next_index = self.index + 1;
        let mut chars = self.source.chars();
        let c = chars.nth(self.index)?;
        self.index = next_index;
        if c == '\n' {
            self.line += 1;
        }

        self.column += 1;
        println!("Return next char");
        dbg!(c);
        Some(c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceRange {
    pub start: SourceOffset,
    pub end: SourceOffset,
}

impl SourceRange {
    pub fn empty() -> Self {
        Self {
            start: SourceOffset {
                offset: 0,
                line: 0,
                column: 0,
            },
            end: SourceOffset {
                offset: 0,
                line: 0,
                column: 0,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceOffset {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

impl Display for SourceOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub value: String,
    pub kind: TokenKind,
    pub range: SourceRange,
}

impl Token {
    fn new(kind: TokenKind, value: String, range: SourceRange) -> Self {
        Self { value, kind, range }
    }

    fn int(value: String, range: SourceRange) -> Self {
        Self::new(TokenKind::Int, value, range)
    }

    pub fn empty(range: SourceRange) -> Self {
        Self::new(TokenKind::Nop, "".to_string(), range)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Int,
    Nop,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct LexError {
    msg: String,
    line: usize,
    column: usize,
}

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: ")
    }
}

#[cfg(test)]
mod tests {
    use crate::lex::{lex, SourceOffset, SourceRange, Token};

    use super::{parse_num, CharStream};

    #[test]
    fn test_numeric_lex() {
        let result = lex("256").unwrap();

        let range = SourceRange {
            start: SourceOffset {
                offset: 0,
                line: 0,
                column: 0,
            },
            end: SourceOffset {
                offset: 3,
                line: 0,
                column: 3,
            },
        };
        let expected = vec![Token {
            value: "256".to_string(),
            kind: crate::lex::TokenKind::Int,
            range,
        }];
        assert_eq!(expected, result)
    }
}
