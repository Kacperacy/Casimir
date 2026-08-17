# Casimir

A UCI chess engine and chess library written in Rust.

> **Early development.** The library is published to reserve the name; it does not play
> chess yet. See [Status](#status).

## Design

- **Library first.** The engine is one consumer of `casimir`, not the other way round.
  Board representation, move generation, evaluation and search are all usable on their own.
- **Chess960 native.** Castling is stored by rook file rather than assumed from fixed
  squares, so Fischer Random is supported by construction rather than bolted on.
- **Legal move generation.** Moves are generated directly from check and pin masks — there
  is no pseudo-legal stage and no legality filter.
- **No unsafe code**, and no mandatory dependencies.

## Layout

```
casimir/       library: board, move generation, evaluation, search
casimir-uci/   binary: UCI protocol
```

## Installation

Add the library to a project:

```toml
[dependencies]
casimir = "0.0.1"
```

The engine binary is published on the [releases page](https://github.com/kacperacy/Casimir/releases).
It is not distributed through crates.io.

## Building

Requires a recent stable Rust toolchain (edition 2024).

```bash
cargo build --release
```

The engine binary is produced at `target/release/casimir`.

## Usage

Casimir speaks UCI and works with any UCI-compatible GUI — Arena, Cute Chess, BanksiaGUI,
en-croissant.

```bash
./target/release/casimir
```

Point your GUI at that binary to add it as an engine.

## Testing

```bash
cargo test --workspace
```

Move generation is verified with perft against known node counts for both standard chess
and Chess960. The deep perft suite is ignored by default:

```bash
cargo test --release -- --ignored
```

## Status

| | |
|---|---|
| Board representation and FEN | in progress |
| Move generation | not started |
| Search | not started |
| Evaluation | not started |
| UCI | not started |
| Chess960 | not started |

## License

MIT
