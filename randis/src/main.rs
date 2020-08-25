use rand::Rng;
use std::collections::HashSet;

fn main() {

   let mut rng = rand::thread_rng();
   let mut suc = 0;
   let trails = 100000;

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

    println!("Prob {}", suc as f64 / trails as f64);
   
}
