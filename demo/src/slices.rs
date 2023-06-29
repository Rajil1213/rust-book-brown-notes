pub fn test() {
    //? Slices
    let mut s: String = String::from("hello, world");
    let word = first_word(&s);
    s.clear(); // this line makes the following line meaningless
    println!("Index to the first space is {word}");

    let s = String::from("hello, world");
    let hello = &s[0..5];
    let world = &s[7..12]; // pointer to byte index 7 of `s` with length 5

    println!("first word = {hello}, second word = {world}");

    let s: String = String::from("hello, world");
    let first_word_slice = first_word_as_slice(&s);
    // s.clear(); // this requires a mutable borrow that is not allowed to mix with immutable borrow below
    println!("The first word is: {first_word_slice}");

    //? Slices as fn params
    let s = String::from("hello world");
    println!("First word = {}", first_word_from_slice(&s));
    println!("First word = {}", first_word_from_slice(&s[..]));
    println!("First word = {}", first_word_from_slice(&s[0..7]));

    let s_literal = "hello world";
    println!("First word = {}", first_word_from_slice(&s_literal));
    println!("First word = {}", first_word_from_slice(&s_literal[..]));
    println!("First word = {}", first_word_from_slice(&s_literal[0..7]));

    let non_str = [1, 2, 3, 4, 5];
    let slice = &non_str[1..3]; // slice beginning at index 1 with length (3-1)=2
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    bytes.len()
}

fn first_word_as_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn first_word_from_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
