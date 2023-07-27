use std::{env, error::Error, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // use if..let because there is nothing to unwrap
    if let Err(err) = run(config) {
        println!("Application error: {err}");
        process::exit(1);
    };
}

struct Config {
    searchstring: String,
    filepath: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Self, &str> {
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

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filepath)?;

    println!("contents:\n{}", contents);
    Ok(())
}
