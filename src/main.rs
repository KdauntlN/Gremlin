use std::process;
use minigrep::{Cli, Commands, FindConfig, GrepConfig};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    // This is a stupid way to handle CLI parsing but it's so much easier than doing it manually
    match &cli.command {
        Commands::Grep { query, file_path, ignore_case, count } => {
            let config = GrepConfig::new(query, file_path, ignore_case, count);

            if let Err(e) = minigrep::run_grep(config) {
                eprintln!("Application error: {e}");
                process::exit(1);
            }
        }

        Commands::Find { target, root } => {
            let config = FindConfig::new(target, root);
        }
    }
}
