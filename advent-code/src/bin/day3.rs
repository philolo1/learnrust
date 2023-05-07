use std::{fs, str::FromStr};
use anyhow::{anyhow, Error, Result, Context};
use std::env;
use std::collections::HashMap;


fn main() -> Result<()> {
    let file_name = env::args().nth(1).context("one file is necessary")?;
    println!("file_name: {:?}", file_name);

    let content = fs::read_to_string(file_name)?;


    let mut points = HashMap::new();
    let mut num = 0;

    for c in 'a'..='z' {
        num+= 1;
        points.insert(c, num);
    }

    for c in 'A'..='Z' {
        num+= 1;
        points.insert(c, num);
    }

    println!("points: {:?}", points);

    let text_lines: Vec<char> = content.lines()
        .map(|line| (line[0..line.len() /2].to_string(), line[line.len()/2..line.len()].to_string()))
        .flat_map(|(left, right)| {

            let mut map = HashMap::new();

            for a in left.chars() {
                map.insert(a, 1);
            }

            for a in right.chars() {
                if let Some(_) = map.get(&a) {
                    println!("Value {}", a);
                    return Ok(a);
                }
            }

            return Err(anyhow!("This should never happen"))
        })
        .collect();

    println!("TextLines : {:?}", text_lines);

    let score : i32 = text_lines.iter().map(|x| {
        let n: i32 = match points.get(x) {
            Some(num) => *num,
            None => 0,
        };
        return n;
    }).sum();


    println!("Score : {:?}", score);


    return Ok(());
}
