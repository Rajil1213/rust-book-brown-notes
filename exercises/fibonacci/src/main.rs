use std::io;

fn main() {
    let mut last_two: [u128; 2] = [0, 1];

    let mut n = String::new();

    println!("generating nth fibonacci number, enter value of n: ");
    io::stdin().read_line(&mut n).expect("unable to read line");

    // usize necessary for array indexing
    let n: usize = n
        .trim()
        .parse()
        .expect("please enter a valid positive number");

    if n <= 2 {
        println!("Fibonacci Number {n} = {}", last_two[n.saturating_sub(1)]);
        return;
    }

    let mut new: u128;
    for _ in 2..=n {
        new = last_two[0] + last_two[1];
        last_two[0] = last_two[1];
        last_two[1] = new;
    }

    println!("Fibonacci Number {n} = {}", last_two[1]);
}
