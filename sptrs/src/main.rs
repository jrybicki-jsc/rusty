use crate::List::{Cons, Nil};

enum List {
   Cons(i32, Box<List>),
   Nil,
}


fn main() {
    let b = Box::new(5); // store i32 on the heap
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    //dref
    let x = 5;
    let y = &x;
    
    assert_eq!(5, x);
    assert_eq!(5, *y);
    //assert_eq!(5, y);

    let y = Box::new(x);
    assert_eq!(5, *y);  //equals despite being a reference to a box

    let y = MyBox::new(x);
    assert_eq!(5, *y);

    let me = MyBox::new(String::from("Rust"));
    hello(&me);

    let c = CustomSmartPointer{
        data: String::from("my stuff"),
    };

    println!("Custom smp. ptr created!");
}

fn hello(name: &str) {
   println!("Hello, {}!", name);
}

struct CustomSmartPointer {
   data: String,
}

impl Drop for CustomSmartPointer {
   fn drop(&mut self) {
      println!("Dropping custom sm. pointer with data {}!", self.data);
   }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
   fn new(x: T) -> MyBox<T> {
      MyBox(x)
   }

}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) { 
        println!("Dropping mybox...");
    }
} 

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }

}
