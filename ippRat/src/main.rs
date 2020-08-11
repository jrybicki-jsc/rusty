use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Triangular};
use std::fmt;

#[derive(PartialEq, Copy, Clone)]
enum Sex {
    Male,
    Female,
}

#[derive(PartialEq, Copy, Clone)]
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
    let mut ret: Vec<Rat> = Vec::new();
    let d = Triangular::new(min_weight, max_weight, mode_weight).unwrap();
    let mut rng = rand::thread_rng();

    for _ in 0..num_rats {
        let weight = d.sample(&mut rng);
        let mut sex = Sex::Male;
        if rand::random() {
            sex = Sex::Female;
        }
        ret.push(Rat { weight, sex });
    }
    ret
}

fn average(rats: &Vec<Rat>, sex: Option<Sex>) -> f64 {
    let mut sum = 0.0;
    let mut count = rats.len() as f64;
    match sex {
        Some(s) => {
            count = rats.iter().filter(|r| r.sex == s).count() as f64;
            sum = rats
                .iter()
                .filter(|r| r.sex == s)
                .map(|r| r.weight)
                .sum::<f64>();
        }
        _ => sum = rats.iter().map(|r| r.weight).sum::<f64>(),
    };

    sum / count
}

fn get_fitness(rats: &Vec<Rat>, goal: f64) -> f64 {
    let avg = average(rats, None);

    avg / goal
}

fn sort(rats: &mut Vec<Rat>) {
    rats.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap());
}

fn select(rats: &mut Vec<Rat>, to_retain: usize) -> (Vec<Rat>, Vec<Rat>) {
    sort(rats);
    let mut males: Vec<Rat> = Vec::new();
    let mut females: Vec<Rat> = Vec::new();

    for r in rats.iter() {
        match r.sex {
            Sex::Male => {
                if males.len() < to_retain / 2 {
                    males.push(*r);
                }
            }
            _ => {
                if females.len() < to_retain / 2 {
                    females.push(*r);
                }
            }
        };

        if (males.len() + females.len()) >= to_retain {
            break;
        }
    }

    (males, females)
}

fn breed(males: &mut Vec<Rat>, females: &mut Vec<Rat>, litter_size: usize) -> Vec<Rat> {
    let mut rng = thread_rng();

    males.shuffle(&mut rng);
    females.shuffle(&mut rng);
    let mut children: Vec<Rat> = Vec::new();

    for (m, f) in males.iter().zip(females.iter()) {
        for _ in 0..litter_size {
            let weight;
            if f.weight < m.weight {
                weight = rng.gen_range(f.weight, m.weight);
            } else {
                weight = rng.gen_range(m.weight, f.weight);
            }

            let mut sex = Sex::Male;
            if rand::random() {
                sex = Sex::Female;
            }
            let child = Rat { weight, sex };
            children.push(child);
        }
    }
    children
}

fn mutate(children: &mut Vec<Rat>, mutate_ods: f64, mutate_min: f64, mutate_max: f64) {
    let mut rng = thread_rng();

    for c in children {
        if rng.gen::<f64>() < mutate_ods {
            print!("New mutation of {}", c);
            let new_weight = c.weight * rng.gen_range(mutate_min, mutate_max);
            c.weight = new_weight;
            println!(" changed into {}", c);
        }
    }
}

fn main() {
    let litter_size = 8;
    let mutate_odds = 0.01;
    let mutate_min = 0.5;
    let mutate_max = 1.2;
    let target_weight = 30.0;
    let to_retain = 20;
    let generation_limit = 500;
    let mut generation = 0;

    let mut rats = get_sample(to_retain, 12.0, 24.0, 18.0);

    while generation < generation_limit {
        println!("Population {}", generation);
        println!("Avg. male weight {}", average(&rats, Some(Sex::Male)));
        println!("Avg. pop. weight {}", average(&rats, None));
        let fitnes = get_fitness(&rats, target_weight);
        println!("Fitness          {}", fitnes);
        if fitnes >= 1.0 {
            break;
        }

        let (mut males, mut females) = select(&mut rats, to_retain);
        println!("Got {} males and {} females", males.len(), females.len());

        let mut children = breed(&mut males, &mut females, litter_size);
        mutate(&mut children, mutate_odds, mutate_min, mutate_max);
        rats.clear();
        rats.append(&mut children);
        for (m, f) in females.iter().zip(males.iter()) {
             rats.push(*f);
             rats.push(*m);
        }
      
        generation+=1;
    }
}
