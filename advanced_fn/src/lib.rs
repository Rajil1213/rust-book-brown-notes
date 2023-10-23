pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn do_twice_works() {
        let x = 0;
        assert_eq!(2, do_twice(add_one, x));
    }
}
