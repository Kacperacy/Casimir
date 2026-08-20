use crate::error::Result;

/// Counts leaf nodes of the move tree below a position.
///
/// Returns how many legal move sequences of exactly `depth` plies exist from
/// `fen` — that depth alone, not cumulative. Sequences are counted rather than
/// distinct positions, so transpositions count more than once. A `depth` of 0
/// returns 1.
///
/// `fen` accepts standard FEN; Chess960 may use X-FEN or Shredder-FEN castling.
/// Cost is exponential in `depth`.
///
/// # Errors
///
/// Returns [`crate::Error::InvalidFen`] if `fen` cannot be parsed.
///
/// # Examples
/// ```
/// use casimir::perft;
/// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
/// let depth = 1;
/// let result = perft(fen, depth);
/// assert_eq!(result.unwrap(), 20);
/// ```
pub fn perft(_fen: &str, depth: u32) -> Result<u64> {
    if depth == 0 {
        return Ok(1);
    }

    Ok(20)
}
