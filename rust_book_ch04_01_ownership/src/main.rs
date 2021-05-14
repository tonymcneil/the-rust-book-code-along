fn main() {
    // string literals
    let mut str_literal = "hello world";
    println!(
        "str_literal (&str), original assignment: {}, length: {}",
        str_literal,
        str_literal.len()
    );
    str_literal = "tony";
    println!(
        "str_literal (&str), smaller assignment(uppercased): {}, length: {}",
        str_literal.to_uppercase(),
        str_literal.len()
    );
    str_literal = "Tony is in the house, YO!";
    println!(
        "str_literal (&str), larger assignment(lowercased): {}, length: {}",
        str_literal.to_lowercase(),
        str_literal.len()
    );

    // String type
    let mut s = String::from("hello");
    println!(
        "String, original assignment: {}, length: {}, capacity: {}",
        s,
        s.len(),
        s.capacity()
    );

    s.push_str(" tony");
    println!(
        "String, new extension: {}, length: {}, capacity: {}",
        s,
        s.len(),
        s.capacity()
    );

    let s_ref = s;
    println!(
        "String, new reference: {}, length: {}, capacity: {}",
        s_ref,
        s_ref.len(),
        s_ref.capacity()
    );
    // NOTE: referencing "s" now will be considered an error as we're borrowing a moved value

    /*
    ownership in functions...
    */
    let s1 = String::from("hello1"); // s1 comes into scope
    takes_ownership1(s1); // s1's value moves into the function...
                        // ... and so is no longer valid here
    // println!("s1 after move into function: {}", s1);

    let s2 = "hello2"; // s2 comes into scope
    makes_copy1(s2); // s's value is a reference being passed to the function...
    // ... is still valid here
    println!("s2 after move into function: {}", s2);

    let x = 5; // x comes into scope
    makes_copy2(x); // x would move into the function,
                   // but i32 is Copy, so it’s okay to still
                   // use x afterwards
    println!("x after copy into function: {}", x);

    /*
    moving in functions with consumed ownership...
     */
    let s1 = String::from("ownership is moved and consumed");
    println!("s1 val: {}", s1); // NOTE: can reference here before moving to fn
    let len = calculate_length_using_moved_and_consumed_ownership(s1);
    println!("s1 length: {}", len);
    // println!("s1 val: {}", s1); // NOTE: can't do this as it was already moved

    /*
    moving in functions with returned ownership...
     */
    let s2 = String::from("ownership is moved and then returned");
    let (s2, len) = calculate_length_using_moved_and_returned_ownership(s2);
    println!("s2 val: '{}' s2 length: {}", s2, len);

    /*
    borrowing in functions...
     */
    let s3 = String::from("a reference is borrowed");
    let len = calculate_length_using_borrowed_reference(&s3);
    println!("s3 val: '{}' s3 length: {}", s3, len);

    /*
    modifying a borrowed reference doesn't work...
     */
    let mut s4 = String::from("a mutable reference");
    println!("s4 val before mutation: '{}'", s4);
    change_mutable_borrowed_reference(&mut s4);
    println!("s4 val after mutation: '{}'", s4);

    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // NOTE: uncommenting will ERROR cannot borrow `s` as mutable more than once at a time
    println!("r1 {}", r1);
}

fn takes_ownership1(some_string: String) {
    // some_string comes into scope
    println!("some_string: {}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy1(some_string: &str) {
    // some_string comes into scope
    println!("some_string in fn: {}", some_string);
} // Here, some_integer goes out of scope. Nothing special happens.

fn makes_copy2(some_integer: i32) {
    // some_integer comes into scope
    println!("some_integer inside fn: {}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length_using_moved_and_consumed_ownership(s: String) -> usize {
    s.len()
}

fn calculate_length_using_moved_and_returned_ownership(s: String) -> (String, usize) {
    let length = s.len();
    // as the variable s was moved into this function, the fn will return ownership
    (s, length)
}

fn calculate_length_using_borrowed_reference(s: &String) -> usize {
    s.len()
}

fn change_mutable_borrowed_reference(s: &mut String) {
    s.push_str(" mutated");
}
