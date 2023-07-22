use std::cell::RefCell;
use std::cmp::min;
use std::rc::Rc;
use std::{fs};
use anyhow::{Result, Context};
use std::env;
use std::cmp::Ordering;

#[derive(Debug)]
enum List {
    MyList(Vec<Rc<RefCell<List>>>),
    El(i32)
}

fn compare_list(left: &Rc<RefCell<List>>, right: &Rc<RefCell<List>>) -> Ordering {

    let left_el = left.borrow();
    let left_el = &*left_el;
    let right_el = right.borrow();
    let right_el = &*right_el;

    match(left_el, right_el) {
        (List::MyList(v1), List::MyList(v2)) => {
            println!("COMPARE L {} {}", v1.len(), v2.len());
            let len = min(v1.len(), v2.len());

            for i in 0..len {
                let w1 = &v1[i];
                let w2 = &v2[i];
                match compare_list(w1, w2) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => {
                        continue;
                    },
                    Ordering::Greater => return Ordering::Greater,
                }
            }

            println!("HERE? {} {}", v1.len(), v2.len());
            if v1.len() < v2.len() {
                return Ordering::Less;
            }


            if v1.len() == v2.len() {
                return Ordering::Equal;
            }

            return Ordering::Greater;
        },
        (List::El(e1), List::El(e2)) => {
            println!("Compare {e1} to {e2} {:?}",  e1.cmp(&e2));
            return e1.cmp(&e2)
        },
        (List::MyList(_), List::El(_)) => {
            let arr = vec![right.clone()];
            let new_right = &Rc::new(RefCell::new(List::MyList(arr)));
            return compare_list(left, new_right);
        },
        (List::El(_), List::MyList(_)) => {
            let arr = vec![left.clone()];
            let new_left = &Rc::new(RefCell::new(List::MyList(arr)));
            return compare_list(new_left, right);
        }
    }
}

fn create_list(s: &str, pos: &mut usize) -> List {
    if *pos >= s.len() {
        return List::MyList(vec![]);
    }

    let el = s.chars().nth(*pos).unwrap();

    println!("el {}", el);

    if el == '[' {
        let mut list = vec![];
        *pos += 1;
        loop {
            let last_pos = *pos;
            println!("pos before: {pos}");
            let item = create_list(s, pos);
            println!("pos after: {pos}");
            // no changes
            if last_pos == *pos {
                break;
            }

            list.push(Rc::new(RefCell::new(item)));

            let ch =  s.chars().nth(*pos).unwrap() ;
            println!("ch {ch} pos: {pos}");

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

    let res: Vec<&str> = content.split("\n\n").nth(68).into_iter().collect();

    println!("Res: {:?}", res);

    let mut index = 0;
    let mut sum = 0;
    let mut arr: Vec<i32> = vec![];
    for item in res {
        index+=1;
        let (left, right) = item.split_once('\n').unwrap();
        println!("Index:{index}\nLeft:{left}\nRight:{right}\n");
        let mut pos = 0;
        let left = create_list(left, &mut pos);
        println!("Left: {:?}", left);
        todo!("TEST");
        let mut pos = 0;
        let right = create_list(right, &mut pos);

        println!("Right: {:?}", right);

        match compare_list(&Rc::new(RefCell::new(left)), &Rc::new(RefCell::new(right))) {
            Ordering::Less => {
                println!("TRUE");
                arr.push(index);
                sum += index;
            }
            Ordering::Greater|Ordering::Equal => {
                println!("FALSE");
            },
        }

    }

    println!("arr: {:?}", arr);
    println!("Index: {index} SUM: {}", sum);

    // let mut pos = 0;

    // dbg!(create_list("[]", &mut pos));

    // let mut pos = 0;
    // dbg!(create_list("[12]", &mut pos));


    // let mut pos = 0;
    // dbg!(create_list("[[12],1]", &mut pos));

    Ok(())
}
