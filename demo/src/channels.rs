use std::{sync::mpsc, thread, time::Duration};

pub fn test() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // can't do this now:
        // println!("Val is {val}");
    });

    for received in rx {
        println!("Got: {received}");
    }
}
