use std::{env, process};
use rust_minigrep::{Config, search};

fn main() {

    let args : Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap();

    let results = search(config).unwrap_or_else(|err|{
        println!("Application error: {}", err);
        process::exit(1);
    });
    println!("Result:: {:?} ", results);
}
