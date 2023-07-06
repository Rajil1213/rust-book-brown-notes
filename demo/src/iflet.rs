use crate::matching::{Coin, UsState};

pub fn test() {
    let coins: [Coin; 5] = [
        Coin::Quarter(UsState::Alaska),
        Coin::Dime,
        Coin::Penny,
        Coin::Nickel,
        Coin::Quarter(UsState::Alabama),
    ];
    let mut count = 0;

    for coin in coins.iter() {
        if let Coin::Quarter(state) = coin {
            println!("The quarter is from {:?}", state);
        } else {
            count += 1
        }
    }

    println!("Number of non-quarters = {count}");
}
