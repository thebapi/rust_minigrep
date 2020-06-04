use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    file_name: String,
}

impl Config {
    pub fn new (args:&[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_name = args[2].clone();
        Ok(Config { query, file_name})
    }
}


pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
   // let contents = fs::read_to_string(config.file_name.clone())?;
    // println!("Searching for {}  in the {}", config.query, config.file_name);
    // println!("Contents:: {} ", contents);
    Ok(())
}

pub fn search <'a>(config: Config) -> Result<Vec<String>, Box<dyn Error>> {
    let mut results = Vec::new();
    {
        let contents = fs::read_to_string(&config.file_name)?;
        println!("Searching for {}  in the {}", config.query, config.file_name);
        println!("Contents:: {} ", &contents);
        let query = config.query.to_lowercase();
        println!("Query:: {} ", query);
        let lines = contents.lines();
        for line in  lines {
            if (line.to_lowercase().contains(&query)) {
                results.push(line.to_string());
            }
        }
    }
    Ok(results)
}



