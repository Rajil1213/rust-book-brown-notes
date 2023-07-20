fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn test() {
    let longer_string;
    let string1 = String::from("hello,");
    let string2 = String::from("world");

    longer_string = longer(&string1, &string2);
    println!("The longer string is {longer_string}");
}
