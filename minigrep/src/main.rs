use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for: {}.", query);
    println!("In file: {}.", filename);

    let file_content = fs::read_to_string(filename)
        .expect("Could not read the file");

    println!("File content:\n{}", file_content);
}
