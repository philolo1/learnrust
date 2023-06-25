#[derive(Debug)]
struct Point<T,U> {
    x: T,
    y: U
}

impl<T,U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V,W>(self, other: Point<V,W>) -> Point<T,W> {
        return Point{
            x: self.x,
            y: other.y
        };
    }
}

fn main() {
    let p1 = Point {x : 5, y: 10};
    let p2 = Point {x: 1.0, y: 12.0};
    let p3 = Point {x: 4, y: 20.0};

    println!("Point {:?}", p1);
    println!("Point {:?}", p2);
    println!("Point {:?}", p3);

    let p4 = p1.mixup(p2);
    println!("Point {:?}", p4);

}
