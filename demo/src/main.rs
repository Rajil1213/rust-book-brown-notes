fn main() {
    //? Primitives:
    let a = 5;
    let b = a;

    // both okay:
    println!("The original integer = {a}");
    println!("The new integer = {b}");

    //? Dynamic data:
    let s1 = String::from("hello");
    let s2 = s1;

    // not okay:
    // println!("The original string: {}", s1);

    // okay:
    println!("The new string: {}", s2);

    //? Functions on heap-allocated data type:
    let some_string = String::from("world");
    takes_ownership(some_string);
    // not okay:
    // println!("some_string in callee: {}", some_string);

    //? Functions on stack-allocatd data type:
    let some_integer = 10;
    makes_copy(some_integer);
    // okay:
    println!("some_integer in callee: {some_integer}");

    //? Functions that return ownership on heap-allocated data type:
    let another_string: String = String::from("hello, world");
    let another_string: String = takes_and_gives_back(another_string);
    // okay:
    println!("another_string that was returned = {}", another_string);

    //? Compute and Return
    let yet_another_string: String = String::from("hello, again");
    let (yet_another_string, len) = compute(yet_another_string);
    println!("The length of '{yet_another_string}' is {len}");
}

fn takes_ownership(some_string: String) {
    println!("Taking ownership of {}", some_string);
} // Drop is called here

fn makes_copy(some_integer: i32) {
    // a copy is passed here
    println!("Making copy of {some_integer}");
}

fn takes_and_gives_back(another_string: String) -> String {
    println!("Taking ownership of '{another_string}' and giving it back");
    another_string // return ownership to callee
} // Drop is not called since ownership has been moved

fn compute(yet_another_string: String) -> (String, usize) {
    let length = yet_another_string.len();
    (yet_another_string, length)
}
