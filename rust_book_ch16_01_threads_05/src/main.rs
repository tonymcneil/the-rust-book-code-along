use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Threads, Creating Multiple Producers by Cloning the Transmitter...");
    let (tx01, rx) = mpsc::channel();
    // NOTE: cloning tx for later...
    let tx02 = mpsc::Sender::clone(&tx01);


    println!("Spawning thread using tx channel: tx01...");
    thread::spawn(move || {
        let vals = vec![
            String::from("tx01 A"),
            String::from("tx01 B"),
            String::from("tx01 C"),
            String::from("tx01 D"),
        ];

        for val in vals {
            tx01.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Spawning thread using tx channel: tx02...");
    thread::spawn(move || {
        let vals = vec![
            String::from("tx02 A"),
            String::from("tx02 B"),
            String::from("tx02 C"),
            String::from("tx02 D"),
        ];

        for val in vals {
            tx02.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
