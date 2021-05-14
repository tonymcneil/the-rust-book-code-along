use rust_book_ch07_01_pkg_crates_mods::eat_at_resturant;

// FIXME, call functions from src/lib.rs
fn main() {
    // by convention src/main.rs is the default binary crate
    // front_of_house::hosting::add_to_waitlist();
    println!("I'm hungry...");
    eat_at_resturant();
}
