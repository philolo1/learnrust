use std::{fs, str::FromStr};
use anyhow::{anyhow, Error, Result, Context};
use std::env;
use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug)]
enum OpItem {
    Value,
    Num(i32)
}

#[derive(Debug)]
enum Operator {
    Plus,
    Times,
}

#[derive(Debug)]
struct Operation {
    left: OpItem,
    operator: Operator,
    right: OpItem,
}

impl Operation {
    fn new(content: &str) -> Operation {
        Operation { left: OpItem::Value, operator: Operator::Plus, right: OpItem::Value }
    }
}

#[derive(Debug)]
struct Monkey {
    label: i32,
    items: Vec<i32>,
    operation: Operation,
    divisible: i32,
    true_case: i32,
    false_case: i32
}

impl Monkey {
    fn new(content: &str, label: &mut i32) -> Monkey {
        let mut arr: Vec<&str> = content.split('\n').skip(1).collect();
        println!("Item {:?}", arr.reverse());

        // get starting items
        let start_items = arr.pop().unwrap();
        let (_, start_items) = start_items.split_once(":").unwrap();
        println!("start_items: {:?}", start_items);
        let start_items: Vec<i32> = start_items
            .split(",")
            .flat_map(|x| x.trim().parse())
            .collect();

        println!("Test: {:?}", start_items);

        let operations = arr.pop().unwrap();
        println!("operations {}", operations);
        let operations = Operation::new(operations);
        // TODO

        let divisible  = arr.pop().unwrap();
        let (_, divisible) = divisible.split_once("by ").unwrap();
        let divisible: i32 = divisible.parse().unwrap();

        println!("Divisible {}", divisible);

        let true_case  = arr.pop().unwrap();
        println!("True case {}", true_case);
        let true_case = true_case.split_once("monkey ").unwrap().1;
        let true_case: i32 = true_case.parse().unwrap();

        let false_case  = arr.pop().unwrap();
        let false_case = false_case.split_once("monkey ").unwrap().1;
        let false_case: i32 = false_case.parse().unwrap();


        let monkey = Monkey {
            label: *label,
            items: start_items,
            operation: operations,
            divisible,
            true_case,
            false_case,
        };
        *label += 1;

        return monkey;


    }
}


fn main() ->  Result<()> {
    println!("Day 11");
    let file_name = env::args().nth(1).context("One file is necessary")?;
    let content = fs::read_to_string(file_name)?;

    let monkey_data: Vec<&str> = content.split("Monkey").filter(|x| x.len() > 0).collect();

    let mut counter = 0;
    for item in monkey_data {
        let monkey = Monkey::new(item, &mut counter);
        println!("{:?}", monkey);
    }


    Ok(())
}
