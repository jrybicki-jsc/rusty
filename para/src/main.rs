use std::thread;
use std::time::Duration;
use std::sync::mpsc;


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

   //mpsc: multiple producer single consumer
   let (tx, rx) = mpsc::channel();
   let tx1 = mpsc::Sender::clone(&tx);

   thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        //tx.send borrows the value so it is no longer available here:
        //println!("val = {}", val);

       let vals = vec![
            String::from("from"),
            String::from("the"),
            String::from("thread"),
       ];
       for val in vals {
           tx.send(val).unwrap();
           thread::sleep(Duration::from_secs(1));
       }
   });

   thread::spawn(move || {
      let vals = vec![
            String::from("and"),
            String::from("some"),
            String::from("more"),
       ];
       for val in vals {
           tx1.send(val).unwrap();
           thread::sleep(Duration::from_secs(1));
       }
   });



   for recieved in rx {
          println!("Got {}!", recieved); 
   }
}
