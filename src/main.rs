// This code is written as a simple project to learn rust
// most code will be overly commented as I am noting things to myself

use std::env;
use std::error::Error;
use std::fs;
use std::process;

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

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // does not use unwrap_or_else because run() does not
    // return any value to unwrap on success
    // so we only check if the result is Err
    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // This part could be made more efficent using a different method
        // I will change later when I learn how
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With Text:\n{}", contents);

    Ok(())
}
