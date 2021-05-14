use std::ops::Deref;

fn main() {
    println!("Smart pointers, Deref trait...");

    println!("normal dereferencing...");
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("assertions above passed");
    println!("----------------------------------\n");

    println!("Box dereferencing...");
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("assertions above passed");
    println!("----------------------------------\n");

    println!("Custom Box-like definition with similar dereferencing...");
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("assertions above passed");
    println!("----------------------------------\n");

    println!("Deference coercion on function parameters when a fn is called...");
    hello("Normal &str param");
    let explanation = "\
    Here we’re calling the hello function with the argument &m, which is a reference to a MyBox<String> value.\n\
    Because we implemented the Deref trait on MyBox<T> in Listing 15-10,\n\
    Rust can turn &MyBox<String> into &String by calling deref.\n\
    The standard library provides an implementation of Deref on String that returns a string slice,\n\
    and this is in the API documentation for Deref.\n\
    Rust calls deref again to turn the &String into &str, which matches the hello function’s definition.";
    hello(&MyBox::new(String::from(
        "A MyBox(String) param that gets coerced...",
    )));
    println!("Explanation: {}", explanation);
    println!("----------------------------------\n");
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
