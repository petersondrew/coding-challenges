use std::env;
use std::fs;

pub struct Config {
    case_sensitive: bool,
    file_names: Vec<String>,
    query: String,
}

impl Config {
    pub fn new() -> Result<Self, String> {
        let mut args = env::args().skip(1);
        let query: String = args.next().unwrap_or_else(|| String::new());
        if query.len() < 1 {
            return Err("A query is required".into());
        }
        let maybe_flag = args.next().unwrap_or_else(|| String::new());
        if maybe_flag.len() < 1 {
            return Err("At least one filename is required".into());
        }
        let case_sensitive = maybe_flag == "-i";
        let file_names: Vec<String> = if case_sensitive {
            args.collect()
        } else {
            vec![maybe_flag].into_iter().chain(args).collect()
        };

        if file_names.len() == 0 {
            return Err("One or more files to search is required".to_string());
        }
        Ok(Config {
            case_sensitive,
            file_names,
            query,
        })
    }
}

pub fn search_case_insensitive(
    query: impl Into<String>,
    text: impl Into<String>,
) -> Vec<(usize, String)> {
    let query = query.into().to_lowercase();

    let results = text
        .into()
        .lines()
        .enumerate()
        .filter_map(|(line_number, line)| {
            if line.to_lowercase().contains(&query) {
                return Some((line_number + 1, line.to_string()));
            }
            None
        })
        .collect();
    results
}

pub fn search(query: impl Into<String>, text: impl Into<String>) -> Vec<(usize, String)> {
    let query = query.into();
    let results = text
        .into()
        .lines()
        .enumerate()
        .filter_map(|(line_number, line)| {
            if line.contains(&query) {
                return Some((line_number + 1, line.to_string()));
            }
            None
        })
        .collect();
    results
}

pub fn run(config: Config) {
    config
        .file_names
        .iter()
        .map(|filename| {
            let file = fs::read_to_string(filename)
                .expect(&format!("We were unable to read file: {}", filename));
            if config.case_sensitive {
                let results = search(&config.query, file);
                (filename, results)
            } else {
                let results = search_case_insensitive(&config.query, file);
                (filename, results)
            }
        })
        .for_each(|(filename, results)| {
            println!();
            println!("{}", filename);
            println!();
            results
                .iter()
                .for_each(|(line_number, result)| println!("{}: {}", line_number, result))
        })
}

#[cfg(test)]
mod search_tests {

    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec![(2, "safe, fast, productive.".into())],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec![(1, "Rust:".into()), (4, "Trust me.".into())],
            search_case_insensitive(query, contents)
        );
    }
}
