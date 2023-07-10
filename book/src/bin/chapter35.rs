fn main() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("anything"),
    }

    let x = Some(5);


    match x {
        Some(50) => println!("got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("default case"),
    }

    let x = 3;

    match x {
        1 | 2  => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;

    match x {
        1..=4 => println!("One through 5"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ascii character"),
        _ => println!("something else"),
    }
}
