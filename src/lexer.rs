use std::{iter, str::Chars};

use phf::phf_map;
use thiserror::Error;

use crate::{ast::TokenKind, span::Span};

#[derive(Debug)]
#[non_exhaustive]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

pub struct Lexer<'src> {
    src: &'src str,
    chars: Chars<'src>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("string not terminated")]
    UnterminatedString { pos: usize },
    #[error("unexpected character")]
    UnexpectedCharacter,
}

impl<'src> Lexer<'src> {
    #[must_use]
    pub fn new(source: &'src str) -> Self {
        Self {
            src: source,
            chars: source.chars(),
        }
    }

    #[inline]
    fn advance(&mut self) -> Option<char> {
        self.chars.next()
    }

    #[inline]
    fn first(&self) -> Option<char> {
        self.chars.clone().next()
    }

    #[inline]
    fn second(&self) -> Option<char> {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next()
    }

    #[inline]
    fn pos(&self) -> usize {
        self.src.len() - self.chars.as_str().len()
    }

    #[allow(clippy::too_many_lines)]
    fn advance_token(&mut self) -> Result<Token, Error> {
        loop {
            let start = self.pos();
            let Some(first_char) = self.advance() else {
                return Ok(Token {
                    kind: TokenKind::Eof,
                    span: Span::new(0, 0),
                });
            };
            let token_kind = match first_char {
                c if c.is_whitespace() => continue,
                '(' => TokenKind::LeftParen,
                ')' => TokenKind::RightParen,
                '{' => TokenKind::LeftBrace,
                '}' => TokenKind::RightBrace,
                '[' => TokenKind::LeftBracket,
                ']' => TokenKind::RightBracket,
                ',' => TokenKind::Comma,
                '.' => TokenKind::Dot,
                '-' => TokenKind::Minus,
                '+' => TokenKind::Plus,
                ';' => TokenKind::Semicolon,
                '*' => TokenKind::Star,
                '!' => match self.first() {
                    Some('=') => {
                        self.advance();
                        TokenKind::BangEqual
                    }
                    _ => TokenKind::Bang,
                },
                '=' => match self.first() {
                    Some('=') => {
                        self.advance();
                        TokenKind::EqualEqual
                    }
                    _ => TokenKind::Equal,
                },
                '<' => match self.first() {
                    Some('=') => {
                        self.advance();
                        TokenKind::LessEqual
                    }
                    _ => TokenKind::Less,
                },
                '>' => match self.first() {
                    Some('=') => {
                        self.advance();
                        TokenKind::GreaterEqual
                    }
                    _ => TokenKind::Greater,
                },
                '/' => match self.first() {
                    Some('/') => {
                        while let Some(c) = self.advance() {
                            if c == '\n' {
                                break;
                            }
                        }
                        continue;
                    }
                    _ => TokenKind::Slash,
                },
                '"' => {
                    let mut terminated = false;
                    while let Some(c) = self.advance() {
                        if c == '"' {
                            terminated = true;
                            break;
                        }
                    }
                    if !terminated {
                        return Err(Error::UnterminatedString { pos: start });
                    }
                    TokenKind::String {
                        string: self.src[start + 1..self.pos() - 1].to_string(),
                    }
                }
                c if c.is_ascii_digit() => {
                    let mut has_dot = false;
                    loop {
                        match self.first() {
                            Some('.') => {
                                if has_dot {
                                    break;
                                }
                                has_dot = true;

                                match self.second() {
                                    Some(c) if c.is_ascii_digit() => {}
                                    _ => break,
                                }
                            }
                            Some(c) if c.is_ascii_digit() => {}
                            _ => break,
                        };
                        self.advance();
                    }
                    let num = self.src[start..self.pos()].parse().unwrap();
                    TokenKind::Number(num)
                }
                c if c.is_ascii_alphabetic() || c == '_' => {
                    while let Some(c) = self.first() {
                        if !c.is_ascii_alphanumeric() && c != '_' {
                            break;
                        }
                        self.advance();
                    }

                    let ident =
                        self.src[start..self.src.len() - self.chars.as_str().len()].to_string();
                    KEYWORDS
                        .get(&ident[..])
                        .cloned()
                        .unwrap_or(TokenKind::Identifier { ident })
                }
                _ => return Err(Error::UnexpectedCharacter),
            };
            let end = self.src.len() - self.chars.as_str().len();
            return Ok(Token {
                kind: token_kind,
                span: Span::new(start, end),
            });
        }
    }
}

pub fn tokenize(input: &str) -> impl Iterator<Item = Result<Token, Error>> + '_ {
    let mut lexer = Lexer::new(input);
    iter::from_fn(move || {
        let token = lexer.advance_token();
        if let Ok(Token {
            kind: TokenKind::Eof,
            ..
        }) = token
        {
            None
        } else {
            Some(token)
        }
    })
}

static KEYWORDS: phf::Map<&'static str, TokenKind> = phf_map! {
    "and" => TokenKind::And,
    "class" => TokenKind::Class,
    "else" => TokenKind::Else,
    "false" => TokenKind::False,
    "for" => TokenKind::For,
    "fun" => TokenKind::Fun,
    "if" => TokenKind::If,
    "nil" => TokenKind::Nil,
    "or" => TokenKind::Or,
    "print" => TokenKind::Print,
    "return" => TokenKind::Return,
    "super" => TokenKind::Super,
    "this" => TokenKind::This,
    "true" => TokenKind::True,
    "var" => TokenKind::Var,
    "while" => TokenKind::While,
};
