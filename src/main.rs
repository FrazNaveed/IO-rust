use std::env;
use std::process;
mod lib;
use lib::run;
use lib::Config;

fn main() {
    // getting env variables from the terminal and putting them in a vector
    let args: Vec<String> = env::args().collect();

    // handle the env variablees and case sensitive param
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // run the search
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
