fn main() {
    let data = "initial contents";
    let s = data.to_string();

    let mut s = String::from("foo");
    s.push_str("bar");

    println!("Hello, world: {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("concat: {}", s3);
    //println!("Not valid anymore: {}", s1);

    let ss = String::from("नमस्ते");

    for c in ss.chars() {
        println!("{}", c);
    }
    for b in ss.bytes() {
        println!("{}", b);
    } 
}
