mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}
        pub fn serve_order() {}
        fn take_payement() {}
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
//load from file src/mymod.rs or mymod/lib.rs
mod mymod;

use crate::front_of_house::hosting;
use crate::front_of_house::serving::serve_order as serve;
use crate::front_of_house::serving::take_order;
use std::collections::HashMap;

use crate::mymod::lists;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    let mut meal = front_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    //meal.seasonal_fruit = String::from("Blueberries");
    println!("I would like {} toast please", meal.toast);

    let order1 = front_of_house::Appetizer::Soup;
    let order2 = front_of_house::Appetizer::Salad;

    // with path
    hosting::add_to_waitlist();

    take_order();
    serve();

    let mut map = HashMap::new();
    map.insert(1, 2);

    lists::add_to_list();
}
