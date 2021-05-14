use std::thread;
use std::time::Duration;

fn main() {
    println!("Closures..");

    println!("closure with 1 param..");
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_millis(500));
        num
    };
    expensive_closure(1);
    println!("---------------------------------------\n");

    println!("closure returning its arg...");
    let self_closure = |x| x;
    println!("Result of (self_closure)(42): {}", (self_closure)(42));
    println!("Result of self_closure(21): {}", self_closure(21));
    println!("---------------------------------------\n");

    println!("equlivant definitions...");
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    };
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;
    println!("add_one_v1(1): {}", add_one_v1(1));
    println!("add_one_v2(1): {}", add_one_v2(1));
    println!("add_one_v3(1): {}", add_one_v3(1));
    println!("add_one_v4(1): {}", add_one_v4(1));
    println!("---------------------------------------\n");

    println!("define a closure in a struct, note the calling style...");
    struct Whatever<T>
    where
        T: Fn(u32) -> u32,
    {
        closure: T,
    }
    let what = Whatever { closure: |x| x + 1 };
    println!("(what.closure)(1): {}", (what.closure)(1));
    println!("---------------------------------------\n");

    println!(
        "Closure declaration types:\n\
         * FnOnce - consumes the variables it captures, by taking ownership\n\
         * FnMut - mutably borrows values from environment\n\
         * Fn - borrows values from the environment immutably."
    )
}
