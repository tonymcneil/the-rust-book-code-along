fn main() {
    // loop with break return
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result: {}", result);

    // while
    let mut num = 3;

    while num != 0 {
        println!("T-Minus: {}", num);
        num -= 1;
    }
    println!("LIFTOFF!");

    // for loop
    let a = [1, 2, 3, 4, 5];
    for element in a.iter() {
        println!("element: {}", element);
    }

    // for loop to re-write the liftoff countdown above
    for num in (1..4).rev() {
        println!("T-Minus: {}", num);
    }
    println!("LIFTOFF WITH FOR LOOP!");
}
