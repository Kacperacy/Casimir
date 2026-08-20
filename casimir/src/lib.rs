//! A chess engine library, and the core of the Casimir UCI engine.
//!
//! Chess960 is native, move generation is fully legal — moves come
//! straight from check and pin masks, with no pseudo-legal stage.
//!
//! No `unsafe`, no mandatory dependencies.
//!
//! Early development: it does not play chess yet, and the API is unstable.
#![warn(missing_docs)]

mod error;
mod perft;

pub use error::{Error, Result};
pub use perft::perft;
