use std::{fs, str::FromStr};
use anyhow::{Error, Result, Context};
use std::env;

#[derive(Debug, Clone)]
struct File {
    size: usize,
    name: String
}

impl FromStr for File {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (size, name) = s.split_once(" ").context("needs space")?;
        let size = str::parse(size)?;

        return Ok(
            File{
                size,
                name: name.to_string()
            }
        );
    }
}

#[derive(Debug, Clone)]
struct Directory {
    parent: Option<Box<Directory>>,
    current_path: String,
    directories: Vec<Directory>,
    files: Vec<File>,
    num: i32
}

impl Directory {
    fn add_files(&mut self, files: &Vec<&str>) {
        self.files = files
            .iter()
            .flat_map(|x| str::parse::<File>(x))
            .collect();
    }

    fn add_dics(&mut self, dics: &Vec<&str>) {
        let new_box = Box::new(self.clone());
        self.directories = dics
            .iter()
            .map(|x| {
                let (_, name) = x.split_once(' ').unwrap();
                return Directory {
                    parent: Some(new_box.clone()),
                    current_path: String::from(name),
                    directories: vec![],
                    files: vec![],
                    num: 0
                }
            })
            .collect();
    }
}



fn main() -> Result<()> {
    println!("Hello world");

    let mut res = Directory {
        parent: None,
        current_path: String::from(""),
        directories: vec![],
        files: vec![],
        num: 0
    };

    println!("res : {}", res.num);

    let res2 = &mut res;

    println!("res2 : {}", res2.num);
    res2.num = 3;
    println!("res2 : {}", res2.num);
    println!("res : {}", res.num);





    return Ok(());
}
