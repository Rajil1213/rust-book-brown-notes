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
}
