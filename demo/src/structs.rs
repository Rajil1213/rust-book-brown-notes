pub struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    pub fn x(&self) -> &X1 {
        &self.x
    }

    pub fn y(&self) -> &Y1 {
        &self.y
    }

    // here, we use <X2, Y2> instead of <X1, Y1> since those are the ones that need defining
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f64, f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
