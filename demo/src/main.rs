use demo::{Inventory, ShirtColor};
mod iterators;
mod ownership;

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {user_pref1:?} gets {giveaway1:?}");

    let giveaway2 = store.giveaway(None);
    println!("The user with no preference gets {giveaway2:?}");

    println!("=====================");

    ownership::test();

    println!("=====================");

    iterators::test();
}
