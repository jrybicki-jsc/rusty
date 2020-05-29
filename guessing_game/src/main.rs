use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);
   
    loop { 
    println!("Please input your guess.");
    let mut guess = String::new(); //by default variables are immutable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
           Ok(num) => num,
           Err(_) => {
              println!("Please enter a number");
              continue;
           },
    };

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too little..."),
        Ordering::Greater => println!("Too much!"),
        Ordering::Equal => {
               println!("You won!");
               break;
        },
    }
    }
}
