pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    num: i32
}

impl Guess {
    fn new(num: i32) -> Guess {
        if num < 1 || num > 100 {
            panic!("Num should be between 1 and 100");
        }
        return Guess {
            num
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn greeting_contains_name() {
        let name = "Carol";
        let result = greeting(name);
        assert!(
            result.contains(name),
            "Greeting did not contain name `{}`, value was `{result}`",
            name
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}

fn main() {
}
