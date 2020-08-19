use std::collections::HashMap;
use std::fs;
use std::io;

use hai::{count_syllables, read_cmu};

fn clean_word(word: &str) -> String {
    let mut w = word.replace(|c: char| !c.is_alphabetic(), "");
    w.make_ascii_lowercase();

    w
}

fn alt_markov() {
    let filename = String::from("./data/train.txt");
    let contents = fs::read_to_string(filename).expect("Failed to open haiku");
    println!("Got coropra: {}", contents.len());

    let it1 = contents.split(" ");
    let it2 = contents.split(" ").skip(1);
    let it3 = contents.split(" ").skip(2);

    let m:Vec<_> = it3.zip(it1.zip(it2)).collect();

    println!("here:\n{:?}->{}", m[0].1, m[0].0);
    println!("here:\n{:?}->{}", m[1].1, m[1].0);

    let mut di: HashMap<(String, String), Vec<String>> = HashMap::new();

    for e in m {
         let mytu = ((e.1).0.to_string(), (e.1).1.to_string());

         let v = di.entry(mytu).or_insert(Vec::<String>::new());
         v.push(e.0.to_string());
    }

    println!("Markov created");
    let mut c = 0;
    for (k, v) in di {
       println!("({},{}) -> {:?}", k.0, k.1, v);
       c+=1;
       if c == 10 {
         break;
       }
    }

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
    //gen_markov();
    alt_markov();
}
