use std::{fs, str::FromStr};
use anyhow::{anyhow, Error, Result, Context};
use std::env;
use std::collections::HashMap;


fn main() -> Result<()> {
    let file_name = env::args().nth(1).context("one file is necessary")?;
    println!("file_name: {:?}", file_name);

    let content = fs::read_to_string(file_name)?;

    let res : Vec<i32> = content.lines().flat_map(
        |x| -> Result<i32> {
            let (a, b) = x.split_once(',').context("Need to have ,")?;
            let (a_left, a_right) = a.split_once('-').context("Needs to have -")?;
            let a_left: i32 = str::parse(a_left)?;
            let a_right: i32 = str::parse(a_right)?;

            let (b_left, b_right) = b.split_once('-').context("Needs to have -")?;
            let b_left: i32 = str::parse(b_left)?;
            let b_right: i32 = str::parse(b_right)?;


            // if a_left <= b_left && a_right >= b_right {
            //     return Ok(1);
            // }
            // if b_left <= a_left && b_right >= a_right {
            //     return Ok(1)
            // }
            //
            //

            if a_left >= b_left && a_left <= b_right {
                return Ok(1)
            }

            if a_right >= b_left && a_right <= b_right {
                return Ok(1)
            }

            if b_left >= a_left && b_left <= a_right {
                return Ok(1)
            }

            if b_right >= a_left && b_right <= a_right {
                return Ok(1)
            }

            println!("arr {} {} {} {}", a_left, a_right, b_left, b_right);

            return Ok(0)
        }).collect();

    println!("res: {:?}", res);

    let sum: i32 = res.iter().sum();

    println!("sum: {:?}", sum);

    return Ok(());
}
