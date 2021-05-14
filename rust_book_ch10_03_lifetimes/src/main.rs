use std::fmt::Display;

fn main() {
    println!("Lifetimes...");

    let s1 = String::from("tony123");
    let s2 = "samantha";

    println!("Using a function with a declared lifetime...");
    let longer_string = longest(s1.as_str(), s2);
    println!("s1: {}, s2:{}, longer_string: {}", s1, s2, longer_string);
    println!("------------------------------------\n");

    println!("Passing in params with different lifetimes...");
    let string1 = String::from("abcd");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is: {}", result);
    }
    println!("------------------------------------\n");

    println!("Calling a function returning an owned value as opposed to a reference to avoid lifetime specification...");
    println!("greeting1 is: {}", greeting1());
    println!("------------------------------------\n");

    println!("Calling a function returning a reference with 'static lifetime declared...");
    println!("greeting2 is: {}", greeting2());
    println!("------------------------------------\n");

    println!("Calling a function returning a reference with a lifetime the same as the marker passed in...");
    let lifetime_marker = "just used for marking lifetime";
    println!("greeting3 is: {}", greeting3(lifetime_marker));
    println!("------------------------------------\n");

    println!("Lifetimes in struts...");
    let novel = String::from("Call me MR-T. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("i: {:?}", i);
    println!("------------------------------------\n");

    println!(
        "Using generic type parameters, trait bounds, and lifetimes in a function definition..."
    );
    let s1 = "abc123";
    let s2 = "xyz99";
    let l = longest_with_an_announcement(s1, s2, "getting longest...");
    println!("longest_with_an_announcement result: {:?}", l);
    println!("------------------------------------\n");
}

fn longest<'a>(string1: &'a str, string2: &'a str) -> &'a str {
    if string1.len() > string2.len() {
        &string1
    } else {
        &string2
    }
}

fn greeting1() -> String {
    String::from("hello1")
}

fn greeting2() -> &'static str {
    "hello2"
}

fn greeting3<'a>(_marker: &'a str) -> &'a str {
    // NOTE: I just used the _marker param to mark a lifetime
    "hello3"
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
