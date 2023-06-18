#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

struct IpAddrr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x : i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message  {
    fn some_function(self) {
        println!("VAL {:?}", self);
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let localhost  = IpAddrKind::V4(127, 0, 0, 1);
    println!("Localhost: {:?}", localhost);

    let msg = Message::Move{x: 1, y: 12};
    msg.some_function();

    let some_number = Some(12);
    let some_string = Some(String::from("12"));
    let some_string_2 = Some("12");

    let absent_number: Option<i32> = None;

    let x = 5;
    let y: Option<i32> = None;

    let sum = x + y.unwrap_or(0);

    println!("some code : {}", sum);

}
