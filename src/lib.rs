use std::{error::Error, fs};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    query: String,

    file_path: String,

    #[arg(long, short, help = "Ignore case in search")]
    ignore_case: bool,

    #[arg(long, short, help = "Display how many lines contain the query")]
    count: bool,
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub count: bool,
}

impl Config {
    pub fn build(args: Args) -> Result<Config, &'static str> {

        let query = args.query;
        let file_path = args.file_path;

        let ignore_case = args.ignore_case;
        let count = args.count;

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
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents ="\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec![(1, "safe, fast, productive.")], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec![(0, "Rust:"), (3, "Trust me.")], search_case_insensitive(query, contents));
    }
}