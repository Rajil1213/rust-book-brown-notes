fn main() {
    let mut x = 6;
    println!("The value of x = {x}");
    x = 5;
    println!("The value of x = {x}");

    // shadowing
    let y = 10;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y inside this scope is: {y}");
    }

    println!("The value of y outside is: {y}");

    // shadowing types
    let spaces = "    ";
    let spaces = spaces.len();

    println!("The number of spaces is {spaces}");
}
