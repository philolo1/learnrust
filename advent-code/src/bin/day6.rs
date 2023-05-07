use std::{fs, str::FromStr};
use anyhow::{anyhow, Error, Result, Context};
use std::env;
use std::collections::HashMap;

#[derive(Debug)]
struct Input {
    items: Vec<Vec<char>>,
}

fn main() -> Result<()> {
    let file_name = env::args().nth(1).context("one file is necessary")?;
    println!("file_name: {:?}", file_name);

    let content = fs::read_to_string(file_name)?;

    for i in 0..content.len() {
        let a = content.chars().nth(i).unwrap();
        let b = content.chars().nth(i + 1).unwrap();
        let c = content.chars().nth(i + 2).unwrap();
        let d = content.chars().nth(i + 3).unwrap();

        if a != b && a != c && a != d &&
            b!= c && b != d &&
            c != d
        {
            println!("Res: {}", i+4);
            break;
        }
    }

    return Ok(());
}
