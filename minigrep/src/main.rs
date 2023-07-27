use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let (searchstring, filepath) = parse_args(&args);
    let contents = fs::read_to_string(filepath).expect(&format!("{} does not exist", filepath));

    println!("searchstring = {searchstring}, filepath = {filepath}");
    println!("contents:\n{}", contents);
}

fn parse_args(args: &[String]) -> (&str, &str) {
    let searchstring = &args[1];
    let filepath = &args[2];

    (searchstring, filepath)
}
