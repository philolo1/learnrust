
fn main() {
    // let x = 5;
    // println!("The value of x is: {}", x);
    // let x = 6;
    // println!("The value of x is: {}", x);
    // const SUBSCRIBER_COUNT = 100_000;
    //
    let f: u8 = 255;
    println!("U: {}", f);
    // f = f + 1;
    // println!("U: {}", f);

    let tup = (12, "Hi");
    println!("text: {}", tup.1);

    let (_, name) = tup;

    println!("name: {}", name);

    for a in 0..=10 {
        println!("num: {}", a);
    }
}
