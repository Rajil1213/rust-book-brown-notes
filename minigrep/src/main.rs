use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_args(&args);
    println!(
        "searchstring = {}, filepath = {}",
        config.searchstring, config.filepath
    );

    let contents = fs::read_to_string(config.filepath).expect("provided filepath does not exist");

    println!("contents:\n{}", contents);
}

struct Config {
    searchstring: String,
    filepath: String,
}

fn parse_args(args: &[String]) -> Config {
    let searchstring = args[1].clone();
    let filepath = args[2].clone();

    Config {
        searchstring,
        filepath,
    }
}
