fn main() {
    let mut n = String::from("Hello");

    n.push_str(", world!");
    println!("{}", n);

    let a = String::from("Hello");
    let b = a;
    println!("{}", b); //a is no longer valid, it was moved to b

    let x = 10;
    let _y = x; //stack variables are not moved, both remain valid

    let c = b.clone();
    println!("b={} c={}", b, c);

    take_owner(b);
    //    println!("b = {}", b); //b is no longer valid as it has been moved to the funciton

    let m = give_owner();
    println!("Now I own: {}", m);

    let r = switch_ow(n);
    println!("r = {}", r);

    let s = calculate_length(&r);
    println!("The length of '{}' is {}", r, s);

    let mut r = String::from("Yooo");
    extend(&mut r);
    println!("extended to {}", r);

    let mut s = String::from("Some st");
    // at any time you can have as many immutable references as you wish
    let r1 = &s;
    let r2 = &s;

    println!("Our references: {}, {}", r1, r2);

    //but only one mutable reference is allowed
    let r3 = &mut s;
    println!("Mutable reference: {}", r3);

    let my = String::from("Hello world here");
    let first = first_word(&my);
    println!("First word is {}", first);
    //my.clear(); //the compiler prevents you from keeping a reference to a slice of data are no longer valid
    println!("my is {}", my);
    println!("First word is {}", first);
}

fn first_word(a: &String) -> &str {
    let bytes = a.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &a[0..i]; //returns a slice of string from 0 to i (first word)
        }
    }
    &a[..] //no space, means the whole string was a single word
}

fn take_owner(s: String) {
    println!("The string is: {}", s);
}

fn give_owner() -> String {
    let s = String::from("here it comes");
    s
}

fn switch_ow(a: String) -> String {
    a
}

fn extend(a: &mut String) {
    a.push_str("<<");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
