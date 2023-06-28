use std::env;
use std::fs;
use std::error::Error;

struct Config {
    query: String,
    filename: String
}

fn main() -> Result<(),  Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args)?;

    println!("Searching for {}", config.query);
    println!("File {}", config.filename);

    let  contents = fs::read_to_string(config.filename)?;

    println!("Content: {}", contents);

    return Ok(());
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {

        if args.len() != 3 {
            return Err("Exactly two parameters need to be provided");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            query,
            filename
        })
    }
}
