use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::Colorize;

fn main() {
    println!("Hello world {}", 3);

    let secret_number: u32 = rand::thread_rng().gen_range(0..100);
    println!("scret number: {}", secret_number);

    println!("Guess the number!");
    loop {
        println!("Please input your guess");




        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Should have some input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };



        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "To small!".red()),
            Ordering::Equal => {
                println!("{}", "Correct".green());
                break;
            },
            Ordering::Greater => println!("{}", "To big".red()),
        }
    }
}
