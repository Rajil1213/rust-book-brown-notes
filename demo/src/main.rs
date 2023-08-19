use std::{thread, time::Duration};
mod channels;
mod mutexes;
mod threads;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from main thread!");
        thread::sleep(Duration::from_millis(2));
    }

    handle.join().unwrap();

    threads::test().join().unwrap();

    channels::test();

    mutexes::test();
}
