use std::process;
use minigrep::{Config, Cli, Commands};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Grep { query, file_path, ignore_case, count } => {
            let config = Config::build(query, file_path, ignore_case, count).unwrap_or_else(|err| {
                eprintln!("Problem parsing arguments: {err}");
                process::exit(1);
            });

            if let Err(e) = minigrep::run(config) {
                eprintln!("Application error: {e}");
                process::exit(1);
            }
        }
    }
}
