use std::sync::Mutex;

fn main() {
    println!("Threads, Mutex...");

    let m = Mutex::new(5);
    println!("m before mutex lock and modify: {:?}", m);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m after mutex lock and modify: {:?}", m);
}
