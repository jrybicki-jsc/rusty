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
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
   fn new(x: T) -> MyBox<T> {
      MyBox(x)
   }

}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }

}
