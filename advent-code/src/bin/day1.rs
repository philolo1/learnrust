use std::fs;
use anyhow::{Result, Context};
use std::env;

fn main() -> Result<()> {
    let file_name = env::args().nth(1).context("one file is necessary")?;
    println!("file_name: {:?}", file_name);
    let file_content = fs::read_to_string(file_name)?;
    let arr : Vec<&str> = file_content.split("\n\n").collect();

    println!("Arr: {:?}", arr);

    let num_arr: Vec<i32> = arr.iter().map(|user| {
        let lines: Vec<i32> = user.lines().flat_map(str::parse::<i32>).collect();
        let mut sum = 0;
        for num in lines {
            sum += num;
        }
        return sum;
    }).collect();

    println!("NumArr: {:?}", num_arr);
    let max: &i32 = num_arr.iter().max().context("Something")?;
    println!("Max: {:?}", max);

    return Ok(());
}
