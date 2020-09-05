use std::fs;
//use std::io;
use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Triangular};
use std::str::FromStr;

fn get_data(fname: &str) -> Vec<f64> {
    let co = fs::read_to_string(fname).expect(&format!("Failed to open datafile {}", fname));
    println!("Got data {}", co.len());

    let mut ret = Vec::<f64>::new();
    for line in co.lines() {
        let datum: f64 = f64::from_str(line).unwrap();
        ret.push(datum);
    }

    ret
}

fn main() {
    let bonds = get_data("data/10-yr_TBond_returns_1926-2013_pct.txt");
    let stocks = get_data("data/SP500_returns_1926-2013_pct.txt");
    let blend_40_50_10 = get_data("data/S-B-C_blend_1926-2013_pct.txt");
    let blend_50_50 = get_data("data/S-B_blend_1926-2013_pct.txt");
    let infl_rate = get_data("data/annual_infl_rate_1926-2013_pct.txt");

    let start_value = 2000000;
    let min_years = 18;
    let max_years = 40;
    let most_likely_years = 25;
    let num_cases = 5000;

    let d = Triangular::new(min_years as f64, max_years as f64, most_likely_years as f64).unwrap();
    let mut rng = rand::thread_rng();

    let mut case_count = 0;
    //while case_count < num_cases {
    let mut investment = start_value;
    let start_year = rng.gen_range(0, bonds.len());
    let duration = d.sample(&mut rng) as usize;
    let end_year = start_year + duration;

    let mut returns = Vec::new();
    let mut inflation = Vec::new();
    for i in start_year..end_year {
        returns.push(bonds[i % bonds.len()]);
        inflation.push(infl_rate[i % infl_rate.len()]);
    }

    println!("{:?}", returns);
    //}
}
