use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread 1: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Main Thread: {i}");
        thread::sleep(Duration::from_millis(1));
    }

    let vecs = vec![1, 2, 3];

    let handle2 = thread::spawn(move || {
        println!("vector content: {:?}", vecs);
    });

    // this makes sure that the thread runs completely before procedding the code.
    handle.join().unwrap();
}
