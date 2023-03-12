use std::io;

fn main() {
    println!("Hello what is your name?");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Hello {}", input)
        },
        Err(e) => {
            println!("Error {}", e)
        }
    }
}
