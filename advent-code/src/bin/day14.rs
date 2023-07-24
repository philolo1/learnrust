use std::cmp::{min, max};
use std::{fs};
use anyhow::{Result, Context};
use std::env;


use std::collections::HashMap;

type Pair<'a> = (&'a str, &'a str);

fn main() ->  Result<()> {
    println!("Day 11");
    let file_name = env::args().nth(1).context("One file is necessary")?;
    let content = fs::read_to_string(file_name)?;

    let res: Vec<&str> = content.split("\n").filter(|x| x.len() > 0).collect();

    let mut my_map = HashMap::new();

    for line in res {
        let arr: Vec<Pair> = line.split(" -> ").flat_map(|x| x.split_once(',')).collect();

        let arr: Vec<(i32, i32)> = arr.into_iter().map(|(a,b)| (a.parse().unwrap(), b.parse().unwrap())).collect();
        dbg!(&arr);

        for i in 0..(arr.len()-1) {
            let (x1, y1) = arr[i];
            let (x2, y2) = arr[i+1];

            if x1 == x2 {
                let mut y = min(y1, y2);
                while y <= max(y1, y2) {
                    my_map.insert((x1, y), true);
                    y += 1;
                }
            } else if y1 == y2 {
                let mut x = min(x1, x2);
                while x <= max(x1, x2) {
                    my_map.insert((x, y1), true);
                    x += 1;
                }
            } else {
                panic!("Not a straight line");
            }
        }

    //    dbg!(&my_map.len());
    }

    let mut counter = 0;
    // simulate falling
    while simulate_fall(&mut my_map) == true {
        counter += 1;
    }

    dbg!(counter);



    Ok(())
}

fn simulate_fall(my_map: &mut HashMap<(i32, i32), bool>) -> bool  {
    let (mut x, mut y) = (500, 0);

    let mut index = 0;

    loop {
        if let None = my_map.get(&(x,y+1)) {
            y+=1;
        } else if None == my_map.get(&(x-1,y+1)) {
            x -= 1;
            y += 1;
        } else if None == my_map.get(&(x+1,y+1))  {
            x += 1;
            y += 1;
        } else {
            break;
        }

        index+= 1;

        if index > 1000 {
            break;
        }
    }

    if index <= 1000 {
        dbg!(&index);
        dbg!((&x,&y));
        my_map.insert((x,y), true);
        return true;
    } else {
        dbg!(&index);
        return false;
    }
}
