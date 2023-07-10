pub fn test() {
    let mut v = vec![1, 2, 3];

    v.push(4);

    let fourth = &v[3];
    // v.push(5) // would not work here as mutable borrow occurs here to potentially reallocate the vector
    println!("The fourth element via index is {fourth}");

    let third = v.get(2);
    match third {
        Some(third) => println!("The third element via `get` is {third}"),
        None => println!("There is no third element :("),
    }

    // iterating through a vector, immutably
    for v_ref in &v {
        let value_plus_one = *v_ref + 1;
        println!("v_ref + 1 = {v_ref} + 1 = {value_plus_one} ");
    }

    // iterating through a vector, mutably
    for v_ref in &mut v {
        print!("v_ref + 10 = {v_ref} + 10 = ");
        *v_ref += 10;
        println!("v_ref = {v_ref}");
    }
}
