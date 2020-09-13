use std::fs;
use std::str::FromStr;
use std::collections::HashMap;


fn get_data(fname: &str) -> Vec<usize> {
    let co = fs::read_to_string(fname).expect(&format!("Failed to open datafile {}", fname));
    println!("Got data {}", co.len());

    let mut ret = Vec::<usize>::new();
    let mut dc = HashMap::<char, usize>::new();
    for line in co.lines() {
        let datum: usize = usize::from_str(line).unwrap();
        ret.push(datum);
        let first = line.chars().next().unwrap();
        let mut v = dc.entry(first).or_insert(0);
        *v+=1;
    }

    for (digit, freq) in &dc {
       println!("{} => {}", digit, freq);
    }
    ret
}

fn main() {
    let v = get_data("data/Illinois_votes.txt");
}
