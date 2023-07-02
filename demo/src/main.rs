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

    let another_rect = &Rect {
        height: 10,
        width: 5,
    };

    let area = rectangle::calculate_area(rect);
    println!("Area of the rectange {:?} = {area} (from function)", rect);
    println!(
        "Area of the rectange {:?} = {} (from method)",
        rect,
        rect.area()
    );

    println!(
        "{:?} can hold {:?} ? {}",
        rect,
        another_rect,
        rect.can_hold(another_rect)
    );
    println!(
        "{:?} can hold {:?} ? {}",
        another_rect,
        rect,
        another_rect.can_hold(rect)
    );

    let square_from_rect = Rect::square(10);
    println!(
        "Area of the square {:?} is {}",
        square_from_rect,
        square_from_rect.area()
    );
}
