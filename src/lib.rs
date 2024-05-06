use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // could be optimised by not using clone
        let query = args[1].clone();
        let file_path = args[2].clone();

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
    // room for improvement
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // room for improvment
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
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
