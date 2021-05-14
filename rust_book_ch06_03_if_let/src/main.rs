fn main() {
    let some_u8_val = Some(0u8);

    // the match way...
    match some_u8_val {
        Some(3) => println!("got 3"),
        _ => {
            println!("not 3");
            ()
        }
    }

    // the if let way...
    if let Some(3) = some_u8_val {
        println!("got 3");
    } else {
        println!("not 3");
        ()
    }
}
