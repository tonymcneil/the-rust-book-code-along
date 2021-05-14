use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("Threads, Mutex, Sharing a Mutex<T> Between Multiple Threads...");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 1..11 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            println!("thread: {}, num before modification: {}", i, *num);
            *num += 1;
            println!("thread: {}, num after modification: {}", i, *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
