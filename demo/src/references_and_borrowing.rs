pub fn test() {
    //? passing references
    let some_string: String = String::from("hello, world");
    let length = calculate_length_on_ref(&some_string);
    println!("Length of '{some_string}' is {length}");

    //? passing mutable references
    let mut some_string: String = String::from("hello");
    println!("string before change: {some_string}");
    change_ref(&mut some_string);
    println!("string after change: {some_string}");

    //? scoping multiple mutable referencs:
    let mut s = String::from("hello");
    {
        let ref1 = &mut s;
        println!("Mutable ref1 = {}", ref1);
    }
    let ref2 = &mut s;
    println!("Mutable ref2 = {}", ref2);

    //? mixing mutable and immutable references
    let mut s: String = String::from("hello");

    let ref1 = &s;
    let ref2 = &s;
    println!("Mutable references: {} and {}", ref1, ref2);

    let ref3 = &mut s;
    println!("Immutable reference: {}", ref3);

    //? dangling reference
    // let dangling_ref = dangle();
}

fn calculate_length_on_ref(s: &String) -> usize {
    s.len()
}

fn change_ref(s: &mut String) {
    s.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
