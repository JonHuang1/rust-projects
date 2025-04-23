use std::env;
use std::error::Error;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    
    #[test]
    fn case_insensitive() {
        let query="rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

pub fn search_case_insensitive<'a>(
    query:&str, 
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results: Vec<&'a str> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&'a str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
/*
 * "new" changed to "build" because programmers never expect "new" to fail 
 */
//    fn new(args: &[String]) -> Config {
//        if args.len() < 3 {
//            panic!("not enough arguments");
//        }

//        let query: String = args[1].clone();
//        let file_path: String = args[2].clone();

//        Config { query, file_path }
//    }

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
        let query = args[1].clone();
        let file_path = args[2].clone();

        // .is_ok() returns Result type and just checks if IGNORE_CASE 
        // is set to anything at al if it is returns value it is set to
        // else returns false. For this we don't care what the value is
        // so we use .is_ok() instead of unwrap(), expect(), etc
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { 
            query, 
            file_path,
            ignore_case,
        })
    }
}

/*
 * logic moved into Config constructor
 */
//fn _parse_config(args: &[String]) -> Config {
//    let query: String = args[1].clone();
//    let file_path: String = args[2].clone();

//    Config { query, file_path }
//} 

