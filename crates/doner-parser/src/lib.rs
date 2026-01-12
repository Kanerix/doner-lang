pub mod ast;
pub mod error;
pub mod expr;
pub mod stmt;

pub use crate::{ast::*, error::*, stmt::*};

use doner_lexer::{TokenKind, TokenStream};

/// Parse a full program from a `TokenStream` produced by `doner-lexer`.
///
/// This function is responsible for:
/// - Driving the parser over the `TokenStream`
/// - Repeatedly delegating to `parse_stmt` for each top‑level statement
/// - Stopping when EOF is reached
///
/// The actual statement/expression parsing logic lives in `stmt`/`expr`
/// modules. This file only orchestrates the high‑level flow.
pub fn parse(tokens: TokenStream) -> Result<Program> {
    let mut parser = Parser::new(tokens);
    let mut statements: Vec<Stmt> = Vec::new();

    while !parser.is_eof() {
        let stmt = parse_stmt(&mut parser)?;
        statements.push(stmt);
    }

    Ok(Program { statements })
}

/// Lightweight parser over a `TokenStream`.
pub struct Parser {
    tokens: TokenStream,
    pos: usize,
}

impl Parser {
    /// Create a new parser over the given token stream.
    pub fn new(tokens: TokenStream) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Advance the parser by one token and return the token that was consumed.
    pub fn next(&mut self) -> Option<TokenKind> {
        let tok = self.tokens.get(self.pos).cloned();
        if tok.is_some() {
            self.pos += 1;
        }
        tok
    }

    /// Look at the current token without consuming it.
    pub fn peek(&self) -> Option<&TokenKind> {
        self.tokens.get(self.pos)
    }

    /// Look at the next token (lookahead by one) without consuming it.
    pub fn peek_next(&self) -> Option<&TokenKind> {
        self.tokens.get(self.pos + 1)
    }

    /// Returns `true` if the current token is EOF or we're past the end.
    pub fn is_eof(&self) -> bool {
        matches!(self.peek(), Some(TokenKind::EOF) | None)
    }

    /// Current position index in the token stream.
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Expect the next token in the stream to equal `token`.
    ///
    /// This consumes the next token from the stream.
    pub fn expect(&mut self, token: TokenKind) -> Result<()> {
        match self.next() {
            Some(t) if t == token => Ok(()),
            Some(t) => Err(Error::ParseError(format!(
                "expected token {token:?} got {t:?}",
            ))),
            None => Err(Error::ParseError(format!(
                "expected token {token:?} got nothing",
            ))),
        }
    }
}
