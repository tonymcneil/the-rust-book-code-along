fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let hello = &s[..5];
    let world = &s[6..];
    println!("{} {}", hello, world);

    let hello_world = &s[..];
    println!("{}", hello_world);

    println!("first_word: {}", first_word(&s[..]));
    println!("first_word: {}", first_word("tick tock"));

    let a = [1, 2, 3, 4, 5];
    let first_3 = &a[..3];
    println!("{:?}", first_3);
    // for (_, &item) in first_3.iter().enumerate() {
    //     println!("{}", item);
    // }
}

fn first_word(s: &str) -> &str {
    let s_bytes = s.as_bytes();

    for (i, &item) in s_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
