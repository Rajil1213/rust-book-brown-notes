pub fn test() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // create and store iterator

    for val in v1_iter {
        // loop through the values in the iterator
        println!("Got: {val}");
    }
}
