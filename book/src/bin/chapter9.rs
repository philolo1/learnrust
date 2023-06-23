use std::fs::File;
// use std::io::ErrorKind;
use std::io::Read;
use std::io;
use std::error::Error;

/*fn main() {
    a();
}


fn a() {
    b();
}


fn b() {
    c(22);
}

fn c(num: i32) {
    if num == 22 {
        panic!("Don't pass 22!");
    }
}
*/

/*
fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error);
            }
        }
    };

    println!("{:?}", f);
}
*/

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() -> Result<(), Box<dyn Error>> {
    read_username_from_file()?;
    Ok(())
}
