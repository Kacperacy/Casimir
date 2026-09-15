use casimir::perft;

const TESTS_FOLDER: &str = "tests/data";

fn load_perft_suite(file_name: &str) -> Vec<(String, u32, u64)> {
    // Load test cases from file
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(TESTS_FOLDER)
        .join(file_name);
    let perft_data = std::fs::read_to_string(&path).expect("Failed to read perft suite file");
    let mut test_cases = Vec::new();

    for line in perft_data.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue; // Skip empty lines and comments
        }

        // Parse format: FEN; depth count; depth count; ...
        let (fen, parts_str) = line
            .split_once(';')
            .expect("Invalid perft test case format");
        let fen = fen.trim().to_string();
        let parts: Vec<&str> = parts_str.split(';').collect();

        for part in parts {
            let (depth_str, count_str) = part
                .trim()
                .split_once(' ')
                .expect("Invalid perft test case format");
            let depth: u32 = depth_str[1..].parse().expect("Invalid depth value");
            let count: u64 = count_str.parse().expect("Invalid count value");

            test_cases.push((fen.clone(), depth, count));
        }
    }

    test_cases
}

fn run_perft_suite(file_name: &str) {
    let test_cases = load_perft_suite(file_name);

    // Run all test cases and collect failed ones
    let failed_cases: Vec<(String, u32, u64, Result<u64, String>)> = test_cases
        .into_iter()
        .filter_map(|(fen, depth, expected_count)| {
            let result = perft(&fen, depth);
            match result {
                Ok(count) if count == expected_count => None,
                Ok(count) => Some((fen, depth, expected_count, Ok(count))),
                Err(_) => Some((fen, depth, expected_count, Err("Error occurred".into()))),
            }
        })
        .collect();

    // Print all failed cases
    if failed_cases.is_empty() {
        println!("All test cases passed.");
    } else {
        eprintln!("Some test cases failed:");
        for (fen, depth, expected_count, actual_count) in &failed_cases {
            eprintln!(
                "Failed case: FEN: {}, Depth: {}, Expected: {}, Actual: {}",
                fen,
                depth,
                expected_count,
                match actual_count {
                    Ok(count) => count.to_string(),
                    Err(err) => err.clone(),
                }
            );
        }

        panic!("{} test cases failed.", failed_cases.len());
    }
}

#[test]
fn perft_depth_1() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let depth = 1;

    let result = perft(fen, depth);

    assert_eq!(result.expect("perft should not return an error"), 20);
}

#[ignore = "Not implemented yet"]
#[test]
fn perft_standard_suite() {
    run_perft_suite("perft-standard.epd");
}
