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

fn doors() {

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


fn main() {
     let mut rng = rand::thread_rng();
     let max_people = 40;
     let runs = 1000;
     
     for people in 2..max_people {
         let mut shar =0;
         for run in 0..runs {
              let mut bdays = Vec::<usize>::new();
              let mut bhash = HashSet::<usize>::new();
              for i in 0..people {
                   let bday:usize = rng.gen_range(0, 365);
                   bdays.push(bday);
                   bhash.insert(bday);
              }

              let diff = bdays.len() - bhash.len();
              if diff >0 {
                     shar+=1;
              }
         }
         println!("{} people in a room: Probabilty shared: {}", people, shar as f64 / runs as f64);

     }

}
