use std::collections::HashMap;
use std::fs;
use std::io;
use rand::Rng;
use rand::seq::SliceRandom;

use hai::{count_syllables, read_cmu};

fn clean_word(word: &str) -> String {
    let mut w = word.replace(|c: char| !c.is_alphabetic() & !c.is_whitespace(), "");
    w.make_ascii_lowercase();

    w
}

fn alt_markov() -> HashMap<(String, String), Vec<String>>{
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
         let mytu = (clean_word(&(e.1).0.to_string()), clean_word(&(e.1).1.to_string()));

         let v = di.entry(mytu).or_insert(Vec::<String>::new());
         v.push(clean_word(&e.0.to_string()));
    }
    di
}

fn gen_markov() -> HashMap<String, Vec<String>> {
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
    haikudict
}


fn gen_markov1() -> HashMap<String, Vec<String>> {
    let filename = String::from("./data/train.txt");
    let co = fs::read_to_string(filename).expect("Failed to open haiku");
    let contents = clean_word(&co);
    println!("Got coropra: {}", contents.len());

   
    
    let i1 = contents.split(" ");
    let i2 = contents.split(" ").skip(1);
    
    let zipp:Vec<_> = i1.zip(i2).collect();
    let mut hd: HashMap<String, Vec<String>> = HashMap::new();

    for (prev, word) in zipp {
        let mut v = hd.entry(clean_word(&prev.to_string())).or_insert(Vec::<String>::new());
        v.push(clean_word(&word.to_string()));
    }

   hd
    
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
    let m0 = gen_markov1();

    let mydict = read_cmu(String::from("./data/cmudict.dict"));
    let m1 = gen_markov();
    let m2 = alt_markov();

    println!("len {}", m0.keys().len()); 
    let mut rng = rand::thread_rng();
    let ind = rng.gen_range(0, m0.keys().len());
  
    let random_word = m0.keys().skip(ind).next().unwrap();
    println!("Random word: {}", random_word);
    let conts: Vec<String> = m0.get(random_word).unwrap().to_vec();
    println!("Possible continuations: {:?}", conts);

    let cont = conts.choose(&mut rng).unwrap(); 
    println!("Sentence: {} {}", random_word, cont);
   
    
}
