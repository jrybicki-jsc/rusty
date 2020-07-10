use gu::{Draw, Screen, Button};

struct SelectBox {
   width: u32,
   height: u32,
   options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing selectBox");
    }
}

fn main() {
    println!("Hello, world!");

    let screen = Screen {
        components: vec![
             Box::new(SelectBox{
                   width: 75,
                   height: 10,
                   options: vec![String::from("Yes"), String::from("No"), String::from("Cancel"),]
              }),
             Box::new(Button { width: 50, height: 10, label: String::from("OK") }),
        ],};

    screen.run();
}
