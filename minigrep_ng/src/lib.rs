use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(&config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query, &file_content)
    } else {
        search_case_insensitive(&config.query, &file_content)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .split("\n")
        .filter(|&line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_ascii_lowercase();
    contents
        .split("\n")
        .filter(|&line| line.to_ascii_lowercase().contains(&query))
        .collect()
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            Err("Not enough arguments")
        } else {
            let query = args[1].clone();
            let filename = args[2].clone();

            let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

            Ok(Config {
                query,
                filename,
                case_sensitive,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_parsing() {
        let filename = "foo".to_string();
        let query = "bar".to_string();
        let config =
            Config::new(&vec!["foobar".to_string(), query.clone(), filename.clone()]).unwrap();

        assert_eq!(config.filename, filename);
        assert_eq!(config.query, query);
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = get_content();

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    fn get_content() -> &'static str {
        "\
Rust:
safe, fast, productive.
Duct tape.
Pick three.
Trust me."
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = get_content();

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
