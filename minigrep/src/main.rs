use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let searchstring = &args[1];
    let filepath = &args[2];

    println!("searchstring = {searchstring}, filepath = {filepath}");
}
