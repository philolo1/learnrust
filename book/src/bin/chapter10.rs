#[derive(Clone,Copy)]
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}


impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.x.partial_cmp(&other.x) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.y.partial_cmp(&other.y)
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 65];
    let largest = get_largest(&number_list);

    let point_list = vec![Point{x:122, y:1}, Point{x: 3, y: 200}, Point{x: 10, y: 10}, Point{x: 111, y: 3}];
    let largest_point = get_largest(&point_list);

    println!("The largest number is {}", largest);
    println!("The largest point is {:?}", largest_point);
}

fn get_largest<T: PartialOrd + Copy>(number_list: &Vec<T>) -> T {
    let mut largest = number_list.get(0).unwrap();

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    return *largest;
}
