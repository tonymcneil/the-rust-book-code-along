fn main() {
    println!("Iterators...");
    println!("-----------------------------------------\n");

    println!("using iter method on vec...");
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("val: {}", val);
    }
    println!("-----------------------------------------\n");

    println!("accessing the iterator implicitly on vec (also via reference borrow)...");
    for val in &v1 {
        println!("val: {}", val);
    }
    println!("-----------------------------------------\n");

    println!("testing the next() method in the Iterator trait...");
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
    println!("assertions passed if this line prints");
    println!("-----------------------------------------\n");

    println!("testing the sum() method in the Iterator trait...");
    v1_iter = v1.iter(); // NOTE: re-assigning to reset the iterator state
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
    println!("assertions passed if this line prints");
    println!("-----------------------------------------\n");

    println!("testing the map() method in the Iterator trait...");
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
    println!("-----------------------------------------\n");

    println!(
        "closure passed to filter() method in the Iterator trait capturing it's environment..."
    );
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        // NOTE: closure passed to filter captures it's environment i.e. shoe_size
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
    filters_by_size();
    println!("assertions passed if this line prints");
    println!("-----------------------------------------\n");

    println!("creating an iterator that counts to 5...");
    struct FiveCounter {
        count: u32,
    }

    impl FiveCounter {
        fn new() -> FiveCounter {
            FiveCounter { count: 0 }
        }
    }

    impl Iterator for FiveCounter {
        type Item = u32;
        fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let mut five_counter = FiveCounter::new();
    assert_eq!(five_counter.next(), Some(1));
    assert_eq!(five_counter.next(), Some(2));
    assert_eq!(five_counter.next(), Some(3));
    assert_eq!(five_counter.next(), Some(4));
    assert_eq!(five_counter.next(), Some(5));
    assert_eq!(five_counter.next(), None);
    println!("assertions passed if this line prints");
    println!("-----------------------------------------\n");

    println!("chaining Iterator trait methods...");
    let sum: u32 = FiveCounter::new()
        .zip(FiveCounter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
    println!("assertions passed if this line prints");
    println!("-----------------------------------------\n");

}
