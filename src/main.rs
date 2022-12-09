
use std::env;
use std::process;

use ecma_rs::Lexer;
use ecma_rs::from_filepath;

#[derive(Debug)]
struct Config {
    path: String,

}

impl Config {

    fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Not enough arguments")
        }
        let path = args[1].clone();
        Ok(Config { path })
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    let contents = from_filepath(&config.path);
    let lexer = Lexer::new(&contents);
    let tokens: Vec<ecma_rs::Token> = lexer.collect();
    println!("The tokens: {:?}", tokens);
}
