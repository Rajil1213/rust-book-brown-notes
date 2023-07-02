use crate::rectangle::Rect;

mod rectangle;
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut p = Point { x: 0, y: 0 };
    let x = &mut p.x;

    *x += 1;

    println!("{}, {}", p.x, p.y);

    println!("------------------------------");

    let scale = 2;
    let rect = &Rect {
        height: dbg!(10u32 * scale),
        width: 20u32,
    };

    let area = rectangle::calculate_area(rect);
    println!("Area of the rectange {:?} = {area}", rect);
}
