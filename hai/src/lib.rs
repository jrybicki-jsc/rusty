use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub fn count_syllables(sentence: &String, dictionary: &HashMap<String, usize>) -> usize {
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

pub fn read_cmu(filename: String) -> HashMap<String, usize> {
    //let filename = String::from("./data/cmudict.dict");
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
