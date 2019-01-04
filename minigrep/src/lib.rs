use std::env;
use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.filename)?;

    let results = if config.case_insensitive {
        search_case_insensitive(&config.query, &file_content)
    } else {
        search(&config.query, &file_content)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
          result.push(line);
        }
    }

    result
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_insensitive = env::var("CASE_INSENSITIVE").is_ok();

        Ok(Config {
            query,
            filename,
            case_insensitive
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me!";

        assert_eq!(
            vec!["Rust:", "Trust me!"],
            search_case_insensitive(query, content));
    }
}