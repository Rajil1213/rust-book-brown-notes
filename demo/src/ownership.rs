use std::thread;

pub fn test() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling immutable closure: {list:?}");
    only_borrows();
    println!("After calling immutable closure: {list:?}");

    let mut borrows_mutably = || list.push(4);
    borrows_mutably();
    println!("After calling mutable closure: {list:?}");

    println!("Before defining moving closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
