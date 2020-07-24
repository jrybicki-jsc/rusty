use rand::Rng;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io;

fn pmbc() -> Result<(), Box<dyn Error>> {
    let filename = String::from("./f.txt");
    let contents = fs::read_to_string(filename)?;

    let mut letters = HashMap::new();

    for c in contents.chars() {
        if c.is_ascii_alphabetic() {
            let count = letters.entry(c).or_insert(0);
            *count += 1;
        }
    }

    println!("{:?}", letters);

    for (c, &number) in letters.iter() {
        print!("{}:", c);
        for _ in 0..number {
            print!("{}", c);
        }
        print!("\n");
    }

    Ok(())
}

fn main() {
    pmbc();

    return;

    let first = vec!["Ann", "Bell", "John", "Worn", "Bill"];
    let last = vec!["Crab", "Milk", "Onerg", "Zonk", "Smack"];

    let mut rng = rand::thread_rng();

    loop {
        let findex = rng.gen_range(0, first.len());
        let lindex = rng.gen_range(0, last.len());
        println!("Name: {} {}", first[findex], last[lindex]);

        println!("\n[Q]uit?");
        let mut g = String::new(); //by default variables are immutable
        io::stdin().read_line(&mut g).expect("Failed to read line");

        if g.starts_with('Q') {
            break;
        }
    }
}
