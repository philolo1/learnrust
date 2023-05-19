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

#[derive(Debug)]
struct Directory {
    parent: Option<Box<Directory>>,
    current_path: String,
    directories: Vec<Directory>,
    files: Vec<File>
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
                    files: vec![]
                }
            })
            .collect();
    }
}



fn main() -> Result<()> {
    let file_name = env::args().nth(1).context("one file is necessary")?;
    println!("file_name: {:?}", file_name);

    let content = fs::read_to_string(file_name)?;

    let mut main_dir = Directory{
        parent: None,
        current_path: String::from("/"),
        directories: vec![],
        files: vec![]
    };

    let commands: Vec<&str> = content.split("$").filter(|x| x.len() > 0).map(|x| x.trim()).collect();

    println!("Commands: {:?}", commands);

    let mut current_dic = &mut main_dir;

    for command in commands {
        let split_opt = command.split_once("\n");

        if let None = split_opt {
            println!("cd {}", command);
            let cmd = command[2..].trim().to_string();
            if cmd == "/" {
                current_dic = &mut main_dir
            } else if cmd == ".." {
                current_dic = &mut current_dic.parent.unwrap()
            } /* else {
                for dic in &current_dic.directories {
                    println!("Hello world");
                    if dic.current_path == cmd {
                        println!("path found");
                        // TODO this might make problems
                        current_dic = &mut dic;
                        break;
                    }
                }
            }
              */
        }

        if let Some((a,b)) = split_opt {
            println!("LS {}\n {}", a, b);
            let dics: Vec<&str> = b.lines().filter(|x| x.contains("dir")).collect();

            println!("Dics {:?}", dics);

            current_dic.add_dics(&dics);
            println!("main_dics {:?}", current_dic);
            println!("main_dics {:?}", main_dir);

            todo!("Here");

            // println!("main_dics {:?}", current_dic);
            // println!("DIR {:?}", dics);

            let files: Vec<&str> = b.lines().filter(|x| !x.contains("dir")).collect();

            current_dic.add_files(&files);

            // println!("Files {:?}", files);


            // println!("current_dic {:?}", current_dic);

        }
    }


    println!("main_dics {:?}", current_dic);


    return Ok(());
}
