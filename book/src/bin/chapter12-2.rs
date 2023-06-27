use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
where
    T: Display
{
    println!("Annoucements {}", ann);
    if x.len() < y.len() {
        return x;
    }
    return y;
}

fn main() {
    longest_with_an_announcement(
        String::from("pato").as_str(),
        String::from("papapa").as_str(),
        "Hi"
    );
}
