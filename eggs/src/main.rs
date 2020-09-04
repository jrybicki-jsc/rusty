use std::fs;
//use std::io;
//use rand::Rng;
use std::str::FromStr;


fn get_data(fname: &str) -> Vec::<f64>{
    let co = fs::read_to_string(fname).expect(&format!("Failed to open datafile {}", fname));
    println!("Got data {}", co.len());

    let mut ret = Vec::<f64>::new();
    for line in co.lines() {
        let datum:f64 = f64::from_str(line).unwrap();
        ret.push(datum);
    }

    ret
}

fn main() {
       let bonds = get_data("data/10-yr_TBond_returns_1926-2013_pct.txt");
       let stocks = get_data("data/SP500_returns_1926-2013_pct.txt");
       let  blend_40_50_10 = get_data("data/S-B-C_blend_1926-2013_pct.txt");
       let blend_50_50 = get_data("data/S-B_blend_1926-2013_pct.txt");
       let infl_rate = get_data("data/annual_infl_rate_1926-2013_pct.txt");
      println!("{:?}", infl_rate);
}
