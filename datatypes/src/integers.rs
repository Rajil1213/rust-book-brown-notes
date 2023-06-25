pub fn overflow() {
    const NUM1: u8 = 10;
    const NUM2: u8 = 240;

    // let total: u8 = NUM1 + NUM2; // compilation error
    let total: u8 = NUM1.wrapping_add(NUM2);
    println!("{NUM1} + {NUM2} = {total}");

    // return None if overflow occurred
    let total1: Option<u8> = NUM1.checked_add(NUM2);

    match total1 {
        None => println!("{NUM1} + {NUM2} overflowed"),
        Some(total) => println!("{NUM1} + {NUM2} = {total}"),
    }

    // check if overflow occurred
    let total2: (u8, bool) = NUM1.overflowing_add(NUM2);
    println!(
        "{NUM1} + {NUM2} = {} with overflow = {}",
        total2.0, total2.1
    );

    // saturate min-max values
    let total3 = NUM1.saturating_add(NUM2);
    println!("{NUM1} + {NUM2} = {total3}");
}
