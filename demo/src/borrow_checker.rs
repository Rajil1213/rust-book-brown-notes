pub fn test() {
    let mut data: Vec<i32> = vec![1, 2, 3]; // data:RWO
    println!("The vec before pushing an element: {:?}", data);

    let num: &i32 = &data[2]; // data:R, num:RO, *num:R
    println!("The third element in the Vec is {}", *num); // data: RWO, num: null, *num: null

    data.push(4); // data: RWO
    println!("The vec after pushing an element: {:?}", data); // data: null

    let mut x: Vec<char> = vec!['a', 'b', 'c'];
    println!("vec before captitalization = {:?}", x);
    ascii_capitalize(&mut x);
    println!("vec after captitalization = {:?}", x);

    let v1 = vec![1, 2, 3]; //v1: RWO
    let mut v2 = v1; // v2: RWO, v1: no permissions
    v2.push(4); // v2: RWO, v1: no permissions
                // println!("{}", v1[0]); // invalid ref
    println!("v2 = {:?}", v2);
}

// the fn takes a non-owning, mutable ref to `v`
// v: RO, *v: RW
fn ascii_capitalize(v: &mut Vec<char>) {
    // c borrows v[0], making v immutable and taking ownership
    let c = &v[0]; // v: R, *v: R, c:RO

    if c.is_ascii_lowercase() {
        // after this call, c's lifetime ends releasing the ownership to `v`
        // v: R), *v: RW, up: RO
        let up = c.to_ascii_uppercase();

        // at this point, we can mutate *v
        v[0] = up; // this is the same as (*v)[0]
    } else {
        println!("Already capitalized: {:?}", v);
    }
}
