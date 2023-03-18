
use std::env::{args, Args};

struct Input {
    number1: i32,
    operator: char,
    number2: i32,
}

impl Input {
    fn calculate(&self) -> Option<i32> {
        match self.operator {
            '+' => Some(self.number1 + self.number2),
            '-' => Some(self.number1 - self.number2),
            '*' => Some(self.number1 * self.number2),
            '/' => if self.number2 != 0 {
                Some(self.number1 / self.number2)
            } else {
                None
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus() {
        let input = Input { number1: 2, operator: '+', number2: 3 };
        assert_eq!(input.calculate(), Some(5));
    }

    #[test]
    fn test_minus() {
        let input = Input { number1: 2, operator: '-', number2: 3 };
        assert_eq!(input.calculate(), Some(-1));
    }

    #[test]
    fn test_multiply() {
        let input = Input { number1: 2, operator: '*', number2: 3 };
        assert_eq!(input.calculate(), Some(6));
    }

    #[test]
    fn test_divide() {
        let input = Input { number1: 6, operator: '/', number2: 3 };
        assert_eq!(input.calculate(), Some(2));
    }

    #[test]
    fn test_divide_by_zero() {
        let input = Input { number1: 6, operator: '/', number2: 0 };
        assert_eq!(input.calculate(), None);
    }

    #[test]
    fn test_unsupported_operator() {
        let input = Input { number1: 2, operator: '%', number2: 3 };
        assert_eq!(input.calculate(), None);
    }

   #[test]
    fn test_valid_operator() {
        let operator = '+';
        assert!(is_valid_operator(operator));
    }

    #[test]
    fn test_invalid_operator() {
        let operator = '%';
        assert!(!is_valid_operator(operator));
    }
}

fn get_number(args: &mut Args) -> Option<i32> {
    args.next()?.parse::<i32>().ok()
}


fn is_valid_operator(op: char) -> bool {
    matches!(op, '+' | '-' | '*' | '/')
}

fn get_operator(args: &mut Args) -> Option<char> {
    let op = args.next()?.chars().next()?;
    if is_valid_operator(op) {
        Some(op)
    } else {
        None
    }
}

fn main() {
    let mut args = args();

    if args.len() != 4 {
        eprintln!("3 parameters are required: number1 operator number2");
        std::process::exit(1);
    }

    // discard first arg
    args.next();

    let num1 = get_number(&mut args).expect("Failed to parse first number");
    let op = get_operator(&mut args).expect("Invalid operator");
    let num2 = get_number(&mut args).expect("Failed to parse second number");

    let input = Input {
        number1: num1,
        operator: op,
        number2: num2,
    };

    match input.calculate() {
        Some(result) => println!("{} {} {} = {}", input.number1, input.operator, input.number2, result),
        None => eprintln!("Error: Unsupported operator or invalid operation"),
    }
}

