pub fn test_conditions() {
    let a = 1;
    let b = 2;
    let greater_or_equal = if a >= b { a } else { b };

    println!("{a} vs {b}, greater or equal = {greater_or_equal}")
}
