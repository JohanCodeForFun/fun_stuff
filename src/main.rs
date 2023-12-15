use std::sync::{Arc, Mutex};
use std::thread;

fn send_strings() {
    let vals = vec![
        String::from("hi 1"),
        String::from("hi 2"),
        String::from("hi 3"),
    ];
}

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap())
}
