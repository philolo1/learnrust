#[test]
fn iterator_test() {
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}



#[derive(PartialEq, Debug)]
pub struct Shoe {
    size: u32,
    style: String,
}

pub fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker")
        },
        Shoe {
            size: 13,
            style: String::from("sandal")
        },
        Shoe {
            size: 10,
            style: String::from("boot")
        },
    ];

    let res = shoes_in_my_size(shoes, 10);
    println!("Shoes: {:?}", res);

    assert_eq!(
        res,
        vec![
        Shoe {
            size: 10,
            style: String::from("sneaker")
        },
        Shoe {
            size: 10,
            style: String::from("boot")
        },
        ])


}

struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    println!("HEllo world");
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for value in v1_iter {
        println!("Got {}", value);
    }

    let v2: Vec<_>= v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2,3,4]);

    println!("OK");

    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a,b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
