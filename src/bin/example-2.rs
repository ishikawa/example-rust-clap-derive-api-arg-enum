//! ```ignore
//! $ cargo run --bin example-2 -- -h
//! error: `#[derive(ArgEnum)]` only supports non-unit variants, unless they are skipped
//!  --> src/bin/example-2.rs:8:5
//!   |
//! 8 |     Error { panic: bool },
//!   |     ^^^^^
//!
//! error: could not compile `example-rust-clap-derive-api-arg-enum` due to previous error
//! ```
use clap::Parser;

#[derive(Debug, Clone, clap::ArgEnum)]
enum Level {
    Debug,
    Info,
    Warning,
    Error { panic: bool },
}

#[derive(clap::Parser)]
struct Args {
    #[clap(arg_enum, long = "level")]
    level: Level,
}

fn main() {
    let args = Args::parse();

    println!("level = {:?}", args.level);
}
