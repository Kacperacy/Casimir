use casimir::perft;

#[test]
fn perft_depth_1() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let depth = 1;

    let result = perft(fen, depth);

    assert_eq!(result.expect("perft should not return an error"), 20);
}
