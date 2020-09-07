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

fn simulate(asset: &Vec<f64>, infl_rate: &Vec<f64>) -> f64 {
    let start_value = 2000000;
    let min_years = 18;
    let max_years = 40;
    let most_likely_years = 25;
    let num_cases = 5000;
    let withdrawal: f64 = 80000.0;

    let d = Triangular::new(min_years as f64, max_years as f64, most_likely_years as f64).unwrap();
    let mut rng = rand::thread_rng();

    let mut case_count = 0;
    let mut bankrupts = 0;

     while case_count < num_cases {
        case_count+=1;
        let start_year = rng.gen_range(0, asset.len());
        let duration = d.sample(&mut rng) as usize;
        let end_year = start_year + duration;

        let mut returns = Vec::new();
        let mut inflation = Vec::new();
        for i in start_year..end_year {
            returns.push(asset[i % asset.len()]);
            inflation.push(infl_rate[i % infl_rate.len()]);
        }

        let mut withdrawal_adj = 0.0;
        let mut investments = start_value as f64;

        for (index, i) in returns.iter().enumerate() {
            let curr_inf = inflation[index] as f64 / 100.0;
            if index == 0 {
                withdrawal_adj = withdrawal;
            } else {
                withdrawal_adj = withdrawal_adj * (1.0 + curr_inf);
            }

            investments -= withdrawal_adj;
            investments = investments * (1.0 + i / 100.0);
//            println!("{:.2} {:.2}", investments, withdrawal_adj);
            if investments <= 0.0 {
                bankrupts+=1;
  //              println!("Bankrupt");
                break;
            }
        }
    }
    bankrupts as f64 / num_cases as f64

}

fn main() {
    let bonds = get_data("data/10-yr_TBond_returns_1926-2013_pct.txt");
    let stocks = get_data("data/SP500_returns_1926-2013_pct.txt");
    let blend_40_50_10 = get_data("data/S-B-C_blend_1926-2013_pct.txt");
    let blend_50_50 = get_data("data/S-B_blend_1926-2013_pct.txt");
    let infl_rate = get_data("data/annual_infl_rate_1926-2013_pct.txt");

    let prob = simulate(&stocks, &infl_rate);

   println!("Bankruptcy probabilty {:.2}", prob);
}
