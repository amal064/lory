// TODO handle span information
// TODO errors

use std::iter::Peekable;

use itertools::PeekingNext;
use thiserror::Error;

use crate::{
    ast::{BinOp, Expression, TokenKind, UnaryOp},
    lexer::{self, Token},
};

type LexerResult = Result<Token, lexer::Error>;
type ParseResult = Result<Expression, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("temporary error")]
    Temp,
    #[error(transparent)]
    LexerError(#[from] lexer::Error),
}

pub struct Parser<T>
where
    T: Iterator<Item = LexerResult>,
{
    tokens: Peekable<T>,
}

macro_rules! eat_tok {
    ($self:ident, $pattern:pat) => {
        $self
            .tokens
            .peeking_next(|t: &LexerResult| match t {
                Ok(t) => matches!(&t.kind, $pattern),
                Err(_) => false,
            })
            .transpose()?
    };
}

macro_rules! eat_and_translate {
    ($self:ident, $($pattern:pat => $res:expr),+) => {
        if let Some(t) = eat_tok!($self, $($pattern)|+)
        {
            Some(match t.kind {
                $($pattern => $res,)*
                _ => unreachable!(),
            })
        } else {
            None
        }
    };
}

impl<T> Parser<T>
where
    T: Iterator<Item = LexerResult>,
{
    pub fn new(tokens: T) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    pub fn parse_expression(&mut self) -> ParseResult {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> ParseResult {
        let mut left = self.parse_comparison()?;

        while let Some(op) = eat_and_translate!(self,
            TokenKind::EqualEqual => BinOp::Equal,TokenKind::BangEqual => BinOp::NotEqual)
        {
            let right = self.parse_comparison()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> ParseResult {
        let mut left = self.parse_term()?;
        while let Some(op) = eat_and_translate!(self,
            TokenKind::Greater => BinOp::Greater,
            TokenKind::GreaterEqual => BinOp::GreaterEqual,
            TokenKind::Less => BinOp::Less,
            TokenKind::LessEqual => BinOp::LessEqual)
        {
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(self.parse_term()?),
            };
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> ParseResult {
        let mut left = self.parse_factor()?;
        while let Some(op) =
            eat_and_translate!(self, TokenKind::Plus => BinOp::Add, TokenKind::Minus => BinOp::Sub)
        {
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(self.parse_factor()?),
            };
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> ParseResult {
        let mut left = self.parse_unary()?;
        while let Some(op) =
            eat_and_translate!(self, TokenKind::Star => BinOp::Mul, TokenKind::Slash => BinOp::Div)
        {
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(self.parse_unary()?),
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> ParseResult {
        if let Some(op) = eat_and_translate!(self, TokenKind::Bang => UnaryOp::Not, TokenKind::Minus => UnaryOp::Neg)
        {
            Ok(Expression::UnaryOp {
                op,
                expr: Box::new(self.parse_unary()?),
            })
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> ParseResult {
        #[allow(unused_variables)]
        if let Some(l) = eat_and_translate!(self, TokenKind::Lit(l) => l) {
            return Ok(Expression::Literal(l));
        } else if eat_tok!(self, TokenKind::LeftParen).is_some() {
            let expr = self.parse_equality()?;
            eat_tok!(self, TokenKind::RightParen);
            return Ok(expr);
        }
        Err(Error::Temp)
    }
}
