use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    run(config);
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

fn run(config: Config) {
    let contents = fs::read_to_string(config.filepath).expect("provided filepath does not exist");

    println!("contents:\n{}", contents);
}
