fn main() {
    ////////////////////////////////////////////
    println!("Example 1: coins to cents...");
    println!(" Penny in cents: {:?}", value_in_cents(Coin::Penny));
    println!(" Nickel in cents: {:?}", value_in_cents(Coin::Nickel));
    println!(" Dime in cents: {:?}", value_in_cents(Coin::Dime));
    println!(
        " Quarter in cents: {:?}",
        value_in_cents(Coin::Quarter(UsState::Alabama))
    );

    ////////////////////////////////////////////
    println!("Example 2: matching against Option...");
    println!(" None + 1: {:?}", plus_one(None));
    println!(" Some(41) + 1: {:?}", plus_one(Some(41)));

    ////////////////////////////////////////////
    println!("Example 2: matching single digit primes...");
    match_single_digit_prime_under_10(1);
    match_single_digit_prime_under_10(2);
    match_single_digit_prime_under_10(3);
    match_single_digit_prime_under_10(4);
    match_single_digit_prime_under_10(5);
    match_single_digit_prime_under_10(6);
    match_single_digit_prime_under_10(7);
    match_single_digit_prime_under_10(8);
    match_single_digit_prime_under_10(9);
    match_single_digit_prime_under_10(10); // NOTE: over 9 is caught by the _ general match
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    // Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            // multi-line...
            print!(" Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            print!(" State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(num: Option<i32>) -> Option<i32> {
    match num {
        None => None,
        Some(val) => Some(val + 1),
    }
}

fn match_single_digit_prime_under_10(num: u8) {
    match num {
        1 => println!("{}: yes", num),
        2 => println!("{}: no", num),
        3 => println!("{}: yes", num),
        4 => println!("{}: no", num),
        5 => println!("{}: yes", num),
        6 => println!("{}: no", num),
        7 => println!("{}: yes", num),
        8 => println!("{}: no", num),
        9 => println!("{}: no", num),
        _ => println!("{}: is over 9 so not considering", num),
    }
}
