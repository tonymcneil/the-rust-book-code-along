use std::{env, fs};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
        args.next(); // NOTE: the first arg is always the programs name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string!".to_string()),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename!".to_string()),
        };

        Ok(Config {
            query: query,
            filename: filename,
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}

// pub fn run(config: &Config, mut _output: impl std::io::Write) -> Result<(), String> {
pub fn run(config: &Config) -> Result<(), String> {
    let contents = match fs::read_to_string(&config.filename) {
        Ok(contents) => contents,
        Err(msg) => {
            let wrapped_msg = format!("Could not read file contents, got error {}", msg);
            return Err(wrapped_msg);
        }
    };

    let results = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = &query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_case_sensitive_single_result() {
        // given
        let query = "duct";
        let contents = "\
        Rust:\n\
        safe, fast, productive.\n\
        Pick three.\n\
        Dust tape";

        // when
        let res = search_case_sensitive(query, contents);

        assert_eq!(vec!["safe, fast, productive."], res);
    }

    #[test]
    fn search_case_insensitive_single_result() {
        // given
        let query = "rUsT";
        let contents = "\
        Rust:\n\
        safe, fast, productive.\n\
        Pick three.\n\
        Trust me.";

        // when
        let res = search_case_insensitive(query, contents);

        // then
        assert_eq!(vec!["Rust:", "Trust me."], res);
    }
}
