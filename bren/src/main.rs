use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

const BENFORD:[f64; 9]  = [30.1, 17.6, 12.5, 9.7, 7.9, 6.7, 5.8, 5.1, 4.6];

fn get_data(fname: &str) -> HashMap::<usize, usize> {
    let co = fs::read_to_string(fname).expect(&format!("Failed to open datafile {}", fname));
    println!("Got data {}", co.len());

    let mut ret = Vec::<usize>::new();
    let mut dc = HashMap::<usize, usize>::new();
    for line in co.lines() {
        let first = line.chars().next().unwrap();
        let fna: usize = usize::from_str(&first.to_string()).unwrap();
        let mut v = dc.entry(fna).or_insert(0);
        *v+=1;
    }
    let total_count = dc.values().sum::<usize>();

    for (digit, freq) in &dc {
       println!("{} => {} ({:.2} %)", digit, freq, 100.0* (*freq as f64 / total_count as f64) );
    }
    dc
}

fn get_expected(total_count: usize)->Vec<usize> {
    let mut ben = Vec::new();
    for e in &BENFORD {
       ben.push(e);
    }
    let res:Vec<usize>  = ben.iter().map(|x| ((*x  *total_count as f64)/ 100.0) as usize).collect();
    res
}

fn main() {
    let dc = get_data("data/Illinois_votes.txt");
    let total_count = dc.values().sum::<usize>();
    let expected = get_expected(total_count);
    for (i, v) in expected.iter().enumerate() {
        println!("{} => {:.2}",i+1, v); 
        
    }
    let mut chi_stat = 0.0;
    for (i,v) in dc {
       let v2 = expected[i-1];
       println!("{} {} {}", i, v, v2);
       let chi_sq = (v as f64-v2 as f64).powf(2.0);
       chi_stat+=chi_sq as f64 / v2 as f64;
    }
    println!("Chi Squared stat {}", chi_stat);
    println!("Critical value at p=0.02 is 15.51");
}
