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

    let mut text_lines: Vec<&str> = content.lines().collect();

    let mut new_arr: Vec<(&str,&str, &str)> = vec![];

    loop {
        if text_lines.len() == 0 {
            break;
        }
        new_arr.push(
            (
                text_lines.pop().unwrap(),
                text_lines.pop().unwrap(),
                text_lines.pop().unwrap()
            )
        )
    };

    let result: Vec<char>  = new_arr
        .iter()
        .flat_map(|(b1, b2, b3)| {
            let mut map = HashMap::new();
            let mut map2 = HashMap::new();

            for a in b1.chars() {
                map.insert(a, 1);
            }

            for a in b2.chars() {
                map2.insert(a, 1);
            }

            for a in b3.chars() {
                if let Some(_) = map.get(&a) {
                    if let Some(_) = map2.get(&a) {
                        return Ok(a);
                    }
                }
            }

            return Err(anyhow!("This should never happen"))
        })
        .collect();

    println!("TextLines : {:?}", text_lines);

    let score : i32 = result.iter().map(|x| {
        let n: i32 = match points.get(x) {
            Some(num) => *num,
            None => 0,
        };
        return n;
    }).sum();


    println!("Score : {:?}", score);


    return Ok(());
}
