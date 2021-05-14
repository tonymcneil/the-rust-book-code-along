use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Threads...");
    let (tx, rx) = mpsc::channel();

    println!("Note the spawned thread will be stopped when the main thread ends, if the returned JoinHandle is not joined from the main thread...");
    thread::spawn(move || {
        for i in 1..10 {
            println!("hi number: {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
        tx.send(String::from("spawned thread finished!")).unwrap();
    });

    for i in 1..5 {
        println!("hi number: {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // NOTE: this call will block on the main thread until the spawned thread finishes
    // handle.join().unwrap();

    let mut msg = rx.try_recv();
    while msg.is_err() {
        println!("main thread waiting on spawned thread...");
        thread::sleep(Duration::from_millis(1));
        msg = rx.try_recv();
    }

    println!(
        "spawned thread finished with message: {}, main thread now resumed",
        msg.unwrap()
    );
    println!("--------------------------------------------------------\n")
}
