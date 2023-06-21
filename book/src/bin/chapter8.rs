use std::collections::HashMap;

fn main() {
    let a = [1,2,3];
    let mut v: Vec<i32> = Vec::new();

    v.push(12);

    let mut v2 = vec![1,2,8];


    println!("{:?}", a);
    println!("{:?}", v);
    println!("{:?}", v2);
    println!("{:?}", v2.get(2));

    for i in &mut v2 {
        *i *= 2;
    }

    for i in &v2 {
        println!("{}", i);
    }

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
    println!("{}", s);

    let s1 = String::from("Hello ");
    let s2 = String::from("world!");
    let s3 = format!("{}{}", s1, s2);

    println!("VAL : {}", s3);

    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();


    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    println!("val: {:?}", scores.get(&String::from("Blue")));


}
