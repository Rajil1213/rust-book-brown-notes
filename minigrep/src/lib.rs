use std::{error::Error, fs};

pub struct Config {
    searchstring: String,
    filepath: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &str> {
        // first => program_name, second, third => arguments
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let searchstring = args[1].clone();
        let filepath = args[2].clone();

        Ok(Self {
            searchstring,
            filepath,
        })
    }
}

fn search<'a>(searchstring: &'a str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filepath)?;

    println!("contents:\n{}", contents);
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
}
