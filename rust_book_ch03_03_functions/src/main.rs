/// order doesn't matter
fn function_here() {
    println!("function here");
}

/// main entry point, functions can call functions defined before them
fn main() {
    println!("Hello, world!");
    function_here();
    function_there(42);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("x = {}, y = {}", x, y);

    println!("five = {}", five());

    println!("plus_one(254) = {}", plus_one(254));
    // println!("plus_one(255) = {}", plus_one(255)); // will panic/wrap for debug/release respectively
    println!("plus_one_into(255) = {}", plus_one_into(255));
}

/// function taking a param
fn function_there(x: u8) {
    println!("function there, with arg: {}", x);
}

fn five() -> i32 {
    5
}

/// running this with arg 255 in debug mode will panic
/// however running in release mode will wrap to 0
fn plus_one(x: u8) -> u8 {
    x + 1
}

fn plus_one_into(x: u8) -> u16 {
    let x: u16 = x.into();
    x + 1
}
