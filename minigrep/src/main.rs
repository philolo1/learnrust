use std::env;
use std::error::Error;

use minigrep::Config;


fn main() -> Result<(),  Box<dyn Error>> {

    let config = Config::new(env::args())?;

    return minigrep::run(config);
}



