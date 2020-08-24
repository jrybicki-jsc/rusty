use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

const NUM_EQUIV_VOLUMES: usize = 1000;
const MAX_CIVS: usize = 5000;
const TRAILS: usize = 1000;
const CIV_STEP_SIZE: usize = 100;

fn main() -> std::io::Result<()> {
    let mut rng = rand::thread_rng();

    let mut num_civs = 100;
    let mut cpv = Vec::new();
    let mut probs = Vec::new();
    for num_civs in (2..MAX_CIVS).step_by(CIV_STEP_SIZE) {
        let civs_per_volume:f64 = num_civs as f64/ NUM_EQUIV_VOLUMES as f64;

        let mut num_single_civs = 0;
        for i in 0..TRAILS {
            let mut locations = Vec::new();
            while locations.len() < num_civs {
                let location = rng.gen_range(0, NUM_EQUIV_VOLUMES);
                locations.push(location);
            }

            let mut counter: HashMap<usize, u16> = HashMap::new();
            for l in locations {
                let mut v = counter.entry(l).or_insert(0);
                *v += 1;
            }

            let si = counter
                .iter()
                .filter(|(l, c)| **c == 1)
                .map(|(l, c)| l)
                .count();
            num_single_civs += si;
        }
        let prob:f64 = 1.0 - (num_single_civs as f64/ (num_civs * TRAILS) as f64);

        println!("Num civs         {}", num_civs);
        println!("Civs per volume: {}", civs_per_volume);
        println!("Probability      {}", prob);
        cpv.push(civs_per_volume);
        probs.push(prob);
    }

    let mut file = File::create("foo.txt")?;
    for (c, p) in cpv.iter().zip(probs.iter()) {
       file.write_all(format!("{},{}", c, p).as_bytes())?;
    }

    Ok(())
}
