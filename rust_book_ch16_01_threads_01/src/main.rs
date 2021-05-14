use std::thread;
use std::time::Duration;

fn main() {
    println!("Threads...");

    println!("Note the spawned thread will be stopped when the main thread ends...");
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number: {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number: {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    println!("NOTE: we probably didn't get to count to all the numbers in the spawned thread before the main thread ended and thus stopped the spawned thread!");
    println!("--------------------------------\n");

}
