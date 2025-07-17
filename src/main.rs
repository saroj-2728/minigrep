use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect(); // Gets all passed args into an iterator and .collect() turns the iterator into a collection (here vector)

    let args = minigrep::Args::build(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {}", err);
        println!("\nUsage: minigrep 'search-query' 'file-path'");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(args) {
        println!("Application error: {e}");
        process::exit(1);
    }
}