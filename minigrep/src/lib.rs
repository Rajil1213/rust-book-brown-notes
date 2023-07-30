use std::{env, error::Error, fs};

pub struct Config {
    searchstring: String,
    filepath: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &str> {
        // first => program_name, second, third => arguments
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        const IGNORE_CASE_ENV_KEY: &str = "IGNORE_CASE";

        let searchstring = args[1].clone();
        let filepath = args[2].clone();
        let ignore_case = env::var(IGNORE_CASE_ENV_KEY).is_ok();

        Ok(Self {
            searchstring,
            filepath,
            ignore_case,
        })
    }
}

fn search<'a>(searchstring: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = vec![];

    for line in contents.lines() {
        if line.contains(searchstring) {
            result.push(line);
        }
    }

    result
}

fn case_insensitive_search<'a>(searchstring: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = vec![];

    let searchstring = searchstring.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&searchstring) {
            result.push(line)
        }
    }

    result
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filepath)?;

    let matching_lines = match config.ignore_case {
        true => case_insensitive_search(&config.searchstring, &contents),
        false => search(&config.searchstring, &contents),
    };

    for matching_line in matching_lines {
        println!("{matching_line}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let searchstring = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(searchstring, contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let searchstring = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(searchstring, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let searchstring = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive.", "Duct tape."],
            case_insensitive_search(searchstring, contents)
        );
    }
}
