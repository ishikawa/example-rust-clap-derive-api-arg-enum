//! ```ignore
//! $ cargo run --bin example-5 -- -h
//!     Finished dev [unoptimized + debuginfo] target(s) in 0.01s
//!      Running `target/debug/example-5 -h`
//! example-rust-clap-derive-api-arg-enum
//!
//! USAGE:
//!     example-5 --level <LEVEL>
//!
//! OPTIONS:
//!     -h, --help             Print help information
//!         --level <LEVEL>    [possible values: debug, info, warning, error, error-panic]
//! ```
use std::str::FromStr;

use clap::Parser;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Level {
    Debug,
    Info,
    Warning,
    Error { panic: bool },
}

const LEVEL_NAME_TO_VALUE_VARIANTS: [(&str, Level); 5] = [
    ("debug", Level::Debug),
    ("info", Level::Info),
    ("warning", Level::Warning),
    ("error", Level::Error { panic: false }),
    ("error-panic", Level::Error { panic: true }),
];

const LEVEL_POSSIBLE_VALUES: [&str; LEVEL_NAME_TO_VALUE_VARIANTS.len()] = [
    LEVEL_NAME_TO_VALUE_VARIANTS[0].0,
    LEVEL_NAME_TO_VALUE_VARIANTS[1].0,
    LEVEL_NAME_TO_VALUE_VARIANTS[2].0,
    LEVEL_NAME_TO_VALUE_VARIANTS[3].0,
    LEVEL_NAME_TO_VALUE_VARIANTS[4].0,
];

impl FromStr for Level {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        LEVEL_NAME_TO_VALUE_VARIANTS
            .iter()
            .find(|(name, _)| *name == s)
            .map(|(_, target)| target.clone())
            .ok_or_else(|| format!("Unrecognized input: {}", s))
    }
}

#[derive(clap::Parser)]
struct Args {
    #[clap(long = "level", possible_values=LEVEL_POSSIBLE_VALUES)]
    level: Level,
}

fn main() {
    let args = Args::parse();

    println!("level = {:?}", args.level);
}
