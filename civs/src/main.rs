use rand::Rng;


const NUM_EQUIV_VOLUMES: usize = 1000;
const MAX_CIVS: usize = 5000;
const TRAILS: usize = 1000;
const CIV_STEP_SIZE: usize = 100;


fn main() {
     let mut rng = rand::thread_rng();

     let mut num_civs = 100;
     let civs_per_volume = num_civs / NUM_EQUIV_VOLUMES;

     for i in 0..TRAILS {
         let mut locations = Vec::new();
         while locations.len() < num_civs {
              let location = rng.gen_range(0, NUM_EQUIV_VOLUMES);
              locations.push(location);
         }

      }

}

