use rand::prelude::*;
use rand::Rng;
use std::fs;

fn reveal(contents: &str, lookahead: u8) -> String {
    let mut counter = 0;

    let mut ret = String::new();

    for c in contents.chars() {
        if c.is_ascii_whitespace() {
            continue;
        }

        counter += 1;

        if c.is_ascii_punctuation() {
            counter = 0;
        }
        if counter == lookahead {
            print!("{}", c.to_uppercase());
            ret.push(c);
        } else {
            print!("{}", c);
        }
    }

    ret
}

fn read_dic() -> String {
    let filename = String::from("./dictionary.txt");
    let contents = fs::read_to_string(filename).expect("Failed to open dictionary");

    contents
}

fn make_list(message: &str, pos: usize) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let dict = read_dic();
    let mut words: Vec<&str> = dict.lines().collect();
    let mut rng = rand::thread_rng();

    for c in message.chars() {
        if !c.is_alphanumeric() {
            continue;
        }
        let len = rng.gen_range(7, 12);

        rng.shuffle(&mut words);
        for word in words.iter() {
            if word.len() != len {
                continue;
            }

            if word.to_lowercase().find(c) == Some(pos) {
                ret.push(word.to_string());
                break;
            }
        }
    }

    ret
}

fn main() {
    let filename = String::from("./message.txt");
    let contents = fs::read_to_string(filename).expect("Failed to open message file");

    let m = reveal(&contents, 3);
    println!("\n{}", m);

    let msg = String::from("some message");
    let lst = make_list(&msg, 3);
    let msg = lst.join("\n");
    println!("{}", msg);
}
