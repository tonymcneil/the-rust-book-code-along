fn main() {
    println!("Strings..");
    let mut s = String::new();
    println!("empty s: {}", s);
    println!("-----------------------------\n");

    s = "stuff".to_string();
    println!("populated s: {}", s);
    println!("-----------------------------\n");

    s = String::from("another");
    println!("another s: {}", s);
    println!("-----------------------------\n");

    let greeting_v = vec![
        String::from("السلام عليكم"),
        String::from("Dobrý den"),
        String::from("Hello"),
        String::from("שָׁלוֹם"),
        String::from("नमस्ते"),
        String::from("こんにちは"),
        String::from("안녕하세요"),
        String::from("你好"),
        String::from("Olá"),
        String::from("Здравствуйте"),
        String::from("Hola"),
    ];
    println!("multi-lingual greetings...");
    for greeting in greeting_v {
        println!("greeting: {}", greeting);
    }
    println!("-----------------------------\n");

    println!("growing a string with push_str...");
    let mut s1 = String::from("foo");
    let s2 = "bar";
    println!("s1: {}, pushing s2: {}...", s1, s2);
    s1.push_str(s2);
    println!("s1: {} after push of s2: {}", s1, s2);
    println!("-----------------------------\n");

    println!("growing a string with + operator and string slice...");
    let s1 = String::from("Hello, ");
    println!("s1: {}", s1);
    let s2 = String::from("world!");
    println!("s2: {}", s2);
    let s3 = s1 + &s2; // NOTE: s1 has been moved here and can no longer be used
    println!("s3: {} (s1 moved and can't be accessed)", s3);
    println!("-----------------------------\n");

    println!("concatenating strings with format!...");
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s: {}", s);
    println!("-----------------------------\n");

    let _s = String::from("hello tony");
    // let h = _s[0]; // NOTE: can't be indexed

    println!("Slicing into a string [0..4] where each char is 2 x u8 values in Unicode...");
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s: {}", s);

    println!("Iterating over string characters using .chars()...");
    let s = "नमस्ते";
    println!("original string: {}", s);
    println!("each char...");
    for c in s.chars() {
        println!("---> {}", c);
    }
}
