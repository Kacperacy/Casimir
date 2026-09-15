use casimir::bench;

const TEST_FOLDER: &str = "tests/data";

fn load_bench_suite(file_name: &str) -> Vec<String> {
    // Load test cases from file
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(TEST_FOLDER)
        .join(file_name);
    let bench_data = std::fs::read_to_string(&path).expect("Failed to read bench suite file");
    let mut test_cases = Vec::new();

    for line in bench_data.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue; // Skip empty lines and comments
        }
        test_cases.push(line.to_string());
    }
    test_cases
}

fn run_bench_suite(file_name: &str, depth: u32, expected_total_nodes: u64) {
    let test_cases = load_bench_suite(file_name);

    // Run all test cases and sum the total nodes searched
    let total_nodes: u64 = test_cases
        .into_iter()
        .map(|fen| match bench(&fen, depth) {
            Ok(count) => count,
            Err(_) => panic!("Error occurred while running bench for FEN: {}", fen),
        })
        .sum();
    println!("Total nodes searched: {}", total_nodes);
    assert_eq!(total_nodes, expected_total_nodes);
}

#[ignore = "Not implemented yet"]
#[test]
fn bench_standard_suite() {
    const EXPECTED_TOTAL_NODES: u64 = 1000000; // Replace with the expected total nodes for the suite
    const DEPTH: u32 = 5; // Replace with the desired depth for the suite

    run_bench_suite("bench.epd", DEPTH, EXPECTED_TOTAL_NODES);
}
