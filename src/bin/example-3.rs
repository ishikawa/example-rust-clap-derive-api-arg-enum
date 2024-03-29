//! ```ignore
//! $ cargo run --bin example-3 -- -h
//!     Finished dev [unoptimized + debuginfo] target(s) in 0.01s
//!      Running `target/debug/example-3 -h`
//! example-rust-clap-derive-api-arg-enum
//!
//! USAGE:
//!     example-3 --level <LEVEL>
//!
//! OPTIONS:
//!     -h, --help             Print help information
//!         --level <LEVEL>    [possible values: debug, info, warning, error, error-panic]
//! ```
use clap::Parser;

#[derive(Debug, Clone)]
enum Level {
    Debug,
    Info,
    Warning,
    Error { panic: bool },
}

const LEVEL_VALUE_VARIANTS: [Level; 5] = [
    Level::Debug,
    Level::Info,
    Level::Warning,
    Level::Error { panic: false },
    Level::Error { panic: true },
];

impl clap::ArgEnum for Level {
    fn value_variants<'a>() -> &'a [Self] {
        &LEVEL_VALUE_VARIANTS
    }

    fn to_possible_value<'a>(&self) -> Option<clap::PossibleValue<'a>> {
        let name = match self {
            Level::Debug => "debug",
            Level::Info => "info",
            Level::Warning => "warning",
            Level::Error { panic } => {
                if *panic {
                    "error-panic"
                } else {
                    "error"
                }
            }
        };

        Some(clap::PossibleValue::new(name))
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
