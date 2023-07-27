use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let searchstring = &args[1];
    let filepath = &args[2];

    let contents = fs::read_to_string(filepath).expect(&format!("{} does not exist", filepath));

    println!("searchstring = {searchstring}, filepath = {filepath}");
    println!("contents:\n{}", contents);
}
