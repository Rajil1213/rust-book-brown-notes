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

    // unsugarring for...in
    let mut iter: std::slice::Iter<'_, i32> = v.iter();
    let n1: &i32 = iter.next().unwrap(); // get the next value and unwrap the option to get the Some value
    let n2: &i32 = iter.next().unwrap();
    let n3: &i32 = iter.next().unwrap();
    let n4: &i32 = iter.next().unwrap();
    println!("{},\n{},\n{},\n{},", n1, n2, n3, n4);

    let end: Option<&i32> = iter.next(); // this is an option type

    match end {
        Some(val) => println!("End value = {val}"),
        None => println!("Reached the end of the vector"),
    }

    // from quiz#2
    let mut v = vec![1, 2, 3];
    let mut v2: Vec<&mut i32> = Vec::new();

    for i in &mut v {
        v2.push(i);
    }

    *v2[0] = 5;
    let a = *v2[0];
    let b = v[0];

    println!("a = {a}, b = {b}");
}
