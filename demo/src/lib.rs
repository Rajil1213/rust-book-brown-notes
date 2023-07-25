#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn panic_if_greater(input: i32, limit: i32) -> String {
    if input > limit {
        panic!("Input cannot be greater than {limit}, received {input}");
    }

    format!("Congratulations, you stayed under the limit, received {input}!")
}

pub fn adder(left: i32, right: i32) -> i32 {
    left + right + left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 7,
            height: 6,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 7,
            height: 6,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // should return "Hello, Carol"
        // greeting (`Hello`) may change, so only test input
        assert!(
            result.contains("Carol"),
            "Greeting did not contain the name, value was {}",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Input cannot be greater than")]
    fn greater_than_limit() {
        let value = 60;
        let limit = 40;

        panic_if_greater(value, limit);
    }

    #[test]
    fn adder_works() -> Result<(), String> {
        if adder(2, 3) == 5 {
            Ok(())
        } else {
            Err(String::from(format!(
                "two plus three does not equal five, got {}!",
                adder(2, 3)
            )))
        }
    }
}
