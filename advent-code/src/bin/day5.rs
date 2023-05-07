use std::{fs, str::FromStr};
use anyhow::{anyhow, Error, Result, Context};
use std::env;
use std::collections::HashMap;

#[derive(Debug)]
struct Input {
    items: Vec<Vec<char>>,
}

impl FromStr for Input {
    type Err= Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut items: Vec<Vec<char>> = vec![];
        let lines = s.lines().last().unwrap();
        let num_containers = lines.split(' ').flat_map(str::parse::<i32>).max().context("should have a number")?;

        println!("num containers: {}", num_containers);

        let mut my_lines: Vec<&str> = s.lines().collect();
        println!("my lines {:?}", my_lines);

        for _ in 0..num_containers {
            items.push(vec![])
        }

        my_lines.pop();

        my_lines.reverse();

        for line in my_lines {
            println!("lines: {}", line);
            for index in 0..num_containers {
                let content = line
                    .chars()
                    .nth((4 * index + 1) as usize);

                if let Some(el) = content {
                    if el != ' ' {
                        println!("content: {}", el);
                        items[index as usize].push(el);

                    }
                }
            }
        }



        Ok(
            Input {
                items
            }
        )

    }
}

impl Input {
    fn move_item(&mut self, num: usize, from: usize, to: usize) {
        for _ in 0..num {
            let el = self.items[from].pop().unwrap();
            self.items[to].push(el);
        }
    }

    fn move_item2(&mut self, num: usize, from: usize, to: usize) {
        let mut v: Vec<char> = vec![];
        for _ in 0..num {
            let el = self.items[from].pop().unwrap();
            v.push(el)
        }

        for _ in 0..num {
            self.items[to].push(v.pop().unwrap());
        }

    }
    fn print_last_item(&mut self) {
        let mut st = String::from("");
        for v in self.items.iter() {
            let ch = *v.last().unwrap();
            st.push(ch);
        }
        println!("val: {}", st);
    }
}

fn main() -> Result<()> {
    let file_name = env::args().nth(1).context("one file is necessary")?;
    println!("file_name: {:?}", file_name);

    let content = fs::read_to_string(file_name)?;

    let (input, instructions) = content.split_once("\n\n").context("Needs to have input and instructions")?;

    println!("Input: {}", input);
    println!("Instructions: {}", instructions);

    let mut input: Input = str::parse(input)?;

    println!("Input: {:?}", input);

    for intr in instructions.lines() {
        let res: Vec<usize> = intr
            .split(' ')
            .filter(|&x| str::parse::<usize>(x).is_ok())
            .flat_map(str::parse::<usize>)
            .collect();
        println!("Arr: {:?}", res);
        input.move_item2(res[0], res[1] - 1, res[2] - 1);
        println!("El: {:?}", input);
    }

    input.print_last_item();


    return Ok(());
}
