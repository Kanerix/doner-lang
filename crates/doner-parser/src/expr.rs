pub use doner_lexer::TokenKind;

use crate::{
    BinaryOp, Parser,
    ast::Expr,
    error::{Error, Result},
};

pub fn parse_expr(parser: &mut Parser, min_bp: u8) -> Result<Expr> {
    let mut lhs = parse_atom(parser)?;

    while let Some(token) = parser.peek() {
        let bp = match BindingPower::try_from(token) {
            Ok(bp) => bp,
            Err(_) => break,
        };

        if bp.left < min_bp {
            break;
        }

        let op = parser.next().unwrap();
        let rhs = parse_expr(parser, bp.right)?;

        lhs = Expr::Binary {
            op: BinaryOp::try_from(op)?,
            left: Box::new(lhs),
            right: Box::new(rhs),
        };

        if bp.non_assoc {
            break;
        }
    }

    Ok(lhs)
}

fn parse_atom(parser: &mut Parser) -> Result<Expr> {
    match parser.next() {
        Some(TokenKind::Int(n)) => Ok(Expr::Int(n)),
        Some(TokenKind::LParen) => {
            let expr = parse_expr(parser, 0)?;
            match parser.next() {
                Some(TokenKind::RParen) => Ok(expr),
                Some(t) => Err(Error::UnexpectedToken(t)),
                None => Err(Error::UnexpectedEoF),
            }
        }
        Some(TokenKind::Minus) => {
            let expr = parse_expr(parser, 0)?;
            Ok(Expr::UnaryNeg(Box::new(expr)))
        }
        Some(t) => Err(Error::UnexpectedToken(t)),
        None => Err(Error::UnexpectedEoF),
    }
}

#[derive(Debug)]
struct BindingPower {
    left: u8,
    right: u8,
    non_assoc: bool,
}

impl BindingPower {
    fn new(left: u8, right: u8, non_assoc: bool) -> Self {
        Self {
            left,
            right,
            non_assoc,
        }
    }
}

impl TryFrom<&TokenKind> for BindingPower {
    type Error = Error;

    fn try_from(op: &TokenKind) -> Result<Self> {
        match op {
            TokenKind::Plus | TokenKind::Minus => Ok(BindingPower::new(1, 2, false)),
            TokenKind::Star | TokenKind::Slash => Ok(BindingPower::new(3, 4, false)),
            TokenKind::Equals => Ok(BindingPower::new(2, 1, true)),
            _ => Err(Error::UnexpectedToken(*op)),
        }
    }
}
