use rand::Rng;
use std::io;

fn main() {
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
