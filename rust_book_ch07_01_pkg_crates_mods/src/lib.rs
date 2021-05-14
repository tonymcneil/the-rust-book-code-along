// NOTE:
//  package: created with "cargo new" command
// - has Cargo.toml file, describes how to build crates
// - has 0 or 1 library crates
// - has 0 or more binary creates
// - has at least one of the above (lib or bin crate)

// NOTE:
// src/lib.rs conventional default library module in implicitly named "crate" module
// src/main.rs conventional default binary module in implicitly named "crate" module
// src/bin holds binary crates

// module structure
// crate
//     └── front_of_house
//     ├── hosting
//     │   ├── add_to_waitlist
//     │   └── seat_at_table
//     └── serving
//     ├── take_order
//     ├── serve_order
//     └── take_payment

mod front_of_house {

    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add-add_to_waitlist()")
        }

        // fn seat_at_table() {}
    }

    // mod serving {
    //     fn take_order() {}

    //     fn serve_order() {}

    //     fn take_payment() {}
    // }
}

mod back_of_house {
    #[derive(Debug)]
    pub struct Food {
        pub toast: String,
        seasonal_fruit: String,
    }

    #[derive(Debug)]
    pub enum Drink {
        Coffee,
        // IceCoffee,
        // Tea,
        IceTea,
    }

    #[derive(Debug)]
    pub struct Breakfast {
        pub food: Food,
        pub drink: Drink,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                food: Food {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"), // can't change from outside module
                },
                drink: Drink::Coffee, // can change from outside of module as it's public
            }
        }

        pub fn list_order(breakfast: &Breakfast) {
            println!("{:?} & {:?}", breakfast.food, breakfast.drink);
        }
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_resturant() {
    // absolute path via explicitly specifying implicit crate module
    // crate::front_of_house::hosting::add_to_waitlist();

    // relative path as this function is in the implicit crate module
    // front_of_house::hosting::add_to_waitlist();

    // utilise the use declaration above this module
    hosting::add_to_waitlist();

    // create brekkie choosing the toast...
    let mut brekkie = back_of_house::Breakfast::summer("Rye");

    // actually, change the public toast, and go for an ice tea...
    brekkie.food.toast = String::from("Rye");
    brekkie.drink = back_of_house::Drink::IceTea;

    // but can NOT change the private seasonal fruit as not declared public
    // brekkie.seasonal_fruit = String::from("Pineapple");

    // list the order
    back_of_house::Breakfast::list_order(&brekkie);
    // list the order again to show the borrow
    //    back_of_house::Food::list_order(&brekkie.0);
}
