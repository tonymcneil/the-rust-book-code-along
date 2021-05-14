fn main() {
    ////////////////////////////////////////////////
    let ip_v4 = IpAddrKind::V4;
    let ip_v6 = IpAddrKind::V6;
    println!("Example 1: v4: {:?}, v6: {:?}", ip_v4, ip_v6);

    ////////////////////////////////////////////////
    let ip_v4 = IpAddrDumb {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let ip_v6 = IpAddrDumb {
        kind: IpAddrKind::V4,
        address: String::from("::1"),
    };
    println!("Example 2: v4: {:?}, v6: {:?}", ip_v4, ip_v6);

    ////////////////////////////////////////////////
    let ip_v4 = IpAddrBetter::V4(String::from("127.0.0.1"));
    let ip_v6 = IpAddrBetter::V6(String::from("::1"));
    println!("Example 3: v4: {:?}, v6: {:?}", ip_v4, ip_v6);

    ////////////////////////////////////////////////
    let ip_v4 = IpAddrBetterAltTypes::V4(127, 0, 0, 1);
    let ip_v6 = IpAddrBetterAltTypes::V6(String::from("::1"));
    println!("Example 4: v4: {:?}, v6: {:?}", ip_v4, ip_v6);

    ////////////////////////////////////////////////
    println!("Example 5: omitted, NOTE: use the standard lib IpAddr implementation for actual apps");


    ////////////////////////////////////////////////
    let message = Message::Write(String::from("my message"));
    print!("Example 6:... ");
    message.call();

    ////////////////////////////////////////////////
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!("Example 7: some_number: {:?}, some_string: {:?}, absent_number: {:?}", some_number, some_string, absent_number);

}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddrDumb {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrBetter {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrBetterAltTypes {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[allow(dead_code)]
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("call on Message: {:?}", self)
    }
}
