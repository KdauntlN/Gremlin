use std::process;
use minigrep::Config;
use minigrep::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();

    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
