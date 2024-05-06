use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // args can be any type that implements Iterator type and returns String items
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // Ignore name of program by consuming first element
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query string provided"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("No file path provided"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// Lifetime parameters: the returned vector should contain string
// slices that reference slices of the argument 'contents' (rather
// than the argument 'query'). This means that the data returned by
// the 'search' function will live was long as the data passed into
// the 'search' function by the 'contents' argument.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // no mutable state means parallel searches possible; no need
    // to manage concurrent access to results vector
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

/*
 _____         _
|_   _|__  ___| |_ ___
  | |/ _ \/ __| __/ __|
  | |  __/\__ \ |_\__ \
  |_|\___||___/\__|___/

*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "peak";
        let contents = "\
Peak Simpsons quote:
If I could just say a few words, 
I'd be a better public speaker.
-Homer Simpson";

        assert_eq!(
            vec!["I'd be a better public speaker."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
trustworthy
noisy
reliable
ancient
\"Four Words That Describe My Rusty Car\"";

        assert_eq!(
            vec!["trustworthy", "\"Four Words That Describe My Rusty Car\""],
            search_case_insensitive(query, contents)
        );
    }
}
