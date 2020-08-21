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
    let co = fs::read_to_string(filename).expect("Failed to open haiku");
    println!("Got coropra: {}", co.len());
    let contents = clean_word(&co);

    let it1 = contents.split(" ");
    let it2 = contents.split(" ").skip(1);
    let it3 = contents.split(" ").skip(2);

    let m:Vec<_> = it3.zip(it1.zip(it2)).collect();

    let mut di: HashMap<(String, String), Vec<String>> = HashMap::new();

    for e in m {
         let mytu = (clean_word(&(e.1).0.to_string()), clean_word(&(e.1).1.to_string()));

         let v = di.entry(mytu).or_insert(Vec::<String>::new());
         v.push(clean_word(&e.0.to_string()));
    }
    di
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

fn random_word(m0: &HashMap<String, Vec<String>>, dict: &HashMap<String, usize>) -> String{
  
   let mut rng = rand::thread_rng();
   loop {
       let ind = rng.gen_range(0, m0.keys().len());
       let word = m0.keys().skip(ind).next().unwrap();
       let syls = count_syllables(&word, &dict);
       if syls <= 4 {
           return word.to_string();
       }
   }
}

fn main() {
    let m0 = gen_markov1();

    let mydict = read_cmu(String::from("./data/cmudict.dict"));
    let m1 = alt_markov();

    let mut rng = rand::thread_rng();
    let mut line = 0;
 loop { 
    let random_word = random_word(&m0, &mydict);
    let conts: Vec<String> = m0.get(&random_word).unwrap().to_vec();

    let cont = conts.choose(&mut rng).unwrap().to_string(); 

    let a = (random_word, cont);
    let m2conts:Vec<String> = m1.get(&a).unwrap().to_vec();
    let c2 = m2conts.choose(&mut rng).unwrap();
    println!(">>Sentence: {} {} {}", a.0, a.1, c2);
    

    line+=1;
    if line > 3 {
          break;
    }
   }
}
