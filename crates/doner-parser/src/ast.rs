use doner_lexer::TokenKind;

use crate::error::{Error, Result};

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Expr),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(f64),
    UnaryNeg(Box<Expr>),
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Div,
    Mul,
}

impl TryFrom<TokenKind> for BinaryOp {
    type Error = Error;

    fn try_from(op: TokenKind) -> Result<Self> {
        match op {
            TokenKind::Plus => Ok(BinaryOp::Add),
            TokenKind::Minus => Ok(BinaryOp::Sub),
            TokenKind::Slash => Ok(BinaryOp::Div),
            TokenKind::Star => Ok(BinaryOp::Mul),
            _ => Err(Error::UnexpectedToken(op)),
        }
    }
}
