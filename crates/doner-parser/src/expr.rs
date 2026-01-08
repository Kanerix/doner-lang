pub use doner_lexer::TokenKind;

use crate::{
    ast::Expr,
    error::{Error, Result},
};

pub fn parse_expr(token: &TokenKind) -> Result<Expr> {
    match token {
        _ => Err(Error::ParseError),
    }
}
