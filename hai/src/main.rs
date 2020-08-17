use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io;

fn count_syllables(sentence: &String, dictionary: &HashMap<String, usize>) -> usize {
    let mut ret: usize = 0;

    for word in sentence.split(" ") {
        let mut w = word.replace(|c: char| !c.is_alphabetic(), "");
        w.make_ascii_lowercase();

        match dictionary.get(&w) {
            Some(&syl) => {
                ret += syl;
            }
            _ => {
                println!("Could not find {} in the dict", word);
            }
        };
    }

    ret
}

fn read_cmu() -> HashMap<String, usize> {
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
        mydict.insert(split[0].to_string(), split.len() - 1);
    }

    println!("Dict created {}", mydict.len());
    mydict
}

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
    let mydict = read_cmu();

    loop {
        let mut sen = String::new();
        io::stdin()
            .read_line(&mut sen)
            .expect("Failed to read line");

        

        let s = count_syllables(&sen, &mydict);
        println!(">syllables {}", s);
   }
}
