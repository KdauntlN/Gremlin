use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Grep {
        query: String,

        file_path: String,

        #[arg(long, short, help = "Ignore case in search")]
        ignore_case: bool,

        #[arg(long, short, help = "Display how many lines contain the query")]
        count: bool,
    },

    Find {
        target: String,

        #[clap(default_value = "C:\\")]
        root: String
    }
}