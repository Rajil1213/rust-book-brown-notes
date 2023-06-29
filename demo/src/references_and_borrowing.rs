pub fn test() {
    let some_string: String = String::from("hello, world");
    let length = calculate_length_on_ref(&some_string);
    println!("Length of '{some_string}' is {length}");
}

fn calculate_length_on_ref(s: &String) -> usize {
    s.len()
}
