use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
   Cons(i32, RefCell<Rc<List>>),
   Nil,
}


impl List {
   fn tail(&self) -> Option<&RefCell<Rc<List>>> {
      match self {
         Cons(_, item) => Some(item),
         Nil => None,
      }
   }
  
}

#[derive(Debug)]
struct Node {
   value: i32,
   children: RefCell<Vec<Rc<Node>>>,
   parent: RefCell<Weak<Node>>,
}

fn main() {
    println!("Hello, world!");
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("Rc count {}", Rc::strong_count(&a));
    println!("A next item is {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("Rc count  {}", Rc::strong_count(&a));
    println!("Rc countb {}", Rc::strong_count(&b));
    println!("A next item is {:?}", b.tail());

    if let Some(link) = a.tail() {
       *link.borrow_mut() = Rc::clone(&b);
    }

    println!("Rc count  {}", Rc::strong_count(&a));
    println!("Rc countb {}", Rc::strong_count(&b));

    let leaf = Rc::new(Node{
          value: 3,
          children: RefCell::new(vec![]),
          parent: RefCell::new(Weak::new()),
     });
    println!("\nStrong: {}\nWeak: {}\n", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    println!("Leaf parent: {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
          value: 5, 
          children: RefCell::new(vec![Rc::clone(&leaf)]),
          parent: RefCell::new(Weak::new()),
     });

     *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
     println!("Leaf parent: {:?}", leaf.parent.borrow().upgrade());

     println!("\nStrong: {}\nWeak: {}\n", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
      println!("Branch\nStrong: {}\nWeak: {}\n", Rc::strong_count(&branch), Rc::weak_count(&branch));
}
