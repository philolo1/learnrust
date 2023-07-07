use std::{fs, str::FromStr};
use anyhow::{anyhow, Error, Result, Context};
use std::env;
use std::collections::HashMap;


fn main() ->  Result<()> {
    let file_name = env::args().nth(1).context("One file is necessary")?;
    let content = fs::read_to_string(file_name)?;

    let mut tail = (0, 0);
    let mut head = (0, 0);

    let mut map : HashMap<(i32,i32), bool> = HashMap::new();

    map.insert(tail, true);

    for line in content.lines() {
        let (left, right) = line.split_once(' ').context("Something")?;

        let symbol = left;
        let amount: u32 = right.parse().context("Cannot convert right number")?;

        println!("Value: '{}' '{}'", left, right);

        for i in 0..amount {
            println!("close {:?} {:?}", tail, head);
            match symbol {
                "R" => {
                    println!("R");
                    head.0 += 1;
                },
                "L" => {
                    println!("L");
                    head.0 -= 1;
                },
                "U" => {
                    println!("U");
                    head.1 += 1;
                },
                "D" => {
                    println!("D");
                    head.1 -= 1;
                },
                _ => {
                    panic!("Unknown command");
                }
            }

            if (tail.0 - head.0).abs() <= 1 && (tail.1-head.1).abs() <=1 {
                // they touch so nothing to do
                continue;
            }

            if tail.0 == head.0 {
                if tail.1 > head.1 {
                    tail.1 -= 1;
                } else {
                    tail.1 += 1;
                }
            } else if tail.1 == head.1 {
                if tail.0 > head.0 {
                    tail.0 -= 1;
                } else {
                    tail.0 += 1;
                }
            } else {
                if head.0 > tail.0 {
                    tail.0 += 1;
                } else {
                    tail.0 -= 1;
                }

                if head.1 > tail.1 {
                    tail.1 += 1
                } else {
                    tail.1 -= 1
                }
            }
            println!("insert");
            map.insert(tail, true);
        }
    }

    let sum = map.iter().count();

    println!("Sum: {}", sum);


    Ok(())
}
