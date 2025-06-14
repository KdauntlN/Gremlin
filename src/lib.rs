use std::{error::Error, fs};
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
    }
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub count: bool,
}

impl Config {
    pub fn build(query: &String, file_path: &String, ignore_case: &bool, count: &bool) -> Result<Config, &'static str> {

        let query = query.clone();
        let file_path = file_path.clone();

        let ignore_case = ignore_case.clone();
        let count = count.clone();

        Ok(Config { query, file_path, ignore_case, count })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
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
    if results.is_empty() {
        println!("Query was not found in specified file");
    }

    for (i, line) in results {
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
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let mut results = Vec::new();
    
    for (i, line) in contents.lines().enumerate() {
        if line.contains(query) {
            results.push((i, line));
        }
    }
    
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&query) {
            results.push((i, line));
        }
    }

    results
}

#[cfg(test)]
mod tests {
    // Should probably have some tests here but I'm lazy
}