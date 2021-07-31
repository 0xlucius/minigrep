use std::env;

fn main() {
    // recieve an iteration of the provided command line arguments
    // and collect them into a vector
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
