
use adv::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}


static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn add_one(x:i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
   f(arg) + f(arg)
}

fn get_cl() -> Box<dyn Fn(i32) -> i32> {
   Box::new(|x| x +1 )
}

fn main() {
    Pancakes::hello_macro();


    let mut num = 7;
    do_twice(add_one, 4);
    

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1={}", *r1);
        println!("r2={}", *r2);
    }

    add_to_count(3);
    unsafe {
        println!("COUNTER {}", COUNTER);
    }

    let p = Point { x: 1, y: 2 };
    let p2 = Point { x: 2, y: 1 };

    assert_eq!(p + p2, Point { x: 3, y: 3 });

    let p = Point { x: 1, y: 2 };
    assert_eq!(p + 1, Point { x: 2, y: 3 });

    let p = Point { x: 1, y: 2 };
    println!("Point is: {}", p);
}

use std::fmt;
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<i32> for Point {
    type Output = Point;

    fn add(self, other: i32) -> Point {
        Point {
            x: self.x + other,
            y: self.y + other,
        }
    }
}
