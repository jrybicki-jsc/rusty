use std::collections::HashMap;
use std::fs;
use std::io;

use hai::{count_syllables, read_cmu};

fn clean_word(word: &str) -> String {
    let mut w = word.replace(|c: char| !c.is_alphabetic(), "");
    w.make_ascii_lowercase();

    w
}

fn gen_markov() {
    let filename = String::from("./data/train.txt");
    let contents = fs::read_to_string(filename).expect("Failed to open haiku");
    println!("Got coropra: {}", contents.len());

    let mut haikudict: HashMap<String, Vec<String>> = HashMap::new(); //Vec::new();
    let mut prev = String::from("");

    for word in contents.split(" ") {
        let w = clean_word(word);
        if w.is_empty() {
            continue;
        }

        if prev.is_empty() {
            prev = w.clone();
            continue;
        }

        let mut v = haikudict
            .entry(prev.clone())
            .or_insert(Vec::<String>::new());
        prev = w.clone();
        v.push(w);
    }

    println!("Haiku dict: {}", haikudict.len());
}

fn reader(mydict: HashMap<String, usize>) {
    loop {
        let mut sen = String::new();
        io::stdin()
            .read_line(&mut sen)
            .expect("Failed to read line");

        let s = count_syllables(&sen, &mydict);
        println!(">syllables {}", s);
    }
}

fn main() {
    let mydict = read_cmu(String::from("./data/cmudict.dict"));
    gen_markov();
}
