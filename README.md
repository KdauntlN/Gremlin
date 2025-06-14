# minigrep

## Overview
minigrep is a lightweight command-line tool for searching text within files.  
It supports case-sensitive and case-insensitive search with an optional flag to ignore case.

## Features
- Search for a query string within a specified file  
- Optional `--ignore-case` (or `-i`) flag for case-insensitive search  
- Simple and easy-to-use CLI interface  
- Clean error handling for common issues like missing files or invalid arguments

## Usage
Basic usage syntax:  
```sh
minigrep <QUERY> <FILE_PATH> [OPTIONS]
```

- `QUERY`: The string to search for  
- `FILE_PATH`: Path to the file to search in  
- `OPTIONS`:  
  - `-i`, `--ignore-case` : Perform case-insensitive search
  - `-c`, `--count` : Displays how many lines contain the expression

Example:  
```sh
minigrep rust example.txt -i
```

This will search for "rust" in `example.txt` ignoring case.

## Notes
 - For a single-word query, no quotes are required
 - If you want to search for a query with multiple words, enclose the query in quotation marks.

Example:
```sh
minigrep "multi-word query" example.txt
```

## License
minigrep is licensed under the MIT License. See the LICENSE file for details.

## Author
Henry Knight

## Contact
 - henryknzed@gmail.com
 - +61 490 791 887
