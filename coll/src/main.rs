use std::collections::HashMap;


fn mean(v: &Vec<i32>) -> f64{

   //let v = vec![1, 2, 3, 4, 5];
   let mut sum:i32 = 0;
   let mut count:i32 = 0;
   for i in v {
       sum= sum + i;
       count = count + 1;
   }

   (sum as f64)/(count as f64)
}

fn mode(v: &Vec<i32>) -> i32 {
   let mut scores= HashMap::new();

   for i in v {
      let count = scores.entry(i).or_insert(0);
      *count+=1;
   }

   let mut occurs =0;

   for (key, value) in &scores {
       if *value > occurs {
          occurs = **key;
       }
    }

   occurs

} 


fn main() {
    let myvec = vec![1, 2, 3, 4, 5, 6, 2, 3, 3];
    let m = mean(&myvec);
    println!("Mean={}", m);

    let mp = mode(&myvec);
    println!("{:?}", myvec);
    println!("Mode: {}", mp);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);


    let mut v = vec![1,2,3,4];

    let third: &i32 = &v[2];
    println!("The third element is: {}", third);

    match v.get(2) {
         Some(third) => println!("The third element is {}", third),
         None => println!("There is no third element"),
    }

    match v.get(100) {
        None => println!("There is no 100th element"),
        _ => println!("There is 100th element!"),
    }

    for i in &mut v {
        println!("{}", i);
        *i += 50;
        println!("{}", i);
    }
    #[derive(Debug)]
    enum Cell {
       Int(i32),
       Float(f64),
       Text(String),
    }

    let row = vec![
       Cell::Int(3),
       Cell::Text(String::from("blue")),
       Cell::Float(10.2),
    ];

   for i in &row {
      println!("{:?}", i);
   }
}
