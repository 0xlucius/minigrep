// This code is written as a simple project to learn rust
// most code will be overly commented as I am noting things to myself

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // recieve an iteration of the provided command line arguments
    // and collect them into a vector
    let args: Vec<String> = env::args().collect();

    // unwrap_or_else will, on success, unwrap the Ok() and return the
    // value inside, or upon fail, it will run the closure
    // anon function we passed into it.
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // does not use unwrap_or_else because run() does not
    // return any value to unwrap on success
    // so we only check if the result is Err
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
