#[derive(Debug)]
pub struct Rect {
    pub width: u32,
    pub height: u32,
}

// methods are defined within this impl block
impl Rect {
    // &self is short for self: &self
    // self is an alias for the type impl block is for
    // in this case `Rect`
    pub fn area(&self) -> u32 {
        self.height * self.width
    }

    pub fn can_hold(&self, other: &Rect) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

impl Rect {
    // associated fn that is **not** a method
    pub fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn calculate_area(rect: &Rect) -> u32 {
    rect.height * rect.width
}
