fn main() {
    let mut x = 7;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    

    //shadowing: the variable is immutable
    let y = 20;
    let y = y+ 29;
    println!("The value of y is {}", y);

    //shadowing creates new variable (thus type can be changed):
    let stri = "bla bla bla";
    let stri = stri.len();
    println!("The length of the string is: {}", stri);


    let x: u16 = 29;

    let z: f32 = 56.3/21.;

    let flag: bool = true;
    let c = 'Z';
   

    let tup: (i32, f64, u8) = (331, 6.2, 1);
    let (a, b, c) = tup;

    println!("The value of a is {}", a);

    let b2 = tup.1;
    println!("The value of b2 is {}", b2);

    let mya = [1, 2, 3, 4, 5];
    let mya: [i32; 5] = [1, 2, 3, 4, 5];

    let fif = [3; 5]; //5 elements of 3

    let first = fif[0];
    println!("First {}", first);
    // let las = fif[10];
    
}
