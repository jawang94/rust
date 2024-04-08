//! # Contained Comment Example
//!
//! Describes the container, aka lib.rs. Shows on front page.

use std::env;
use std::error::Error;
use std::fs;

/// Example Documentation Comment
/// ```
/// // Dat block of code
/// let rust: i32 = String::from("hey");
/// ```
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    // Log some stuff
    println!("Searching for {0}", config.query);
    println!("In file {0}", config.file_path);
    println!("With text:\n{contents}");
    println!("\n...Searching...\n");

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for (i, line) in results.iter().enumerate() {
        println!("{i}. {line}");
    }

    Ok(())
}

pub struct Config<'a> {
    query: &'a String,
    file_path: &'a String,
    ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // Parse config and env
        let mut args_iterator = args.iter(); // iter consumes immutable ref
                                             // we need mut on the iterator because we call it multiple times
        args_iterator.next();
        let query = match args_iterator.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string."),
        };
        let file_path = match args_iterator.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path."),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect() // iterate lines, filter w/ a closure, have it capture query from environment, return a vec with collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:\nsafe, fast, productive.\nPick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "rUSt";
        let contents = "\
      Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
