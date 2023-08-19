use std::{sync::mpsc, thread};

pub fn test() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // can't do this now:
        // println!("Val is {val}");
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
