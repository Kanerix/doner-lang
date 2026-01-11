use doner_lexer::TokenKind;

use crate::{Parser, ast::Stmt, error::Result, expr::parse_expr};

/// Parse a single statement from the token stream.
///
/// The responsibility of this function is:
/// - Look at the current token via the `Parser`
/// - Decide which kind of statement we're dealing with
/// - Delegate to `parse_expr` for the expression part
/// - Consume the trailing semicolon
pub fn parse_stmt(cursor: &mut Parser) -> Result<Stmt> {
    match cursor.peek() {
        Some(TokenKind::Print) => parse_print_stmt(cursor),
        // Any other token starts an expression statement (or it's an error
        // that `parse_expr` will report).
        _ => parse_expr_stmt(cursor),
    }
}

/// Parse a `serve` statement:
fn parse_print_stmt(parser: &mut Parser) -> Result<Stmt> {
    // Consume the `Print` token.
    let _ = parser.next();

    parser.expect(TokenKind::LParen)?;
    let value = parse_expr(parser)?;
    parser.expect(TokenKind::RParen)?;
    parser.expect(TokenKind::SemiColon)?;

    Ok(Stmt::Print(value))
}

/// Parse an expression statement
fn parse_expr_stmt(parser: &mut Parser) -> Result<Stmt> {
    let expr = parse_expr(parser)?;
    parser.expect(TokenKind::SemiColon)?;
    Ok(Stmt::Expr(expr))
}
