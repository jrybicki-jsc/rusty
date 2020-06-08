

fn main() {
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
