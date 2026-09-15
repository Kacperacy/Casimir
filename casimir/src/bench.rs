use crate::error::Result;

/// Counts nodes searched from a position to a fixed depth.
///
/// Returns the number of nodes visited by the search from `fen` at exactly
/// `depth`. The count is deterministic: fixed input, fixed depth, single
/// thread, integer-only arithmetic, and no clock reads, so it is identical on
/// every machine. It changes only when search behaviour changes, never for
/// pure speed work.
///
/// Currently a stub that returns a constant; real search is not implemented.
///
/// # Errors
///
/// Returns [`crate::Error::InvalidFen`] if `fen` cannot be parsed.
///
/// Cost is exponential in `depth`.
pub fn bench(_fen: &str, _depth: u32) -> Result<u64> {
    Ok(1)
}
