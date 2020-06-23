use std::thread;
use std::time::Duration;

fn main() {
   println!("Your workout for today is");
   generate_workout(21, 5);

   let x_cl = |x| x;
   let s = x_cl(String::from("Hello"));
   // this wont work as the typed infered in first run remain valid
   //let m = x_cl(20);
}

fn generate_workout(i: u32, rand: u32) {
   let expensive_clousure = |num| {
       println!("calculating slowly...");
       thread::sleep(Duration::from_secs(2));
       num
   };

   if i < 25 {
      println!("Today do {} pushups!", expensive_clousure(i));
      println!("Today do {} situps!",expensive_clousure(i));
   } else {
      if rand == 3 {
         println!("Take a break!");
      } else {
         println!("Today run for {} minutes", expensive_clousure(i));
      }
   }
}
