use std::{fs, str::FromStr};
use anyhow::{anyhow, Error, Result, Context};
use std::env;
use std::collections::HashMap;


fn calc_solution(content: String, num: usize) -> usize {
    for i in 0..content.len() {
        let mut map = HashMap::new();
        let mut has_found = true;
        for j in 0..num {
            let ch = content.chars().nth((i + j) as usize).unwrap();
            let item = map.get(&ch);
            if let Some(_) = item {
                has_found = false;
                break;
            }
            map.insert(ch, 1);
        }
        if has_found {
            return i + num;
        }
    }
    return 0;
}

fn main() -> Result<()> {
    let file_name = env::args().nth(1).context("one file is necessary")?;
    println!("file_name: {:?}", file_name);

    let content = fs::read_to_string(file_name)?;
    let res = calc_solution(content, 14);

    println!("res: {}", res);



    return Ok(());
}
