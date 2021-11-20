use minigrep_ng::Config;
use std::io;
use std::{env, process};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = Config::new(&args).unwrap_or_else(|error| {
        eprintln!("Problem parsing the arguments: {}.", error);
        process::exit(1);
    });

    if let Err(error) = minigrep_ng::run(&config) {
        eprintln!("Problem running the program: {}.", error);
        process::exit(1);
    }
}
