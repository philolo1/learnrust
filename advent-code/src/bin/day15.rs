use std::cmp::{min, max};
use std::{fs};
use anyhow::{Result, Context};
use std::env;
use regex::Regex;

use std::collections::HashMap;

type Pair<'a> = (&'a str, &'a str);

fn main() ->  Result<()> {
    println!("Day 11");
    let file_name = env::args().nth(1).context("One file is necessary")?;
    let content = fs::read_to_string(file_name)?;

    // let lines: Vec<&str> = content.split("\n").filter(|x| x.len() > 0).collect();

    // dbg!(lines);



    // dbg!(&content);
    let re = Regex::new(r"Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)").unwrap();

    for c in re.captures_iter(&content) {
        let (_,arr) = c.extract::<4>();
        let arr = arr.map(|e| e.parse::<i32>().unwrap());
        dbg!(&arr);
    }



    Ok(())
}


