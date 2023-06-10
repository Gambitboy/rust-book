use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

use std::fmt;
use std::io;

// fn function1() -> fmt::Result {}
// fn function2() -> io::Result<()> {}

use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

pub use crate::front_of_house::hosting;

mod front_of_house;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_wait_list();

    // Relative Path
    front_of_house::hosting::add_to_wait_list();

    // Shortcut with use
    hosting::add_to_wait_list();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries"); // Wont work cause seasonal_fruit is private

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_wait_list();
    }
}

fn deliver_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order()
    }

    fn cook_order() {}
}

// use std::cmp::Ordering;
// use std::io;

// use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;

// use std::io::{self, Write};

// use std::collections::*;
