use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query=  &args[1];
    let filename=  &args[2];
    
    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong while reading the file!");

    println!("With text:\n{}", contents)
}

fn parse_config(args: &[String]) -> (&str, &str) {
    
}