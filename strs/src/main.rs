use std::collections::HashMap;

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


    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 30);

    let team_name = String::from("Black");
    scores.insert(team_name, 20);
    //println!("Invalid team name: {}", team_name);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
         Some(x) => println!("{} score is: {}", team_name, x),
         None => println!("There is no {} team", team_name),
    }

    for (key, value) in &scores {
         println!("{}: {}", key, value);
    }

    scores.insert(String::from("Blue"), 20);
    println!("{:?}", scores);

    scores.entry(String::from("Orange")).or_insert(59);
    scores.entry(String::from("Blue")).or_insert(21);
    println!("{:?}", scores);

    let count = scores.entry(String::from("Blue")).or_insert(0);
    *count+=21;
    println!("{:?}", scores);
    
}
