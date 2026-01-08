pub mod ast;
pub mod error;
pub mod expr;
pub mod stmt;

pub use crate::{
    error::Error,
    ast::{Program, Stmt},
    stmt::parse_stmt,
};

use doner_lexer::TokenKind;

pub fn parse(tokens: &[TokenKind]) -> Result<Program, Error> {
    let mut tokens = tokens.iter();
    let mut stmt: Vec<Stmt> = Vec::new();
    while let Some(token) = tokens.next() {
        stmt.push(parse_stmt(&token)?);
    }
    Ok(Program { statements: stmt })
}
