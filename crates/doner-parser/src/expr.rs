pub use doner_lexer::TokenKind;

use crate::{
    Parser,
    ast::{BinaryOp, Expr},
    error::{Error, Result},
};

pub fn parse_expr(parser: &mut Parser) -> Result<Expr> {
    match parser.peek_next() {
        Some(TokenKind::Plus) | Some(TokenKind::Minus) => {
            let left = match parser.next() {
                Some(TokenKind::Int(i)) => Box::new(Expr::Int(i)),
                Some(t) => return Err(Error::UnexpectedTokenError(t)),
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

            Ok(Expr::Binary { op, left, right })
        }
        Some(_) => {
            // SAFETY: We just peeked at the token, so we know it is not none.
            let t = parser.next().unwrap();
            Err(Error::UnexpectedTokenError(t))
        }
        _ => Err(Error::UnexpectedEoF),
    }
}
