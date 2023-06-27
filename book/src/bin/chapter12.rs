/*fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }

    println!("r : {}", r);
}
*/


fn main() {
    let text = String::from("test");
    let mut text: &str = text.as_str();
    {
        let s = "This will live forever";
        text = s;
    }
    println!("Text: {}", text);
    /*
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
    */
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }
    y
}

fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
