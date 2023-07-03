use crate::Rect;

pub fn test() {
    let mut rect1 = Rect {
        width: 15,
        height: 25,
    };

    println!("setting width to of rect1 to 30");
    rect1.set_width(30); // this is OK

    // let rect1_ref = &rect1;
    // rect1_ref.set_width(20); // this is NOT OK

    println!("Area = {}", rect1.area());

    let rect2 = Rect {
        width: 20,
        height: 10,
    };

    println!("Max Rect = {:?}", rect1.max(rect2));

    // println!("Rect1 = {:?}", rect1); // this is NOT okay as ownership moved by `max`

    println!("------------------------------------");

    let mut rect = Rect {
        width: 0,
        height: 1,
    };
    let other_rect = Rect {
        width: 1,
        height: 0,
    }; // L1

    rect.set_to_max(other_rect); // L3
    println!("rect after setting to max = {:?}", rect);

    println!("other_rect after setting rect to max = {:?}", other_rect);
}
