use std::error::Error;
use std::fs;

pub struct Args {
    pub query: String,
    pub file_path: String,
}

impl Args {
    pub fn build(args: &[String]) -> Result<Args, &'static str> {
        // convention: new() fn is expected to never fail so build() used here
        if args.len() < 3 {
            return Err("Not enough arguments passed to the command!");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Args { query, file_path })
    }
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(args.file_path)?;

    println!("File content: {}", file_contents);

    Ok(())
}