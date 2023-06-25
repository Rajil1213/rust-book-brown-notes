fn five() -> i32 {
    5
}

fn plus_one(input: i32) -> i32 {
    input + 1
}

pub fn test_functions() {
    let x = five();
    println!("five() -> {x}");
    println!("plus_one(five()) -> {}", plus_one(x))
}
