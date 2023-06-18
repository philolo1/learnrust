struct User {
    username: String,
    email: String,
    // sign_in_counter: u64,
    // is_active: bool
}

// struct Color(i32,i32, i32);
// struct Point(i32, i32, i32);
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height

    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    println!("Chapter 5");

    let mut user1 = User {
        email: String::from("test@gmail.com"),
        username: String::from("phil"),
        // is_active: true,
        // sign_in_counter: 1
    };

    let name = user1.username;
    println!("Name: {}", name);
    user1.username = String::from("test2");
    println!("Name: {}", name);

    let new_user = build_user(&user1.email, &user1.username);

    let user3 = User {
        email: String::from("test"),
        username: String::from("TT"),
        ..new_user
    };

    println!("Name: {}", name);
    println!("new Name: {}", new_user.username);
    println!("new Name: {}", user1.email);
    println!("new Name: {}", user3.email);

    let rect = (30, 50);
    println!("The area of the rectangle is {}", area(rect));

    let rect = Rectangle {
        width: 10,
        height: 12
    };

    println!("Rect: {:#?}", rect);

    println!("The area of the rectangle is {}", area_rect(&rect));
    println!("The area of the rectangle is {}", rect.area());

    let rect2 = Rectangle {
        width: 200,
        height: 300
    };


    println!("Can hold: {}", rect2.can_hold(&rect));
    println!("Can hold: {}", rect.can_hold(&rect2));

    let rect3 = Rectangle::square(12);

    println!("Square: {:?}", rect3);

}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_rect(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}


fn build_user(email: &String, username: &String) -> User {
    User {
        email: email.to_string(),
        username: username.to_string(),
        // is_active: true,
        // sign_in_counter: 1
    }
}
