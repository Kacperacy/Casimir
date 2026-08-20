/// Errors from input this crate could not interpret.
///
/// `#[non_exhaustive]`: match with a wildcard arm.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// A FEN string could not be parsed.
    InvalidFen(String),
}

/// This crate's result type, with [`Error`] as the error.
pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidFen(fen) => write!(f, "Invalid FEN string: {}", fen),
        }
    }
}

impl std::error::Error for Error {}
