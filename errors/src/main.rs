use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("Hello, world!");
    let m = vec![1, 2, 3];

    //m[7];

    let f = File::open("hello.txt");
    let f = match f {
       Ok(file) => file,
       Err(error) =>  match error.kind() {
               ErrorKind::NotFound => match File::create("hello.txt") {
                     Ok(fc) => fc,
                     Err(e) => panic!("Problem creating file {:?}", e), 
               },
               other_error => {
                 panic!("Problem opening file: {:?}", other_error)
               }
       },
   };

   let f = File::open("hello.txt").unwrap(); //value inside OK, or panic! if Err
   //let f2 = File::open("hello1.txt").expect("Failed to open hello1.txt"); //same as unwrap with msg

   let cont = read_username_from_file().expect("Failed to read username");
   println!("Username: {}", cont);

   let c2 = read_uname().expect("Unable to read username");
   println!("Uname {}", c2);
}
use std::io;
use std::io::Read;

fn read_uname() -> Result<String, io::Error> {
   let mut f = File::open("uname.txt")?;
   let mut s = String::new();
   f.read_to_string(&mut s)?;
   Ok(s)
 
}

fn read_u() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file() -> Result<String, io::Error> {

   let f = File::open("hello.txt");
   let mut f = match f {
       Ok(file) => file,
       Err(e) => return Err(e),
   };

   let mut s = String::new();

   match f.read_to_string(&mut s) {
       Ok(_) => Ok(s),
       Err(e) => Err(e),
   }
}
