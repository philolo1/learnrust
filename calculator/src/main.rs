use std::env::{args,Args};

fn get_number(args: &mut Args) -> i32 {
    return args.next().unwrap().parse::<i32>().unwrap()
}

fn calculate(num: i32, op: char, num2: i32) -> i32 {
    match op {
        '+' => return num + num2,
        '-' => return num - num2,
        '*' => return num * num2,
        '/' => return num / num2,
        _ => return 0,
    }
}

fn main() {
    let mut args = args();

    if args.len() != 4 {
        panic!("3 parameters are required");
    }

    // discard first arg
    args.next();

    let num1: i32 = get_number(&mut args);
    let op: char = args.next().unwrap().chars().next().unwrap();
    let num2 = get_number(&mut args);

    println!("{:?}", args);
    println!("RES {} {} {} = {}", num1, op, num2, calculate(num1, op, num2));
    println!("Hello, world!");
}
