use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {} in {}", query, filename);

    let contents = fs::read_to_string(filename).expect("Unable to open file");
    println!("Text:\n{}", contents);
}
