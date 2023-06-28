use std::env;
use std::error::Error;

use minigrep::Config;


fn main() -> Result<(),  Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args)?;

    return minigrep::run(config);
}



