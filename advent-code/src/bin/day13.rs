use std::cmp::max;
use std::rc::Rc;
use std::{fs};
use anyhow::{Result, Context};
use std::env;


#[derive(Debug)]
enum List {
    MyList(Vec<Rc<List>>),
    El(i32)
}

fn compare_list(left: Rc<List>, right: Rc<List>) -> bool {



    let left_list_reference = Rc::clone(&left);
    let left_list: &List = &*left_list_reference;

    let right_list_reference = Rc::clone(&right);
    let right_list: &List = &*right_list_reference;

    return match (*left_list, *right_list) {
        (List::MyList(v1), List::MyList(v2)) => {
            let len = max(v1.len(), v2.len());

            for i in 0..len {
                if compare_list(v1[i], v2[i]) == false {
                    return false;
                }
            }

            if v1.len() <= v2.len() {
                return true;
            }

            return false;
        },
        (List::MyList(_), List::El(_)) => {
            let new_right = right;
            let right_el = Rc::new(List::MyList(vec![new_right]));
            return compare_list(left, right_el);
        },
        (List::El(_), List::MyList(e2)) => {
            let new_left = left;
            let left_el = Rc::new(List::MyList(vec![new_left]));
            return compare_list(left_el, right);
        },
        (List::El(e1), List::El(e2)) => e1 <= e2,
    }
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

            list.push(Rc::new(item));

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

    for item in res {
        let (left, right) = item.split_once('\n').unwrap();
        println!("Left:{left}\nRight:{right}\n");
        let mut pos = 0;
        let left = create_list(left, &mut pos);
        let mut pos = 0;
        let right = create_list(right, &mut pos);
    }

    // let mut pos = 0;

    // dbg!(create_list("[]", &mut pos));

    // let mut pos = 0;
    // dbg!(create_list("[12]", &mut pos));


    // let mut pos = 0;
    // dbg!(create_list("[[12],1]", &mut pos));

    Ok(())
}
