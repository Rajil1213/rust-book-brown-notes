use core::fmt;
use std::{fmt::Display, ops::Add};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}

trait Pilot {
    fn fly(&self) -> String;
}

trait Wizard {
    fn fly(&self) -> String;
}

struct Human;

impl Pilot for Human {
    fn fly(&self) -> String {
        String::from("This is your captain speaking!")
    }
}

impl Wizard for Human {
    fn fly(&self) -> String {
        String::from("Up!")
    }
}

impl Human {
    fn fly(&self) -> String {
        String::from("*waves arms furiously*")
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// Super traits
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

// Newtype pattern
struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn adding_points_works() {
        assert_eq!(
            Point { x: 1, y: 2 } + Point { x: 2, y: 1 },
            Point { x: 3, y: 3 }
        );
    }

    #[test]
    fn adding_meters_to_millimeters_works() {
        assert_eq!(Millimeters(2000) + Meters(2), Millimeters(4000));
    }

    #[test]
    fn same_named_methods_work() {
        let human: Human = Human {};

        assert_eq!(human.fly(), "*waves arms furiously*".to_string());
        assert_eq!(
            Pilot::fly(&human),
            "This is your captain speaking!".to_string()
        );
        assert_eq!(Wizard::fly(&human), "Up!".to_string());
    }

    #[test]
    fn same_named_methods_without_self_work() {
        assert_eq!(Dog::baby_name(), "Spot");
        assert_eq!(<Dog as Animal>::baby_name(), "puppy");
    }

    #[test]
    fn supertrait_works() {
        let p = Point { x: 2, y: 3 };
        p.outline_print();
    }

    #[test]
    fn newtype_works() {
        let wrapper_vec = Wrapper(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        println!("{wrapper_vec}");
    }
}
