

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
}
