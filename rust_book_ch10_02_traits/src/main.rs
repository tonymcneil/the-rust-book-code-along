use core::fmt::Display;

fn main() {
    println!("Traits...");

    println!("Tweet struct implementing Summary trait...");
    let tweet = Tweet {
        username: String::from("tony"),
        content: String::from("top of the morning to you all!"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("----------------------------------------\n");

    println!("NewsArticle struct implementing Summary trait...");
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize_default());
    println!("----------------------------------------\n");

    println!("Calling function that takes a param that implements a trait...");
    notify(&tweet);
    println!("----------------------------------------\n");

    println!("Calling function that takes a param that implements multiple traits...");
    let list_i32 = vec![1, 2, 3, 4];
    println!("largest in: {:?} is: {:?}", list_i32, largest(&list_i32));
    let list_char = vec!['a', 'e', 'i', 'o', 'u'];
    println!("largest in: {:?} is: {:?}", list_char, largest(&list_char));
    println!("----------------------------------------\n");

    println!("Calling function that's only implemented for types implementing specific traits...");
    let pair_i32 = Pair::new(4, 5);
    println!("pair_i32: {:?}...", pair_i32);
    pair_i32.cmp_display();
    println!("----------------------------------------\n");
}

pub fn notify(item: &impl Summary) {
    println!("Utilising the Summary trait method now...");
    println!("Breaking news! {}", item.summarize());
}

pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_default(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// fn largest<T>(list: &[T]) -> T
// where
//     T: PartialOrd + Copy,
// {
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    // Copy trait needed for copy of item form list
    for &item in list {
        // PartialOrd trait needed for > comparison
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
