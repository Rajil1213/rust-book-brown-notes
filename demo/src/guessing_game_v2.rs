use std::io;

use rand::Rng;
use std::cmp::Ordering;

struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}");
        }

        Guess { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

pub fn start() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut num_guesses = 5;
    loop {
        println!("Remaining guesses: {}", num_guesses);
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        let guess = Guess::new(guess);

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }

        num_guesses -= 1;
        if num_guesses == 0 {
            println!("Uh-oh! No more guesses remaining. You lose.");
            break;
        }
    }
}
