extern crate web_view;

use rand::seq::SliceRandom;
use rand::Rng;
use std::str::FromStr;
use web_view::*;

fn main() {
    let res = web_view::builder()
        .title("Doors")
        .content(Content::Html(include_str!("pages/index.html")))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(Vec::<Game>::new())
        .invoke_handler(invoke_handler)
        .run()
        .unwrap();
    println!("res: {:?}", res);
}


fn invoke_handler(wv: &mut WebView<std::vec::Vec<Game>>, arg: &str) -> WVResult {
    println!("Arg is {}", arg);

    let mut rng = rand::thread_rng();

    if arg == "exit" {
        println!("exiting!");
        wv.exit();
    }

    let mut data = &mut *wv.user_data_mut();
    if data.len() == 0 {
        println!("No game status found, generating new one!");
        let game = Game::new_random_game(0, 0);
        data.push(game);
        wv.eval(&format!("clean(0, 0)"));
    }

    if arg.starts_with("door") {
        let ss = &arg.to_string()[4..];
        let door_nr = usize::from_str(ss).unwrap();

        println!("Door open {}", ss);
        let mut data = &mut *wv.user_data_mut();
        let mut g = data.pop().unwrap();
        println!("Game {:?}", g);
        let reveal = select_door_to_rev(&g, door_nr);
        let mut cmd = format!("reveal({}, {}, {}, {}, 1)", reveal, door_nr, g.goat_door, 0);
       
        g.selected_door = door_nr;
        g.rev_door = reveal;
        println!("New game status: {:?}", g);
        data.push(g);
        wv.eval(&cmd);
    }
    if (arg=="switch") || (arg=="noswitch") {
        let mut data = &mut *wv.user_data_mut();
        let mut g = data.pop().unwrap();
        
        if arg=="switch" {
           println!("Switching...{:?}", g);
           g.switch();
           println!("Switched...{:?}", g);
        }

        if g.is_winner() {
            println!("Winner!");
        }

        if g.is_goat() {
             println!("no winner");
            
        }

        let mut cmd = format!("reveal({}, {}, {}, {}, 0)", g.selected_door, g.selected_door, g.goat_door, g.price_door);

         wv.eval(&cmd);
         
    }
    Ok(())
}

#[derive(Debug)]
struct Game {
    goat_door: usize,
    price_door: usize,
    selected_door: usize,
    rev_door: usize,
    wins: usize,
    losses: usize,
}

trait Eval {
    fn is_winner(&self) -> bool; 
    fn is_goat(&self) -> bool;
    fn switch(&mut self);
}

trait GameConstructor {
     fn new_random_game(wins: usize, loses: usize) -> Self;
}

impl Eval for Game {
     fn is_winner(&self) -> bool {
          self.price_door == self.selected_door
     }

     fn is_goat(&self) -> bool {
          self.selected_door == self.goat_door
     }

     fn switch(&mut self) {
        let mut doors = vec![1, 2, 3];
        doors.retain(|&x| x != self.selected_door && x != self.rev_door);
        self.selected_door = doors.pop().unwrap();
     }

 }

impl GameConstructor for Game {
     fn new_random_game(wins: usize, loses: usize) -> Self {
        let mut rng = rand::thread_rng();
         let mut doors = vec![1, 2, 3];
          doors.shuffle(&mut rng);

          let goat_door: usize = doors.pop().unwrap();
          let price_door: usize = doors.pop().unwrap();

         let game = Game {
           goat_door: goat_door,
           price_door: price_door,
           selected_door: 0,
           rev_door: 0,
           wins: wins,
           losses: loses,
       };

       game
     }
}

fn select_door_to_rev(game: &Game, selected: usize) -> usize {
    let mut doors = vec![1, 2, 3];
    let mut rng = rand::thread_rng();

    doors.retain(|&x| x != game.price_door && x != selected);

    doors.shuffle(&mut rng);
    println!("Remainng doors: {:?}", doors);
    doors[0]
}
