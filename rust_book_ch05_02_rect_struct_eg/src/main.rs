#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    ///////////////////////////////////////////
    let rect1 = Rectangle {
        width: 50,
        height: 20,
    };
    println!("rect1 area: {}", rect1.area());
    println!("rect1: {:#?}", rect1);

    ///////////////////////////////////////////
    let rect2 = Rectangle {
        width: 10,
        height: 4,
    };

    let rect3 = Rectangle {
        width: 30,
        height: 40,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    ///////////////////////////////////////////
    let square = Rectangle::square(30);
    println!("square area: {}", square.area());
    println!("square: {:#?}", square);
}
