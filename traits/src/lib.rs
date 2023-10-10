use std::ops::Add;

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
}
