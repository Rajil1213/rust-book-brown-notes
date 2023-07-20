fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

pub fn test() {
    let longer_string;
    let string1 = String::from("hello,");
    let string2 = String::from("world");

    longer_string = longer(&string1, &string2);
    println!("The longer string is {longer_string}");

    let novel = String::from("it was the best of time, it was the worst of ...");
    let first_part = novel.split(',').next().expect("could not find a ','");
    let i = ImportantExcerpt { part: first_part };

    println!("First part = {}", i.part);
}
