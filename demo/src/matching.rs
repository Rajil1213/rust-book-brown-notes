#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    // and others
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    // as opposed to `if` `match` can operate on any type
    match coin {
        Coin::Penny => 1,  // this is an arm
        Coin::Nickel => 5, // `=>` separates the arm and the code to run
        Coin::Dime => 10,  // one arm is separated from the other with a comma
        Coin::Quarter(state) => {
            println!("You've got a quarter from {:?}!", state);
            25
        } // code associated with each arm is an expression which "can" be wrapped in curly braces as well
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(value) => Some(value + 1),
    }
}

pub fn test() {
    let coin = Coin::Quarter(UsState::Arizona);
    let cents = value_in_cents(&coin);

    println!("{:?} = {cents} cents", coin);

    println!("----------------------------------------");
    let five = Some(5);
    let five_plus_one = plus_one(five);
    let none = plus_one(None);

    println!("five plus one = {:?}", five_plus_one);
    println!("none plus one = {:?}", none);
}
