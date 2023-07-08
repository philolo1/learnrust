use std::{fs, str::FromStr};
use anyhow::{anyhow, Error, Result, Context};
use std::env;
use std::collections::HashMap;

#[derive(Debug)]
enum Operation {
    Add(i32),
    Noop
}

impl FromStr for Operation {
    type Err= Error;

    fn from_str(s: &str) -> Result<Self> {
        if s == "noop" {
            return Ok(
                Operation::Noop
            );
        }

        let (_,num) = s.split_once(' ').context("No split available")?;
        let num: i32 = num.parse()?;

        return Ok(
            Operation::Add(num)
        )
    }
}

struct Item {
    value: i32,
    time: i32,
    result: i32
}

impl Item {
    fn new() -> Item {
        return Item {
            value: 1,
            time: 1,
            result: 0
        }
    }

    fn increaseTime(&mut self, add: i32) {
        self.time += 1;
        self.value += add;

        match self.time {
            20|60|100|140|180|220 => {
                println!("Circle (time: {} value: {})", self.time, self.value);
                self.result += self.time * self.value;
            },
            _ => {
            }
        }
    }
}


fn main() ->  Result<()> {
    let file_name = env::args().nth(1).context("One file is necessary")?;
    let content = fs::read_to_string(file_name)?;

    let operations : Vec<Operation> = content.lines().flat_map(|line| line.parse()).collect();

    println!("op {:?}", operations);

    let mut time = 0;
    let mut value = 1;

    let mut item = Item::new();

    for o in operations {
        match o {
            Operation::Noop => {
                item.increaseTime(0);
            },
            Operation::Add(x) => {
                item.increaseTime(0);
                item.increaseTime(x);
            }
        }
    }

    println!("Result: {}", item.result);




    Ok(())
}
