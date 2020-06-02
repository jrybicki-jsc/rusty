fn main() {
    println!("Hello, world!");
    let _x = my_func(2029, 1.);

    let y = {
        let x = 3;
        x + 1 //no semicolon to differentiate between stateements and expression
    };

    let x = my_func(y, 0.2);

    let _x = my_func(x, 0.77);

    let _y = plus_one(299);

    let s = 20;

    let y = plus_one(s);
    println!("the values are: s={}, y={}", s, y);

    if y > 20 {
        println!("y is bigger than 20!");
    } else {
        println!("y is smaller...");
    }

    if y % 4 == 0 {
        println!("y is divisible by 4");
    } else if y % 3 == 0 {
        println!("y is disible by 3");
    } else {
        println!("y is not divisible by 4 nor by 3");
    }

    let condition: bool = true;
    let nmbr = if condition { 5 } else { 7 };
    println!("nmbr = {}", nmbr);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!(
        "Result of looping assigment: {} whilst the counter: {}",
        result, counter
    );

    while counter != 5 {
        println!("{}!", counter);
        counter -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The a[{}]={}", index, a[index]);
        index += 1;
    }
    for element in a.iter() {
        println!("{}", element);
    }
}

fn my_func(x: i32, _y: f64) -> i32 {
    println!("Another function. X is {}", x);

    39
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
