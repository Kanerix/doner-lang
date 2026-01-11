//! Error module.

use doner_lexer::TokenKind;

/// An alias for `Result` with the error type set to [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// All the different errors this [`crate`] might produce.
#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("parser error: {0}")]
    ParseError(String),
    #[error("unexpected token error {0:?}")]
    UnexpectedTokenError(TokenKind),
    #[error("got unexpected EoF!")]
    UnexpectedEoF,
}
