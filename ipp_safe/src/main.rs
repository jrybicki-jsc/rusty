use rand::Rng;
use std::convert::TryInto;


fn fitness(combo: &Vec<u8>, attempt: &Vec<u8>) -> u8 {
   let mut score: u8 = 0;
   
    for (c, a) in combo.iter().zip(attempt.iter()) {
         if c==a {
            score+=1;
         }
    }

    score
} 

fn main() {
    let  combo = vec![9, 9, 7, 6, 5];
    let mut attempt = vec![1, 1, 2, 4, 6];
    let mut fit = fitness(&combo, &attempt);
    let mut rng = rand::thread_rng();
    let mut iter = 0;

    while fit < combo.len().try_into().unwrap() {
       let index = rng.gen_range(0, combo.len());
       let val   = rng.gen_range(0, 10);
       let mut next_try = Vec::new();
       next_try.extend(attempt.iter().copied());

       next_try[index] = val;
       
       if fit < fitness(&next_try, &combo) {
          fit = fitness(&next_try, &combo);
          attempt = next_try;
          println!("Current best {:?} \t{}", attempt, fit);
       } 
       iter+=1;
    }

    println!("Done after {} iters", iter);
  
}
