use minigrep;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = minigrep::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // use if..let because there is nothing to unwrap
    if let Err(err) = minigrep::run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    };
}
