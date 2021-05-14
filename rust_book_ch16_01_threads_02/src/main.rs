use std::thread;
use std::time::Duration;

fn main() {
    println!("Threads...");

    println!("Note the spawned thread will be stopped when the main thread ends, if the returned JoinHandle is not joined from the main thread...");
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number: {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number: {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // NOTE: this call will block on the main thread until the spawned thread finishes
    handle.join().unwrap();
    println!("spawned thread finished, main thread now resumed");
    println!("--------------------------------------------------------\n")
}
