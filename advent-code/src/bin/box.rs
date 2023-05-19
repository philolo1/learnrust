use std::{fs, str::FromStr};
use anyhow::{Error, Result, Context};
use std::env;
use std::rc::Rc;


#[derive(Clone, Debug)]
struct MyList {
    val: usize,
    parent: Option<Box<MyList>>,
}




fn main() -> Result<()> {

    let main = Box::new(MyList{
        val: 3,
        parent: None
    });

    let child1 = MyList {
        val: 1,
        parent: Some(main.clone())
    };

    let child2 = MyList {
        val: 2,
        parent: Some(main.clone())
    };

    let mut el = Box::new(child2);
    println!("Val {}", el.val);

    el = el.parent.unwrap();
    println!("Val {}", el.val);


    return Ok(());
}
