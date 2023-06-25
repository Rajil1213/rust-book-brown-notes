use std::io;

fn main() {
    let mut temp_fahrenheit = String::new();
    const CONVERSION_FACTOR: f64 = 5.0 / 9.0;

    println!("Enter the temperature in Fahrenheit");
    io::stdin()
        .read_line(&mut temp_fahrenheit)
        .expect("failed to read line");

    let temp_fahrenheit: f64 = temp_fahrenheit
        .trim()
        .parse()
        .expect("please enter a valid floating-point number");

    let temp_celsius = CONVERSION_FACTOR * (temp_fahrenheit - 32.0);

    println!("{temp_fahrenheit}F = {temp_celsius:.2}C");
}
