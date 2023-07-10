pub fn test() {
    let mut v = vec![1, 2, 3];

    v.push(4);

    let fourth = &v[3];
    println!("The fourth element via index is {fourth}");

    let third = v.get(2);
    match third {
        Some(third) => println!("The third element via `get` is {third}"),
        None => println!("There is no third element :("),
    }
}
