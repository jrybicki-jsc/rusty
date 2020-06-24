use std::thread;
use std::time::Duration;

fn main() {
   println!("Your workout for today is");
   generate_workout(21, 5);

   let x_cl = |x| x;
   let s = x_cl(String::from("Hello"));
   // this wont work as the typed infered in first run remain valid
   //let m = x_cl(20);

   let v1 = vec![2, 4, 5];
   let v1_iter = v1.iter();

   for val in v1_iter{
       println!("Got: {}", val);
   }


}

struct Cacher<T> where T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32,
{
   fn new(calculation: T) -> Cacher<T> {
       Cacher {
          calculation,
          value: None,
       }
   }

   fn value(&mut self, arg: u32) -> u32 {
      match self.value {
           Some(v) => v,
           None => {
               let v = (self.calculation)(arg);
               self.value = Some(v);
               v
           }
      }
   }
}


fn generate_workout(i: u32, rand: u32) {
   let mut expensive_result = Cacher::new(|num| {
       println!("calculating slowly...");
       thread::sleep(Duration::from_secs(2));
       num
   });

   if i < 25 {
      println!("Today do {} pushups!", expensive_result.value(i));
      println!("Today do {} situps!",expensive_result.value(i));
   } else {
      if rand == 3 {
         println!("Take a break!");
      } else {
         println!("Today run for {} minutes", expensive_result.value(i));
      }
   }
}

#[derive(PartialEq, Debug)]
struct Shoe {
     size: u32,
     style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
  
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
      Counter {count:0}
    }
}


impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
       if self.count < 3 {
           self.count+=1;
           Some(self.count)
       } else {
           None
       }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

   #[test]
   fn filter_by_size() {
      let shoes = vec![
           Shoe{size:10, style: String::from("sneaker")},
           Shoe{size:12, style: String::from("sandal")},
           Shoe{size:10, style: String::from("boot")},
       ];

      let in_size = shoes_in_my_size(shoes, 10);
      assert_eq!(in_size, vec![Shoe{size:10, style: String::from("sneaker")}, Shoe{size:10, style: String::from("boot")}]);
   }

   #[test]
   fn tss() {
      let mut counter = Counter::new();
      assert_eq!(counter.next(), Some(1));
      assert_eq!(counter.next(), Some(2));
      assert_eq!(counter.next(), Some(3));
      assert_eq!(counter.next(), None);
   }

   #[test]
   fn it_demo() {
      let v1 = vec![1, 2];
      
      let mut v1_iter = v1.iter();
      assert_eq!(v1_iter.next(), Some(&1));
      assert_eq!(v1_iter.next(), Some(&2));
      assert_eq!(v1_iter.next(), None);
   }

   #[test]
   fn it_sum() {
      let v1 = vec![1, 2, 3];
      let v1_iter = v1.iter();
      let total: i32 = v1_iter.sum();

      assert_eq!(total, 6);
   }

   #[test]
   fn it_map() {
     let v1 = vec![1, 2, 3];
     let v1_iter = v1.iter();

     let v2: Vec<_> = v1_iter.map(|x| x+1).collect(); // lazy evaluation of map
 
     assert_eq!(v2, vec![2, 3, 4]);
   }
}


