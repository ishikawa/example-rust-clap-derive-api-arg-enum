//! ```ignore
//! $ cargo run --bin example-4 -- -h
//!     Finished dev [unoptimized + debuginfo] target(s) in 0.03s
//!      Running `target/debug/example-4 -h`
//! example-rust-clap-derive-api-arg-enum
//!
//! USAGE:
//!     example-4 --level <LEVEL>
//!
//! OPTIONS:
//!     -h, --help             Print help information
//!         --level <LEVEL>    [possible values: debug, info, warning, error, error-panic]
//! ```
use clap::Parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Level {
    Debug,
    Info,
    Warning,
    Error { panic: bool },
}

const LEVEL_NAME_TO_VALUE_VARIANTS: [(&'static str, Level); 5] = [
    ("debug", Level::Debug),
    ("info", Level::Info),
    ("warning", Level::Warning),
    ("error", Level::Error { panic: false }),
    ("error-panic", Level::Error { panic: true }),
];

const LEVEL_VALUE_VARIANTS: [Level; LEVEL_NAME_TO_VALUE_VARIANTS.len()] = [
    LEVEL_NAME_TO_VALUE_VARIANTS[0].1,
    LEVEL_NAME_TO_VALUE_VARIANTS[1].1,
    LEVEL_NAME_TO_VALUE_VARIANTS[2].1,
    LEVEL_NAME_TO_VALUE_VARIANTS[3].1,
    LEVEL_NAME_TO_VALUE_VARIANTS[4].1,
];

impl clap::ArgEnum for Level {
    fn value_variants<'a>() -> &'a [Self] {
        &LEVEL_VALUE_VARIANTS
    }

    fn to_possible_value<'a>(&self) -> Option<clap::PossibleValue<'a>> {
        LEVEL_NAME_TO_VALUE_VARIANTS
            .iter()
            .find(|(_, level)| self == level)
            .map(|(name, _)| clap::PossibleValue::new(name))
    }
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
