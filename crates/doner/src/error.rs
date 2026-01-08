use miette::Diagnostic;
use thiserror::Error;

pub(crate) type Result<T = ()> = miette::Result<T, Error>;

pub(crate) struct Error {
    /// The inner error kind.
    pub inner: ErrorKind,
}

#[derive(Error, Diagnostic, Debug)]
pub(crate) enum ErrorKind {
    /// Something is wrong with the command arguments.
    #[error(transparent)]
    #[diagnostic(
        code(command::lexer_error),
        help("Something wen't wrong when lexeing the file")
    )]
    LexerError(#[from] doner_lexer::Error),
    /// Something is wrong with the command arguments.
    #[error(transparent)]
    #[diagnostic(
        code(command::parser_error),
        help("Something wen't wrong when parsing the file")
    )]
    ParserError(#[from] doner_parser::Error),
    /// Something is wrong with the command arguments.
    #[error(transparent)]
    #[diagnostic(
        code(command::io_error),
        help("Check the file path or directory and permissions")
    )]
    IoError(#[from] std::io::Error),
    /// All other errors that do not fit into a specific category.
    #[error(transparent)]
    #[diagnostic(
        code(command::execution_failed),
        help("Use the `--help` flag to see available options and arguments")
    )]
    Other(#[from] anyhow::Error),
}

impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Error {
            inner: ErrorKind::Other(err.into()),
        }
    }
}
