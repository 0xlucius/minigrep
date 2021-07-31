use std::env;

fn main() {
    // recieve an iteration of the provided command line arguments
    // and collect them into a vector
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
