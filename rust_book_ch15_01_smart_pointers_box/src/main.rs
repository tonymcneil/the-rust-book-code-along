use crate::List::{Cons, Nil};

fn main() {
    println!("Smart pointers, Box...");

    println!("putting a i32 onto the heap via Box...");
    let b = Box::new(5);
    println!("Box(5): {}", b);
    println!("--------------------------------------------\n");

    println!("defining a recursive data structure with Box...");
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list: {:?}", list);
    println!("--------------------------------------------\n");

}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

