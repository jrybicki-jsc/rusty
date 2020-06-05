enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y:i32},
    Write(String),
    CahngeColor(i8, i8, i8),
}

impl Message {
   fn call(&self) {
      println!("{:?}", self);
   }
}

enum Coin {
    Penny,
    Nickel, 
    Dime,
}

fn main() {
    println!("Hello, world!");
    let _home = IpAddr::V4(127,0,0,1);
    let _loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello!"));
    m.call();
    
    println!("Value of Penny is {}", value_in_cents(Coin::Penny));

    let s = 0u8;
    match s {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("patterns need to be exhausive"),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin{
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
    }
}
