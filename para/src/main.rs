use std::thread;
use std::time::Duration;

fn main() {
   let v = vec![1, 2, 3];

   let handle = thread::spawn(move || {
       for i in 1..3 {
           println!("Hi number {} from the thread", i);
           println!("{:?}", v);
       }
   });
    //move protects you from doing that:
   //drop(v);

   for i in 1..5 {
       println!("Hi number {} from main", i);
       thread::sleep(Duration::from_millis(1));
   }

   handle.join().unwrap();
}
