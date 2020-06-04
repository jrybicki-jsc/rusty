struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut user1 = User {
        email: String::from("some@g.com"),
        username: String::from("somena12"),
        sign_in_count: 1,
        active: true,
    };
    user1.email = String::from("new@g.com");
    println!("Hello, world!");

    let user2 = User {
        email: String::from("ne@w.com"),
        username: String::from("oww21"),
        active: user1.active,
        sign_in_count: 2,
    };

    let user3 = User {
        email: String::from("wg@o.com"),
        ..user1
    };


    let mut white = Color(255, 255, 255);
    white.1=244;

    let rect1 = Rectangle {
        height: 30,
        width: 50,
    };

    println!("The area of rectange {:?} is {}", rect1, rect1.area());

    let rect2 = Rectangle {
        height: 10,
        width: 20
    };

    println!("Can {:?} hold {:?} {}", rect1, rect2, rect1.can_hold(&rect2));

    let sq = Rectangle::square(3);
    println!("{:?}", sq);

}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size,}
    }
}

fn get_user(email: String, username: String) -> User {
    User {
       email,
       username,
       active: true,
       sign_in_count: 1,
    }
}
