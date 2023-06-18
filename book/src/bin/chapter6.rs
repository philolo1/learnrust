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

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}



fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Quarter(state) => {
            println!("The state is {:?}", state);
            25
        },
        _ => 0
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

    value_in_cents(Coin::Quarter(UsState::California));

    println!("Test: {:?}", plus_one(Some(4)));
    println!("Test: {:?}", plus_one(None));
}

fn plus_one(x: Option<i32>) -> Option<i32> {

    if let Some(3) = x {
        println!("3");
    }

    match x {
        Some(i)  => Some(i +1),
        _ => None,
    }
}
