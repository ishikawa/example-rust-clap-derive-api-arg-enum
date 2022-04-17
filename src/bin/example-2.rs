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
