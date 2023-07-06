use std::{fs, str::FromStr};
use anyhow::{anyhow, Error, Result, Context};
use std::env;
use std::collections::HashMap;


#[derive(Debug)]
struct Tree {
    value: u32,
    is_visible: bool,
}

impl  Tree {

    fn mark_visible(&mut self) {
        self.is_visible = true;
    }

    fn from_char(s: &char) -> Result<Self> {
        let value: u32 = format!("{}", s).parse()?;
        return Ok(Tree {
            value,
            is_visible: false
        })
    }
}



fn main() ->  Result<()> {
    let file_name = env::args().nth(1).context("One file is necessary")?;
    let content = fs::read_to_string(file_name)?;

    let mut tree_arr: Vec<Vec<Tree>> = content.lines().map(|x| {
        let el: Vec<char> = x.chars().collect();
        let tree : Vec<Tree> = el.iter()
            .flat_map(|x: &char| Tree::from_char(x).context("Test"))
            .collect();
        return tree;
    }).collect();



    let col_size = tree_arr.len();
    let row_size = tree_arr[0].len();

    for a in 0..col_size {
        let mut max = tree_arr[a][0].value;
        for b in 0..row_size {
            if tree_arr[a][b].value > max {
                tree_arr[a][b].mark_visible();
                max = tree_arr[a][b].value;
            }
        }
    }

    for a in 0..col_size {
        let mut max = tree_arr[a][row_size - 1].value;
        for b in (0..row_size).rev() {
            if tree_arr[a][b].value > max {
                tree_arr[a][b].mark_visible();
                max = tree_arr[a][b].value;
            }
        }
    }

    for b in 0..row_size {
        let mut max = tree_arr[0][b].value;
        for a in 0..col_size {
            if tree_arr[a][b].value > max {
                tree_arr[a][b].mark_visible();
                max = tree_arr[a][b].value;
            }
        }
    }

    for b in 0..row_size {
        let mut max = tree_arr[col_size - 1][b].value;
        for a in (0..col_size).rev() {
            if tree_arr[a][b].value > max {
                tree_arr[a][b].mark_visible();
                max = tree_arr[a][b].value;
            }
        }
    }

    for b in 0..row_size {
        tree_arr[col_size - 1][b].mark_visible();
        tree_arr[0][b].mark_visible();
    }

    for a in 0..col_size {
        tree_arr[a][0].mark_visible();
        tree_arr[a][row_size - 1].mark_visible();
    }

    let sum :u32 = tree_arr.iter().flatten().filter(|x| x.is_visible).map(|x| 1).sum();

    println!("SUM: {}", sum);

    let sum = 0;

    let mut new_max = 0;

    for y in 1..(col_size-1) {
        for x in 1..(row_size-1) {
            let mut x1 = 1;

            while x - x1  >= 1 && tree_arr[y][x].value > tree_arr[y][x-x1].value {
                x1 +=1 ;
            }

            let mut x2 = 1;

            while x +x2  + 1 < row_size && tree_arr[y][x].value > tree_arr[y][x+x2].value {
                x2 +=1 ;
            }

            let mut y1 = 1;

            while y - y1  >= 1 && tree_arr[y][x].value > tree_arr[y-y1][x].value {
                y1 +=1 ;
            }

            let mut y2 = 1;

            while y +y2  + 1 < col_size && tree_arr[y][x].value > tree_arr[y+y2][x].value {
                y2 +=1 ;
            }

            let res = x1*x2*y1*y2;

            println!("y: {}, x: {}, {},{},{},{} res: {}", x, y, x1, x2, y1, y2,  res);

            if res > new_max {
                new_max = res;
            }

        }
    }

    println!("New max: {}", new_max);


    Ok(())
}
