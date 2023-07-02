#[derive(Debug)]
pub struct Rect {
    pub width: u32,
    pub height: u32,
}

pub fn calculate_area(rect: &Rect) -> u32 {
    rect.height * rect.width
}
