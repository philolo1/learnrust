use std::{fs};
use anyhow::{Result, Context};
use std::env;
use std::collections::VecDeque;


use std::collections::HashMap;

type Pos = (i32, i32);


#[derive(Debug)]
struct Map<'a> {
    arr: &'a Vec<&'a str>,
    visited: HashMap<Pos, bool>,
    y_size: i32,
    x_size: i32,
}

impl<'a> Map<'a> {

    fn new(arr: &'a Vec<&str>, y_size: i32, x_size: i32) -> Map<'a> {
        Map {
            arr,
            visited: HashMap::new(),
            y_size,
            x_size,
        }
    }
    fn calculate_height(&self, y: usize, x:usize) -> usize {
        let ch = self.arr[y].chars().nth(x).unwrap();

        let res = match ch {
            'S' => 0,
            'a'..='z' => ch as u8 - 'a' as u8,
            'E' => 'z' as u8 - 'a' as u8,
            a => panic!("invalid character '{}'", a),
        };

        return res as usize;

    }

    fn search_pos(&mut self, next_arr: &mut VecDeque<(Pos, usize)>) -> Option<usize> {

    while let Some(el)  = next_arr.pop_front() {
        let len = el.1;
        let current_pos = el.0;
        println!("VISIT {:?} {}", current_pos, len);

        let (y1, x1) = current_pos;

        if x1 < 0 || x1 >= self.x_size || y1 < 0 || y1 >= self.y_size {
            continue;
        }

        if self.visited.contains_key(&current_pos) {
            continue;
        }


        self.visited.insert(current_pos, true);

        let ch = self.arr[y1 as usize].chars().nth(x1 as usize).unwrap();

        println!("CHAR: {}", ch);

        if ch == 'E' {
            return Some(len);
        }

        for (y2, x2) in DIR {
            let (y, x) = ( y1 + y2, x1 + x2);

            println!("POS: {} {}", y, x);


            if self.visited.contains_key(&(y, x)) {
                println!("visited");
                continue;
            }

            if x < 0 || x >= self.x_size || y < 0 || y >= self.y_size {
                continue;
            }

            let height_1 = self.calculate_height(y1 as usize, x1 as usize);

            let height_2 = self.calculate_height(y as usize, x as usize);

            println!("height: {} {}", height_1, height_2);

            if height_2 <= height_1 + 1 {
                println!("PUSH");
                next_arr.push_back(((y,x), len + 1));
            }
        }
    }



    return None;
}
}


const DIR: [Pos; 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];




fn main() ->  Result<()> {
    println!("Day 11");
    let file_name = env::args().nth(1).context("One file is necessary")?;
    let content = fs::read_to_string(file_name)?;

    let y_size = content.lines().count();
    let x_size = content.lines().nth(0).unwrap().len();

    println!("Size: {} {}", y_size, x_size );

    let arr: Vec<&str> = content.lines().collect();

    let mut start_pos: Pos = (0,0);
    let mut end_pos: Pos = (0,0);

    for y in 0..y_size {
        for x in 0..x_size {
            let el = arr[y].chars().nth(x).unwrap();
            match el {
                'S' => { start_pos = (y as i32 ,x as i32)}
                'E' => { end_pos = (y as i32, x as i32)}
                _ =>  {/* do nothing */}
            }
        }
    }

    println!("pos: {:?} {:?}", start_pos, end_pos);


    let mut map = Map::new(&arr, y_size as i32, x_size as i32);

    // println!("Map: {:?}", map);
    let mut search_arr =  VecDeque::new();

    search_arr.push_back((start_pos, 0));

    let r = map.search_pos(&mut search_arr);


    println!("r: {:?}", r);

    /*
    let mut v = VecDeque::new();

    for i in 0..10 {
        v.push_back(i);
    }

    for _i in 0..10 {
        println!("{:?}", v.pop_front());
    }
    */


    println!("Size: y: {} x: {}", y_size, x_size );



    Ok(())
}
