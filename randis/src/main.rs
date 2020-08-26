use rand::Rng;
use std::collections::HashSet;

fn mc_dice(trails: usize) -> f64 {
 let mut rng = rand::thread_rng();
   let mut suc = 0;

   for _ in 0..trails {
       let mut faces = HashSet::new();
       for _ in 0..6 {
           let roll = rng.gen_range(0, 6);
           faces.insert(roll);
       }

       if faces.len() == 6 {
          suc+=1;
       }
    }

    let prob = suc as f64 / trails as f64;
    println!("Prob {}", prob);
    prob
}

fn main() {
   let prob = mc_dice(10000);

   let mut rng = rand::thread_rng();
   let trails = 10000;

   let mut wins = 0;
   let mut change_wins = 0;
   for _ in 0..trails {
       let player_door:usize = rng.gen_range(0, 3);
       let price_door:usize = rng.gen_range(0, 3);
 

       if player_door == price_door {
            wins+=1;
       } else {
            change_wins +=1;
       }
   }

   println!("Player wins: {}", wins as f64 / trails as f64);
   println!("Change wins: {}", change_wins as f64 / trails as f64);
}
