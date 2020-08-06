use std::fmt;
use rand_distr::{Triangular, Distribution};
use rand;

enum Sex {
    Male,
    Female,
}

pub struct Rat {
    weight: f64,
    sex: Sex,
}

impl fmt::Display for Rat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:2.3}, {})", self.weight, self.sex)
    }
}

impl fmt::Display for Sex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Male => write!(f, "Male"),
            _ => write!(f, "Female"),
        }
    }
}

fn get_sample(num_rats: usize, min_weight: f64, max_weight: f64, mode_weight: f64) -> Vec<Rat> {

   let mut ret:Vec<Rat> = Vec::new();
   let d = Triangular::new(min_weight,max_weight, mode_weight).unwrap();
   let mut rng = rand::thread_rng();

   for _ in 0..num_rats {
       let weight = d.sample(&mut rng);
       let mut sex = Sex::Male;
       if  rand::random() {
           sex = Sex::Female;
       }
       ret.push(Rat{weight, sex});
   }
   ret
}

fn main() {

    let rats = get_sample(10, 12.0, 24.0, 18.0);
    println!("Got rat {}", rats[0]);

}
