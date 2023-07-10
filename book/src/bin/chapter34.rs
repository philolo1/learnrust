fn main() {
    enum Language {
        English,
        Spanish,
        Russian,
        Japanese
    }

    let language = Language::English;

    match language {
        Language::English => println!("Something "),
        Language::Japanese => println!("こんにちは"),
        _ => println!("Unknown"),
    }

    let auth_status: Option<&str> = None;
    let is_admin = false;
    let group_id : Result<u8, _> = "34".parse();

    if let Some(status) = auth_status {
        println!("Something {status}");
    } else if is_admin {
        println!("Some test");
    } else if let Ok(group_id) = group_id {
        println!("group id");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);


    while let Some(top) = stack.pop() {
        println!("TOP: {}", top);
    }
}
