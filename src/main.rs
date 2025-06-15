use std::process;
use gremlin::cli::{Cli, Commands};
use gremlin::commands::find::FindConfig;
use gremlin::commands::grep::GrepConfig;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Grep { query, file_path, ignore_case, count } => {
            let config = GrepConfig::new(query, file_path, ignore_case, count);

            if let Err(e) = config.run() {
                eprintln!("Application error: {e}");
                process::exit(1);
            }
        }

        Commands::Find { target, root } => {
            let config = FindConfig::build(target, root).unwrap_or_else(|err| {
                println!("Application error: {err}");
                process::exit(1);
            });

            config.run_search();
        }
    }
}
