use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
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

        Ok(Config { query, file_path })
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

