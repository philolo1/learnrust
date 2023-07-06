use std::{fs, str::FromStr, rc::{Weak, Rc}, cell::RefCell};
use anyhow::{Error, Result, Context};
use std::env;

#[derive(Debug, Clone)]
struct File {
    size: u32,
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
    parent: Weak<RefCell<Directory>>,
    current_path: String,
    directories: RefCell<Vec<Rc<RefCell<Directory>>>>,
    files: Vec<File>
}

impl Directory {

    fn print(&self, level: Option<usize>) {
        let level = level.unwrap_or(0);
        let space = " ".repeat(level);
        println!("{space}{}", self.current_path);
    }


    // calculates the size of the dictionary
    fn calculate_sum(&self) -> u32 {

        let sum_files: u32 = self.files.iter()
            .map(|file| file.size).sum();

        let sum_dics: u32 = self.directories.borrow().iter()
            .map(
                |dic| dic.borrow().calculate_sum()
            )
            .sum();


        return sum_files + sum_dics;
    }


    fn add_files(&mut self, files: &Vec<&str>) {
        self.files = files
            .iter()
            .flat_map(|x| str::parse::<File>(x))
            .collect();
    }

    fn add_dics(&mut self, dics: &Vec<&str>, parent: &Rc<RefCell<Directory>>) {
        self.directories = RefCell::new(dics
            .iter()
            .map(|x| {
                let (_, name) = x.split_once(' ').unwrap();
                return Rc::new(RefCell::new(Directory {
                    parent: Rc::downgrade(parent),
                    current_path: String::from(name),
                    directories: RefCell::new(vec![]),
                    files: vec![]
                }))
            })
            .collect()
        );
    }
}



fn main() -> Result<()> {
    let content = get_content()?;

    let main_dir = Rc::new(RefCell::new(Directory{
        parent: Weak::new(),
        current_path: String::from("/"),
        directories: RefCell::new(vec![]),
        files: vec![]
    }));

   let commands: Vec<&str> = content.split("$")
     .filter(|x| x.len() > 0 && !x.contains("// "))
    .map(|x| x.trim()).collect();

   println!("Commands: {:?}", commands);

   let mut current_dic = Rc::clone(&main_dir);


    for command in commands {
        println!("\n\nPROCESS COMMAND \n \n");
        let split_opt = command.split_once("\n");

        if let None = split_opt {
            println!("cd {}", command);
            let cmd = command[2..].trim().to_string();
            if cmd == "/" {
                current_dic = Rc::clone(&main_dir);
                println!("This works!");
            } else if cmd == ".." {
                let res = current_dic.borrow().parent.upgrade().unwrap();
                current_dic = Rc::clone(&res);
                println!("Cloned")
            } else {

                println!("HERE {}", cmd);

                let mut my_res = None;
                {
                    let dict_mut = current_dic.borrow_mut();
                    let dictionaries = dict_mut.directories.borrow();


                    for dic in dictionaries.iter()  {
                        println!("Hello world");

                        if dic.borrow().current_path == cmd {
                            println!("path found");
                            // TODO this might make problems
                            my_res = Some(Rc::clone(dic));
                            
                            break;
                        }
                    }
                }

                current_dic = my_res.unwrap();

            }
        }
    

        if let Some((a,b)) = split_opt {
            println!("LS {}\n {}", a, b);
            let dics: Vec<&str> = b.lines().filter(|x| x.contains("dir")).collect();

            println!("Dics {:?}", dics);

            current_dic.borrow_mut().add_dics(&dics, &current_dic);


            let files: Vec<&str> = b.lines().filter(|x| !x.contains("dir")).collect();
            current_dic.borrow_mut().add_files(&files);

            println!("Files {:?}", files);


            println!("current_dic {:?}", current_dic);

        }
    }


    // println!("current dicts {:?}", current_dic);
    // println!("main dicts {:?}", main_dir);

    println!("Sum {}", main_dir.borrow().calculate_sum());


    

    main_dir.borrow().print(None);



    return Ok(());
}

fn get_content() -> Result<String, Error> {
    let file_name = env::args().nth(1).context("one file is necessary")?;
    println!("file_name: {:?}", file_name);
    let content = fs::read_to_string(file_name)?;
    Ok(content)
}
