extern crate web_view;

use rand::Rng;
use rand::seq::SliceRandom;
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

fn gen_game(wins: usize, loses: usize) -> Game {
   let mut rng = rand::thread_rng();
   let goat_door: usize = rng.gen_range(1, 4);
        let price_door: usize = rng.gen_range(1, 4);

        let game = Game {
            goat_door: goat_door,
            price_door: price_door,
            wins: wins,
            losses: loses,
        };

   game
}

fn select_door_to_rev(game: &Game, selected: usize) ->usize{

   let mut doors =  vec![1, 2, 3];
   let mut rng = rand::thread_rng();

   doors.retain(|&x| x != game.price_door && x != selected);
 
   doors.shuffle(&mut rng);
   println!("Remainng doors: {:?}", doors);
   doors[0]
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
        let game = gen_game(0, 0);
        data.push(game);
     }

    
    if arg.starts_with("door") {
        let ss = &arg.to_string()[4..];
        let door_nr = usize::from_str(ss).unwrap();

        println!("Door open {}", ss);
        let mut data = &mut *wv.user_data_mut();
        let g = data.pop().unwrap();
        println!("Game {:?}", g);
        let reveal = select_door_to_rev(&g, door_nr);
        let mut cmd = format!("reveal({})", reveal);
        if reveal == g.goat_door {
            cmd = format!("reveal_goat({})", reveal);
         }
   
        data.push(g);
        
        wv.eval(&cmd);
    }
    Ok(())
}

#[derive(Debug)]
struct Game {
    goat_door: usize,
    price_door: usize,
    wins: usize,
    losses: usize,
}
