use procedural_derive::HelloMacro;

pub trait HelloMacro {
    fn hello_macro() {
        println!("hello, Macro!");
    }
}

#[derive(HelloMacro)]
struct Pancakes;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_macro_works() {
        Pancakes::hello_macro();
    }
}
