use std::collections::HashMap;
use std::fs;
use std::io;

use hai::{read_cmu, count_syllables};

fn check_missing(mydict: HashMap<String, usize>) {
    let filename = String::from("./data/train.txt");
    let contents = fs::read_to_string(filename).expect("Failed to open haiku");
    println!("Got coropra: {}", contents.len());

    let mut haikudict = Vec::new();

    for word in contents.split(" ") {
        let mut w = word.replace(|c: char| !c.is_alphabetic(), "");
        w.make_ascii_lowercase();
        if w.is_empty() {
            continue;
        }
        if !mydict.contains_key(&w) {
            //println!("This word is missing in the dictionary {}", w);
            haikudict.push(w);
        }
    }

    println!("Haiku dict: {}", haikudict.len());
}

fn main() {
    let mydict = read_cmu(String::from("./data/cmudict.dict"));

    loop {
        let mut sen = String::new();
        io::stdin()
            .read_line(&mut sen)
            .expect("Failed to read line");

        let s = count_syllables(&sen, &mydict);
        println!(">syllables {}", s);
    }
}
