//! ```ignore
//! $ cargo run --bin example-1 -- -h
//!     Finished dev [unoptimized + debuginfo] target(s) in 0.01s
//!      Running `target/debug/example-1 -h`
//! example-rust-clap-derive-api-arg-enum
//!
//! USAGE:
//!     example-1 --level <LEVEL>
//!
//! OPTIONS:
//!     -h, --help             Print help information
//!         --level <LEVEL>    [possible values: debug, info, warning, error]
//! ```
use clap::Parser;

#[derive(Debug, Clone, clap::ArgEnum)]
enum Level {
    Debug,
    Info,
    Warning,
    Error,
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
