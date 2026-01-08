//! Error module.

/// An alias for `Result` with the error type set to [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// All the different errors this [`crate`] might produce.
#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown token error '{0}'!")]
	UnknownToken(char),
}
