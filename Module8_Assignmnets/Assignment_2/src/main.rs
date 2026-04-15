use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for thread_id in 1..=5 {
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            for step in 1..=10 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;

                println!("Thread {} at step {}", thread_id, step);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
}