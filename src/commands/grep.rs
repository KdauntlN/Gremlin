use std::fs;
use std::error::Error;

pub struct GrepConfig {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub count: bool,
}

impl GrepConfig {
    pub fn new(query: &String, file_path: &String, ignore_case: &bool, count: &bool) -> GrepConfig {
        let query = query.clone();
        let file_path = file_path.clone();
        let ignore_case = ignore_case.clone();
        let count = count.clone();

        GrepConfig { query, file_path, ignore_case, count }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&self.file_path)?;

    let results = if self.ignore_case {
        search_expr_case_insensitive(&self.query, &contents)
    } else {
        search_expr(&self.query, &contents)
    };

    if self.count {
        print_count(results);
    }
    else {
        print_lines(results);
    }

    Ok(())
}
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

fn search_expr<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let mut results = Vec::new();
    
    for (i, line) in contents.lines().enumerate() {
        if line.contains(query) {
            results.push((i, line));
        }
    }
    
    results
}

fn search_expr_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
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