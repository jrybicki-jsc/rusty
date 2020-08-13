use rand::Rng;
use std::convert::TryInto;
use std::time::Instant;

fn fitness(combo: &Vec<u8>, attempt: &Vec<u8>) -> u8 {
    let mut score: u8 = 0;

    for (c, a) in combo.iter().zip(attempt.iter()) {
        if c == a {
            score += 1;
        }
    }

    score
}

fn main() {
    let combo = vec![9, 9, 7, 6, 5, 1, 2, 3, 1, 2];
    let mut attempt = vec![1, 1, 2, 4, 6, 2, 3, 5, 4, 8];
    let mut fit = fitness(&combo, &attempt);
    let mut rng = rand::thread_rng();
    let mut iter = 0;
    let mut indices:Vec<usize> = Vec::new();
    for i in 0..combo.len() {
         indices.push(i);
    }
    
    let now = Instant::now();
    while fit < combo.len().try_into().unwrap() {
        let index_index = rng.gen_range(0, indices.len());
        let index = indices[index_index];
        let val = rng.gen_range(0, 10);
        let mut next_try = Vec::new();
        next_try.extend(attempt.iter().copied());

        next_try[index] = val;

        if fit < fitness(&next_try, &combo) {
            fit = fitness(&next_try, &combo);
            attempt = next_try;
            println!("Current best {:?} \t{}", attempt, fit);
            //dont try this index anymore
            indices.remove(index_index);
        }
        iter += 1;
    }

    println!("Done after {} iters and {} microsec", iter, now.elapsed().as_micros());
}
