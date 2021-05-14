fn main() {
    println!("panic for non-recoverable error...");
    panic!("crash and burn");

    // println!("this will panic at runtime...");
    // let v = vec![1, 2, 3];
    // v[99];
}
