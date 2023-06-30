
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

use List::{Cons, Nil};


fn main() {
    let b = Box::new(5);

    let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("b = {}", b);
    println!("l = {:?}", l);
}
