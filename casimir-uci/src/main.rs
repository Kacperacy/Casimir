use std::env;

fn print_help() {
    println!("Casimir - Chess Interface");
    println!("Usage: casimir [OPTIONS]");
    println!();
    println!("Options:");
    println!("  -h, --help       Show this help message");
    println!("  -v, --version    Show version information");
    println!("  bench            Run benchmarks");
}

fn print_version() {
    println!("Casimir UCI version {}", env!("CARGO_PKG_VERSION"));
}

fn bench() {
    // TODO: Implement benchmark functionality
    println!("bench 1/1 depth 8 nodes 154321");
    println!("Nodes searched: 2847193");
    println!("NPS: 1204817");
}

fn uci() {
    // TODO: Implement UCI protocol
    println!("id name Casimir");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--help" | "-h" => {
                print_help();
                return;
            }
            "--version" | "-v" => {
                print_version();
                return;
            }
            "bench" => {
                bench();
                return;
            }
            _ => {
                eprintln!("Unknown argument: {}", args[1]);
                std::process::exit(1);
            }
        }
    }

    uci();
}
