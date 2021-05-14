fn main() {
    // file order doesn't matter when creating a concrete instance
    let user1 = User {
        email: String::from("user1@example.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 1,
    };
    println!("user1.email: {}, sign_in_count: {}", user1.email, user1.sign_in_count);

    let mut user2 = User {
        email: String::from("user2@example.com"),
        username: String::from("user2"),
        active: true,
        sign_in_count: 1,
    };
    user2.email = String::from("anotheremail@example.com");
    println!("user2.email: {}", user2.email);

    let user3 = build_user(String::from("user3@example.com"), String::from("user3"));
    println!(
        "user3.username: {}, active: {}",
        user3.username, user3.active
    );

    // create struct instance using struct update syntax ..
    let user4 = User {
        email: String::from("user4@example.com"),
        username: String::from("user4"),
        ..user1
    };
    println!(
        "user4.username: {}, active: {}",
        user4.username, user4.active
    );

    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Colour black: {} {} {}", black.0, black.1, black.2);
    println!("Point origin: {} {} {}", origin.0, origin.1, origin.2)
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

/// creates an instance using field init shorthand syntax
fn build_user(email: String, username: String) -> User {
    User {
        email,    // shorthand
        username, // shorthand
        active: true,
        sign_in_count: 1,
    }
}
