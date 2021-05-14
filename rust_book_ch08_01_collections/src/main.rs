fn main() {
    println!("Common collections...");

    ///////////
    // vectors
    ///////////

    // must add type declaration as not inserting any values for rust to infer type
    println!("declaration specifying type...");
    let _v: Vec<i32> = Vec::new();
    println!("Vec _v: {:?}", _v);
    println!("-------------------------------\n");

    // using the vec! macro rust can infer the type
    println!("declaration with infered type...");
    let v = vec![1, 2, 3];
    println!("Vec v: {:?}", v);
    println!("-------------------------------\n");

    // note type declaration is not needed but can be used here (rust could infer type, from push)
    // also must be declared mutable
    println!("declaration with inferred type via most specific type given...");
    let mut v = Vec::new();
    v.push(1);
    v.push(2i8); // most specific type dec implies type
    v.push(3);
    println!("Vec v: {:?}", v);
    println!("now popping a val: {:?}", v.pop());
    println!("Vec v after pop: {:?}", v);
    println!("-------------------------------\n");

    {
        println!("vector going out of scope with implicit Drop...");
        let _v = vec![1, 2, 3, 4];
        println!("Vec _v: {:?}", v);
    } // <- v goes out of scope and is freed here (has implications for references to elements)
    println!("-------------------------------\n");

    // ways to access elements...
    println!("accessing vector elements...");
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    println!("-------------------------------\n");

    println!("TODO: understand reference usage and checking.. ");
    println!("Listing 8-7: https://doc.rust-lang.org/book/ch08-01-vectors.html");
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {}", first);
    println!("-------------------------------\n");

    println!("iterating over vector values...");
    let v = vec![10, 20, 30];
    for i in v {
        println!("i: {}", i);
    }
    println!("-------------------------------\n");

    println!("iterating and mutating vector elements...");
    let mut v = vec![10, 20, 30];
    println!("v before: {:?}", v);
    for i in &mut v {
        *i *= 2;
        println!("i * 2: {}", i);
    }
    println!("v after mutation: {:?}", v);
    println!("-------------------------------\n");

    println!("storing various types in vector using a multi type enum...");

    #[derive(Debug)]
    enum Cell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Cell::Int(42),
        Cell::Float(3.14),
        Cell::Text(String::from("tony")),
    ];
    println!("row: {:?}", row);
    println!("-------------------------------\n");



}
