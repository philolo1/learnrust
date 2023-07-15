use std::{fs, str::FromStr};
use anyhow::{anyhow, Error, Result, Context};
use std::env;
use std::collections::HashMap;
use itertools::Itertools;

use std::cell::{RefCell};
use std::rc::{Rc};

#[derive(Debug, PartialEq, Clone)]
enum OpItem {
    Value,
    Num(i32)
}

#[derive(Debug, PartialEq, Clone)]
enum Operator {
    Plus,
    Times,
}


impl OpItem {
    fn new(str: &str) -> OpItem {
        let num: Result<i32, _> = str.parse();

        match num {
            Ok(val) => OpItem::Num(val),
            Err(_) => OpItem::Value,
        }
    }
}

#[derive(Debug)]
struct Operation {
    left: OpItem,
    operator: Operator,
    right: OpItem,
}

impl Operation {
    fn new(content: &str) -> Operation {
        // Operation: new = old * 19"
        let str = content.split_once("= ").unwrap().1;



        let op = if str.contains("*") {
            Operator::Times
        } else {
            Operator::Plus
        };


        let (left, right) = match op {
            Operator::Times => {
                str.split_once(" * ").unwrap()
            },
            Operator::Plus => {
                str.split_once(" + ").unwrap()
            },
        };


        Operation {
            left: OpItem::new(left),
            operator: op,
            right: OpItem::new(right)
        }
    }
}

mod tests {
    use super::Operation;
    use super::Operator;
    use super::OpItem;

    fn my_test(str: &str, op1: OpItem, op2: Operator, op3: OpItem) {
        let op = Operation::new(str);

        assert_eq!(op1, op.left);
        assert_eq!(op2, op.operator);
        assert_eq!(op3, op.right);
    }

    #[test]
    #[cfg(test)]
    fn test_new() {
        my_test(
            " Operation: new = old * 19",
            OpItem::Value,
            Operator::Times,
            OpItem::Num(19)
        );

        my_test(

  "Operation: new = old + 6",
  OpItem::Value,
  Operator::Plus,
  OpItem::Num(6)
        );

        my_test(

  "Operation: new = old * old",
  OpItem::Value,
  Operator::Times,
  OpItem::Value
        );
    }
}

#[derive(Debug)]
struct WorryNumber {
    num: i32,
    values: Vec<i32>,
    dividers: Rc<Vec<i32>>,
}

impl WorryNumber {
    fn new(num: i32) -> WorryNumber {
        WorryNumber {
            num,
            values: vec![],
            dividers: Rc::new(vec![])
        }
    }
}

#[derive(Debug)]
struct Monkey {
    label: i32,
    items: Vec<WorryNumber>,
    operation: Operation,
    divisible: i32,
    true_case: i32,
    false_case: i32,
    inspect_number: i32
}

impl Monkey {
    fn new(content: &str, label: &mut i32) -> Monkey {
        let mut arr: Vec<&str> = content.split('\n').skip(1).collect();
        println!("Item {:?}", arr.reverse());

        // get starting items
        let start_items = arr.pop().unwrap();
        let (_, start_items) = start_items.split_once(":").unwrap();
        println!("start_items: {:?}", start_items);
        let start_items: Vec<WorryNumber> = start_items
            .split(",")
            .flat_map(|x| x.trim().parse())
            .map(|x| WorryNumber::new(x))
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
            inspect_number: 0
        };
        *label += 1;

        return monkey;


    }

    fn increase_inspect(&mut self) {
        self.inspect_number +=  self.items.len() as i32;
    }

    fn calculate(&mut self, num: &mut WorryNumber, dividers: &mut Vec<i32>) -> i32 {

        if num.values.len() == 0 {
            num.add_dividers(dividers);
        }



        let num = *num;

        let left_number = match self.operation.left {
            OpItem::Value => num,
            OpItem::Num(v) => v,
        };

        let right_number = match self.operation.right {
            OpItem::Value => num,
            OpItem::Num(v) => v,
        };

        const PRIME: i32 = 7_919;

        let result = match self.operation.operator {
            Operator::Plus => (left_number + right_number) % PRIME,
            Operator::Times => (left_number * right_number) % PRIME,
        };

        return result
    }
}


fn main() ->  Result<()> {
    println!("Day 11");
    let file_name = env::args().nth(1).context("One file is necessary")?;
    let content = fs::read_to_string(file_name)?;

    let monkey_data: Vec<&str> = content.split("Monkey").filter(|x| x.len() > 0).collect();

    let mut counter = 0;

    let mut monkeys: Vec<Rc<RefCell<Monkey>>> = vec![];

    let mut dividers = vec![];

    for item in monkey_data {
        let monkey = Rc::new(RefCell::new(Monkey::new(item, &mut counter)));
        println!("{:?}", monkey);
        dividers.push(monkey.borrow().divisible);
        monkeys.push(monkey);
    }

    println!("Dividers: {:?}", dividers);



    for round in 1..2 {
        println!("round {}", round);

        for index in 0..monkeys.len() {
            let mut m = monkeys.get(index).unwrap().borrow_mut();
            m.increase_inspect();


            while let Some(i) = m.items.pop() {
                println!("Item {:?}\n", i);

               let mut new_number = i;
               println!("new_number {:?}", new_number);
               new_number = m.calculate(&new_number, &mut dividers);
               //  println!("new_number {}", new_number);
               //  // new_number = new_number / 3;
               //  println!("new_number {}", new_number);

               //  let index = match new_number % m.divisible {
               //      0 => m.true_case,
               //      _ => m.false_case
               //  };

               //  println!("index {}", index);

               //  let monkey_2 = monkeys.get(index as usize).unwrap();
               //  monkey_2.borrow_mut().items.push(new_number);
            }

        }

    for i in 0..monkeys.len() {
        println!("Monkey {}", monkeys.get(i).unwrap().borrow().label);
        println!("\t items: {:?}", monkeys.get(i).unwrap().borrow().items);

    }

    }

    println!("Round finished");

    let mut arr = vec![];

    for i in 0..monkeys.len() {
        let monkey = monkeys.get(i).unwrap().borrow();
        println!("Monkey {} number: {}", monkey.label, monkey.inspect_number);
        arr.push(monkey.inspect_number);
    }

    arr.sort();

    let (a, b) = (arr[arr.len() - 1], arr[arr.len() - 2]);

    println!("res: {} {} {}", a, b, a*b);



    Ok(())
}
