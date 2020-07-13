fn main() {
    let favorite_color: Option<&str> = Some("red");

    if let Some(color) = favorite_color {
        println!("Using favorite color {}", color);
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} at index {}", value, index);
    }

    let x = Some(5);
    let y = 120;

    match x {
        Some(50) | Some(10) => println!("it is 50"),
        Some(y) if y > 3 => println!("matched with y={}", y),
        _ => println!("Default x={:?}", x),
     }

     println!("Finnaly x= {:?}, y={}", x, y);

     let x = 'c';
     match x {
       'a'..='j' => println!("early"),
       'k'..='z' => println!("late"),
       _ => println!("non ASCI"),
     }

     let p = Point { x: 0, y:7 };
     match p {
        Point {x, y: 0} => println!("only x {}", x),
        Point {x:0, y}  => println!("only y {}", y),
        Point {x, y }   => println!("full pont ({}, {})", x, y), 
     }
     
     let Point {x:a, y:b } = p;
     println!("{}, {}", a, b);

     let msg = Msg::Write(String::from("Some message"));
     match msg {
          Msg::Quit => println!("Quitting on you"),
          Msg::Write(m) => {
              println!("Here is the message: {}", m);
          },
          _ => println!("Nothing to see here"),
     }

     let numbers = (2, 5, 3, 5, 5);
     match numbers {
          (first, .., last) => println!("From {} to {}", first, last),
     }
}

struct Point {
   x: i32,
   y: i32,
}

enum Msg {
   Quit,
   Write(String)
}
