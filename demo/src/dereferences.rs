pub fn test() {
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x; // *x reads the heap value, so a = 1
    *x += 1; // *x on the left-side modifies the heap value,
             //     so x points to the value 2
    println!("a = {a}, x = {x}");

    let r1: &Box<i32> = &x; // r1 points to x on the stack
    let b: i32 = **r1; // two dereferences get us to the heap value
    println!("r1 = {r1}, b = {b}");

    let r2: &mut i32 = &mut *x; // r2 points to the heap value directly (aliasing)
    *r2 += 1; // mutate aliased data
    let c: i32 = *r2; // so only one dereference is needed to read it
    println!("r2 = {r2}, c = {c}");

    println!("Value of x after r2's scope ends: {x} ");
}
