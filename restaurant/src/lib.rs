
use std::io::{self, Write};

mod front_of_house;

fn serve_order() {}

mod back_of_house {

    pub struct Breakfast {
        pub toast: String,
        seasonanl_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonanl_fruit: String::from("peaches")
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

use self::front_of_house::hosting as ho;

pub fn eat_at_restaurant() {
    ho::add_to_waitlist();

    ho::add_to_waitlist();
}
