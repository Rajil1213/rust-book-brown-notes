/**
* Structure of this library crate:
* crate
└── front_of_house
    ├── hosting
    │   ├── add_to_waitlist
    │   └── seat_at_table
    └── serving
        ├── take_order
        ├── serve_order
         take_payment
└── back_of_house
    ├── fix_incorrect_order
    └── cook_order
*/
mod front_of_house;

mod back_of_house;

// bring hosting into scope
use crate::front_of_house::hosting;

fn eat_at_restaurant() {
    // Absolute
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("wheat");
    // this is not allowed:
    // meal.seasonal_fruit = String::from("blueberies");
    println!("I'd like {} toast please", meal.toast);

    // after using `use`:
    hosting::add_to_waitlist();
}
