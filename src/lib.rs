use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content: String = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &content) {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query: String = args[1].clone();
        let filename: String = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query: &str = "Rust";
        let content: &str = "Rust: safe, fast, productive. Pick three.";
        assert_eq!(vec!["Rust: safe, fast, productive. Pick three."], search(query, content));
    }
}
