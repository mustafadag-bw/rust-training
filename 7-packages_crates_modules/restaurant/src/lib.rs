// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

#[derive(Debug)]
pub struct Asparagus {}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// cannot be used in another module without moved into that module.
use crate::front_of_house::hosting;

// pub use .... => external module can reach it as => restaurant::hosting...

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
        // hosting is now stand alone hence access directly via super.
        use super::hosting;
        // this way we know the parent module for the function
        hosting::add_to_waitlist();
    }

    fn cook_order() {}

    // The default for enum variants is to be public.
    pub enum Appetizer {
        Soup,
        Salad,
    }
    // Struct fields follow the general rule of everything being private by default unless annotated with pub.
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
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // with use keyword
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}


// use std::fmt;
// use std::io;
// 
// We define where each result is from
// fn function1() -> fmt::Result {}
// fn function2() -> io::Result<()> {}

// OR: use std::io::Result as IoResult;

