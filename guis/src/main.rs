extern crate web_view;

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
    println!("res: {:?}", res)
}

fn invoke_handler(wv: &mut WebView<std::vec::Vec<Game>>, arg: &str) -> WVResult {
    println!("Arg is {}", arg);

    if arg == "exit" {
        println!("exiting!");
        wv.exit();
    }

    if arg == "init" {
        let mut rng = rand::thread_rng();
        println!("initializing...");
        let goat_door: usize = rng.gen_range(1, 4);
        let price_door: usize = rng.gen_range(1, 4);

        let game = Game {
            goat_door: goat_door,
            price_door: price_door,
            wins: 0,
            losses: 0,
        };
        let mut data = &mut *wv.user_data_mut();
        data.push(game);
    }
    if arg.starts_with("door") {
        let ss = &arg.to_string()[4..];
        let door_nr = usize::from_str(ss).unwrap();

        println!("Door open {}", ss);
        let mut data = &mut *wv.user_data_mut();
        let g = data.pop();
        match g {
            None => println!("You have to start the game first"),
            Some(game) => {
                println!("Current game is: {:?}", game);
                println!("Price door is: {}", game.price_door);
                if (door_nr == game.price_door) {
                    println!("We have a winner!");
                    wv.eval(&format!("winner()"));
                } else if (door_nr == game.goat_door) {
                    wv.eval(&format!("looser()"));
                } else {
                    println!("nothing...");
                }
            }
        }
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
