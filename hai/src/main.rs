use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let filename = String::from("./data/cmudict.dict");
    let contents = fs::read_to_string(filename).expect("Failed to open dictionary");
    println!("Got dict: {}", contents.len());
    let re = Regex::new(r"\w\(\d\)").unwrap();

    let mut mydict = HashMap::new();

    for word in contents.lines() {
        let split = word.split(" ").collect::<Vec<&str>>();
        if re.is_match(&split[0]) {
           println!("Variant detected {}... skipping!", split[0]);
           continue;
        }
        mydict.insert(split[0], split.len()-1);
    } 
   
   println!("Dict created {}", mydict.len()); 
}
