fn main() {
    let ciphertext = "16 12 8 4 0 1 5 9 13 17 18 14 10 6 2 3 7 11 15 19";
    let cols = 4;
    let rows = 5;
    
   

    let mut sp = ciphertext.split(" ").collect::<Vec<&str>>();

    let keys =  "-1 2 -3 4";
    for key in keys.split(" ") {
        let k: i8 = key.parse().unwrap();
      
         let a = if k < 0 {
                sp.drain(0..rows).collect::<Vec<&str>>()
          } else {
              sp.drain(0..rows).rev().collect::<Vec<&str>>()
          };
          for el in a {
               print!("{}\t",el);
           }
          println!("");
    }
}
