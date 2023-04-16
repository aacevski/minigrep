use std::io::{BufRead, BufReader};

/// List of commands that will be executed in the terminal and the output will be searched.
pub const STDOUT_COMMANDS: [&str; 2] = ["ls", "pwd"];

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = std::env::var("CASE_INSENSITIVE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

/// Runs the program and searches for the query in the file, it could be case sensitive or not.
///
/// # Examples
/// ```
/// use minigrep::run;
/// use minigrep::Config;
///
/// let config = Config {
///    query: "duct".to_string(),
///    file_path: "poem.txt".to_string(),
///    ignore_case: false,
///};
///
/// run(config).unwrap();
///```
pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(config.file_path)?;

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

/// Runs a shell command and searches for the query in the output.
///
/// # Examples
/// ```
/// use minigrep::run_command;
/// use minigrep::Config;
///
/// let config = Config {
///   query: "ls".to_string(),
///   file_path: "poem.txt".to_string(),
///   ignore_case: false,
///};
///
/// run_command(config).unwrap();
pub fn run_command(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let output = std::process::Command::new(config.query)
        .output()
        .expect("failed to execute process");

    let reader = BufReader::new(output.stdout.as_slice());

    for line in reader.lines() {
        let line = line.unwrap();

        if line.contains(&config.file_path) {
            println!("{}", line);
        }
    }

    Ok(())
}

/// Search for the query in the contents and return the lines that contain it.
///
/// # Examples
///
/// ```
/// use minigrep::search;
///
/// let query = "duct";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Duct tape";
///
/// assert_eq!(vec!["safe, fast, productive."], search(query, contents));
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Search for the query in the contents and return the lines that contain it, ignoring the case.
///
/// # Examples
/// ```
/// use minigrep::search_case_insensitive;
///
/// let query = "rUsT";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Trust me.";
///
/// assert_eq!(
///    vec!["Rust:", "Trust me."],
///    search_case_insensitive(query, contents)
/// );
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
