pub use doner_lexer::TokenKind;

use crate::{
    Parser,
    ast::{BinaryOp, Expr},
    error::{Error, Result},
};

pub fn parse_expr(parser: &mut Parser) -> Result<Expr> {
    let mut prev: Option<Expr> = None;

    while let Some(token) = parser.peek() {
        if *token == TokenKind::RParen {
            break;
        }

        match parser.peek() {
            Some(TokenKind::Int(_)) => {
                let i = match parser.next() {
                    Some(TokenKind::Int(c)) => c,
                    _ => unreachable!(),
                };
                prev = Some(Expr::Int(i))
            }
            Some(TokenKind::Plus) | Some(TokenKind::Minus) => {
                let left = match prev {
                    Some(p) => Box::new(p),
                    None => return Err(Error::UnexpectedEoF),
                };
                let op = match parser.next() {
                    Some(TokenKind::Plus) => BinaryOp::Add,
                    Some(TokenKind::Minus) => BinaryOp::Sub,
                    Some(t) => return Err(Error::UnexpectedTokenError(t)),
                    None => return Err(Error::UnexpectedEoF),
                };
                let right = match parser.next() {
                    Some(TokenKind::Int(i)) => Box::new(Expr::Int(i)),
                    Some(t) => return Err(Error::UnexpectedTokenError(t)),
                    None => return Err(Error::UnexpectedEoF),
                };

                prev = Some(Expr::Binary { op, left, right });
            }
            Some(_) => {
                // SAFETY: We just peeked at the token, so we know it is not none.
                let t = parser.next().unwrap();
                return Err(Error::UnexpectedTokenError(t));
            }
            _ => return Err(Error::UnexpectedEoF),
        };
    }

    prev.ok_or(Error::UnexpectedEoF)
}
