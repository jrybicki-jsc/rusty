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
}

fn my_func(x: i32, _y: f64) -> i32{
    println!("Another function. X is {}",x);

    39
}

fn plus_one(x: i32) -> i32 {

    x + 1
}
