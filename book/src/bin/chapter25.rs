use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Counter after create a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("Counter after create b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("Counter after create c = {}", Rc::strong_count(&a));
    }
    println!("Counter after create c goes out of scope = {}", Rc::strong_count(&a));
}
