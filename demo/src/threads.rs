use std::thread::{self, JoinHandle};

pub fn test() -> JoinHandle<()> {
    let v = vec![1..5];

    thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    })
}
