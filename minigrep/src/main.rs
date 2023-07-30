use std::env; 
use std::process;

use minigrep::{Config, run}; 

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("err when parsing arguments: {err}"); 
        process::exit(1); 
    });
    println!("searching for {} in {}", config.query, config.file_path); 

    if let Err(e) = run(config) {
        eprintln!("app err: {e}"); 
        process::exit(1); 
    }
}




