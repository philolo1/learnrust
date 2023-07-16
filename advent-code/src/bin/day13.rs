use std::str::FromStr;
use std::{fs};
use anyhow::{Error, Result, Context};
use std::env;
use std::collections::VecDeque;


use std::collections::HashMap;


#[derive(Debug)]
enum List {
    MyList(Vec<Box<List>>),
    El(i32)
}

fn create_list(s: &str, pos: &mut usize) -> List {
    // dbg!(*pos);
    if *pos >= s.len() {
        return List::MyList(vec![]);
    }

    let el = s.chars().nth(*pos).unwrap();

    if el == '[' {
        let mut list = vec![];
        *pos += 1;
        loop {
            let last_pos = *pos;
            let item = create_list(s, pos);

            // no changes
            if last_pos == *pos {
                break;
            }

            list.push(Box::new(item));

            let ch =  s.chars().nth(*pos).unwrap() ;

            if ch  == ']' {
                *pos += 1;
                return List::MyList(list);
            } else if ch  == ',' {
                *pos += 1;
            } else {
                panic!("Unknown characer: {}",  ch);
            }
        }
    } else {
        let mut len: usize = 0;
        while s.chars().nth(*pos).unwrap().is_digit(10) {
            len += 1;
            *pos += 1;
        }

        if len == 0 {
            return List::MyList(vec![]);
        }

        let st = &s[(*pos - len)..(*pos)];
        let num: i32 = st.parse().unwrap();
        return List::El(num);
    }


    return List::MyList(vec![]);

}



fn main() ->  Result<()> {
    println!("Day 11");
    let file_name = env::args().nth(1).context("One file is necessary")?;
    let content = fs::read_to_string(file_name)?;

    let res: Vec<&str> = content.split("\n\n").collect();

    println!("Res: {:?}", res);

    // let mut pos = 0;

    // dbg!(create_list("[]", &mut pos));

    // let mut pos = 0;
    // dbg!(create_list("[12]", &mut pos));


    // let mut pos = 0;
    // dbg!(create_list("[[12],1]", &mut pos));

    Ok(())
}
