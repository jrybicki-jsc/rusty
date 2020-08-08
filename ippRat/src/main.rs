use std::fmt;
use rand_distr::{Triangular, Distribution};
use rand;


#[derive(PartialEq)]
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


fn average(rats: &Vec<Rat>, sex: Option<Sex>) -> f64 {

  let mut sum = 0.0;
  let mut count = rats.len() as f64;
  match sex {
      Some(s) => {
               count =  rats.iter().filter(|r| r.sex==s).count() as f64;
               sum = rats.iter().filter(|r| r.sex == s).map(|r| r.weight).sum::<f64>();
      }, 
      _ => sum = rats.iter().map(|r| r.weight).sum::<f64>(),
  };

  sum / count
}

fn get_fitness(rats: &Vec<Rat>, goal: f64) -> f64{
   let  avg = average(rats, None);
   
    avg/goal
}

fn sort(rats: &mut Vec<Rat>) {
    rats.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap());

}

fn select(rats: &mut Vec<Rat>, to_retain: usize) -> Vec<&Rat> {
   sort(rats);
   let mut ret: Vec<&Rat>  = Vec::new();
   let mut males = 0;
   let mut females = 0;

   for r in rats.iter() {
       match r.sex  {
           Sex::Male => {
               if males < to_retain / 2 {
                  males +=1;
                  ret.push(r);
               }
           }
           _ => {
              if females < to_retain / 2 {
                 females +=1;
                 ret.push(r);
              }
           }
       };

      if ret.len() >= to_retain {
         break;
      }
   }

  ret    
}



fn main() {

    let mut rats = get_sample(10, 12.0, 24.0, 18.0);
    println!("Got rat {}", rats[0]);

    println!("Avg. male weight {}", average(&rats, Some(Sex::Male)));
    println!("Avg. pop. weight {}", average(&rats, None));
    println!("Fitness          {}", get_fitness(&rats, 18.0)); 

    let tops = select(&mut rats, 6);
    for r in tops {
         println!("{}", r);
    }

    println!("");
    for r in rats {
       println!("{}", r);
    }
}
