use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = String::from("./data/cmudict.dict");
    let contents = fs::read_to_string(filename).expect("Failed to open dictionary");
    println!("Got dict: {}", contents.len());
    let re = Regex::new(r"\w\(\d\)").unwrap();

    let mut mydict = HashMap::new();

    for word in contents.lines() {
        let split = word.split(" ").collect::<Vec<&str>>();
        if re.is_match(&split[0]) {
            //            println!("Variant detected {}... skipping!", split[0]);
            continue;
        }
        mydict.insert(split[0], split.len() - 1);
    }

    println!("Dict created {}", mydict.len());

    let filename = String::from("./data/train.txt");
    let contents = fs::read_to_string(filename).expect("Failed to open haiku");
    println!("Got coropra: {}", contents.len());

    let myword = String::from("someword,");
    let w = myword.replace(|c: char| !c.is_alphabetic(), "");
    let mut haikudict = Vec::new();

    for word in contents.split(" ") {
        let mut w = word.replace(|c: char| !c.is_alphabetic(), "");
        w.make_ascii_lowercase();
        if w.is_empty() {
            continue;
        }
        if !mydict.contains_key(&w.as_str()) {
            //println!("This word is missing in the dictionary {}", w);
            haikudict.push(w);
        }
    }

    println!("Haiku dict: {}", haikudict.len());
}
