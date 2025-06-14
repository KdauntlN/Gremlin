use std::{error::Error, fs};
use clap::{Parser, Subcommand};

// I love clap
#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

// WHY IS CLAP SO ANNOYING
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

// Information from using the CLI because apparently that's idiomatic for some reason
pub struct GrepConfig {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub count: bool,
}

impl GrepConfig {
    // I hate this function signature but I hate multiline function signatures more
    pub fn new(query: &String, file_path: &String, ignore_case: &bool, count: &bool) -> GrepConfig {
        let query = query.clone();
        let file_path = file_path.clone();
        let ignore_case = ignore_case.clone();
        let count = count.clone();

        GrepConfig { query, file_path, ignore_case, count }
    }
}

pub struct FindConfig {
    pub target: String,
    pub root: String,
}

impl FindConfig {
    pub fn new(target: &String, root: &String) -> FindConfig {
        let target = target.clone();
        let root = root.clone();

        FindConfig { target, root }
    }
}

pub fn run_grep(config: GrepConfig) -> Result<(), Box<dyn Error>> {
    // I love error propogation why can't C or Python do anything like that
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_expr_case_insensitive(&config.query, &contents)
    } else {
        search_expr(&config.query, &contents)
    };

    if config.count {
        print_count(results);
    }
    else {
        print_lines(results);
    }

    Ok(())
}

fn print_lines(results: Vec<(usize, &str)>) {
    // Don't touch this Rust gets angry if you change it to results == 0
    if results.is_empty() {
        println!("Query was not found in specified file");
    }

    for (i, line) in results {
        // ":03" just means print this number with 3 digits by adding leading zeroes (4 becomes 004, 79 becomes 079, etc.)
        println!("{:03}.  {}", i + 1, line);
    }
}

fn print_count(results: Vec<(usize, &str)>) {
    if !results.is_empty() {
        println!("{} lines contained the expression", results.len());
    } else {
        println!("0 lines contained the expression")
    }
    
}

// Lifetimes are stupid
pub fn search_expr<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let mut results = Vec::new();
    
    for (i, line) in contents.lines().enumerate() {
        if line.contains(query) {
            results.push((i, line));
        }
    }
    
    results
}

// Lifetimes are still stupid
pub fn search_expr_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    // Tuples are also stupid but actually useful
    for (i, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&query) {
            results.push((i, line));
        }
    }

    results
}

// I probably don't need to box these errors
pub fn run_find(config: FindConfig) -> Result<Vec<String>, Box<dyn Error>> {

    Ok(vec![String::new()])
}

#[cfg(test)]
mod tests {
    // Should probably have some tests here but I'm lazy
}