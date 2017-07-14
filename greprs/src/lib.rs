use std::error::Error;
use std::fs::File;
use std::io::prelude::*;


#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough args");
        }

        let given_query = args[1].clone();
        let given_filename = args[2].clone();

        Ok(Config {
            query: given_query,
            filename: given_filename
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>>{
    let mut fh = File::open(config.filename)?;
    let mut contents = String::new();
    fh. read_to_string(&mut contents)?;
    let results = search(&config.query, &contents);
    for result in results {
        println!("{}", result);
    }
    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        } 
    }

    results
} 