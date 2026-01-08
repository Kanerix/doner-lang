use doner_lexer::TokenKind;

use crate::{ast::Stmt, error, expr::parse_expr};

pub fn parse_stmt(token: &TokenKind) -> error::Result<Stmt> {
    match token {
        TokenKind::Print => todo!(),
        TokenKind::Plus => todo!(),
        TokenKind::Minus => todo!(),
        t => Ok(Stmt::Expr(parse_expr(t)?)),
    }
}
