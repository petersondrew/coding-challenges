use std::env;
use std::fs::File;
use std::io::{BufReader, Lines, Result};
use std::io::prelude::*;
use std::path::Path;

pub fn run(config: Config) -> Result<()> {
    let lines = read_lines(config.file_path)?;

    let results = match config.ignore_case {
        true => isearch,
        false => search
    }(
        &config.query,
        lines
    )?;

    for line in &results {
        println!("{}", line);
    }

    if results.is_empty() {
        println!("¯\\_(ツ)_/¯");
    }

    Ok(())
}

// Stream the file
fn read_lines<P>(file_path: P) -> Result<Lines<impl BufRead>>
where P: AsRef<Path>, {
    let file = File::open(file_path)?;
    Ok(BufReader::new(file).lines())
}

fn search(query: &str, lines: Lines<impl BufRead>) -> Result<Vec<String>> {
    lines
        .filter(|r| r.as_ref().is_ok_and(|line| line.contains(query)))
        .collect()
}

fn isearch(query: &str, lines: Lines<impl BufRead>) -> Result<Vec<String>> {
    lines
        .filter(|r| r.as_ref().is_ok_and(|line| {
            line.to_lowercase().contains(query.to_lowercase().as_str())
        }))
        .collect()
}

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    // Taking env::args as something implementing Iterator<String> allows us to avoid cloning
    pub fn build(mut args: impl Iterator<Item = String>) -> std::result::Result<Config, &'static str> {
        // Skip args[0]
        args.next();

        let query = args.next().ok_or("Please supply a query")?;

        let file_path = args.next().ok_or("Please supply a file path")?;

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let res = search(query, contents.as_bytes().lines());
        match res {
            Ok(v) => assert_eq!(vec!["safe, fast, productive."], v),
            _ => assert!(false)
        };
    }

    #[test]
    fn multiple_results() {
        let query = "t";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let res = search(query, contents.as_bytes().lines());
        match res {
            Ok(v) => assert_eq!(vec!["Rust:", "safe, fast, productive.", "Pick three."], v),
            _ => assert!(false)
        };
    }

    #[test]
    fn no_results() {
        let query = "99";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let res = search(query, contents.as_bytes().lines());
        match res {
            Ok(v) => assert_eq!(Vec::<String>::new(), v),
            _ => assert!(false)
        };
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let res = isearch(query, contents.as_bytes().lines());
        match res {
            Ok(v) => assert_eq!(vec!["Rust:", "Trust me."], v),
            _ => assert!(false)
        };
    }
}