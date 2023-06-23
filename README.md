# Programming a Guessing Game

## Introduction

- Skip and go to chapter 3 if you want to learn the syntax first
- Dive in if you prefer learning by failing.
- How does the program work:
    - Generate a random integer between 1 and 1000
    - Prompt the player to enter a guess
    - Indicate whether the guess was too high or too low
    - If the guess is correct, print a congratulatory message and exit

## Setup

- Set up a new project:
    
    ```bash
    cargo new guessing-game
    ## or cargo init (on an existing directory without any files)
    ```
    

## Processing a Guess

- Let’s first get the user input and simply output it:

### The Code

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

<aside>
💡 make sure you have the following rust settings in vscode:

"[rust]": {
"editor.defaultFormatter": "rust-lang.rust-analyzer",
"editor.formatOnSave": true,
},

</aside>

### The Analysis

```rust
use std::io
```

- The first line defines an “import” the `io` package from the `std` library. This is needed to receive user input
- By default, Rust has a set of items defined in the standard library that it brings into scope of every program. This set is called the `prelude`
- If a functionality is not part of the prelude, we need to explicitly bring into the scope of our program with the `use` statement.

```rust
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");
```

- The next three lines define the `main` function and print user instructions to the `stdout`.

```rust
let mut guess = String::new();
```

- Then, we define a  **mutable** variable with the `let mut` keywords, meaning that the value stored in this variable can mutate over the course of our program execution.
- We can also declare a variable with just let which would mean that the variable so defined is immutable and cannot be mutated during the program execution.
- `String::new()` is a function that returns a new instance of a `String`
- `String` is a string type provided by the standard library that is growable, UTF-8 encoded bit of text
- `::` in the `::new` indicates that `new` is an associated function of the `String` type
- An associated function is a function that is implemented on a type, in this case `String`

```rust
io::stdin()
```

- The next line calls the `stdin()` function from the `io` module which allows us to handle user input
    
    <aside>
    💡 if we hadn’t invoked `use`, we could still use `stdin()` with `std::io::stdin()`
    
    </aside>
    

```rust
.read_line(&mut guess)
```

- Next, the line `.read_line(&mut guess)` calls the `read_line` method on the standard input handle to get input from the user.
- `&mut guess` is passed as an argument to this function to tell it what string to store the user input in. The actual function of `read_line` is to append the user input to the end of the specified string type.
- The string, therefore, needs to be mutable
- `&` indicates that this argument is a `reference` allowing multiple parts of your code to access one piece of data without needing to copy that data  into memory multiple times.
- References are immutable by default (just like variables in Rust). Hence, the `mut` keyword is required here.
- The `read_line` method in turn returns a `Result` value.
- `Result` is an enumeration (or `enum`) which is a type that can be in one of multiple possible states. We call each possible state a `variant`.
- A `Result` enum’s variants are `Ok` and `Err`.
- The `Ok` variant indicates that the operation was successful, and inside `Ok` is the successfully generated value.
- The `Err` variant indicates that the operation failed, and `Err` contains information about how or why the operation failed.
- An instance of the `Result` enum has an `expect` method associated with it.

```rust
.expect("Failed to read line");
```

- The `expect` method causes the program to crash (panic) and display the message that you passed as an argument to `expect` in this case “Failed to read line”.
- If the `read_line()` returns an `Err`, it is a likely a result of an error coming from the underlying OS.
- If the `read_line()` returns an `Ok`, `expect` will take the return value that `Ok` is holding and return just that value, in this case, the number of bytes in the user’s input.
- Without this line, the program will compile but you’ll get a warning: `#[warn(unused_must_use)]` ⇒ this `Result` may be an `Err` variant which should be handled.

```rust
println!("You guessed: {guess}")
```

- In the final line, we interpolate the value of `guess` variable in the format string using the `{}` notation.
- If we want to evaluate an expression then, we can do so by using empty curly braces:
    
    ```rust
    let x = 10;
    let y = 20;
    println!("x = {x} and x + y = {}", x + y);
    // prints x = 10 and x + y = 30
    ```
    
    Note that there shouldn’t be any space between the braces and the variable being evaluated.
    

### Running the Program

- Run `cargo run`.
    
    ```
    $ cargo run
    Compiling guessing-game v0.1.0 (~/rust-book-brown-notes)
        Finished dev [unoptimized + debuginfo] target(s) in 1.74s
         Running `target/debug/guessing-game`
    Guess the number!
    Please input your guess.
    10
    You guessed: 1
    ```
    

## Generating a Random Number

### Using a Crate

- To generate a random number we use another crate called `rand` as the required functionality is not yet provided by the standard library
- A crate can be a `binary` crate or a `library` crate
- The program we are building is a `binary` crate
- `rand` is a library crate that can be `use`'d in our program
- We can get the `rand` package by including it in the `[dependencies]` section in the `Cargo.toml` file.
    
    ```toml
    [dependencies]
    rand = "0.8.5"
    ```
    
    We are using the above version according to the Book.
    
- Cargo understands Semantic Versioning or SemVer.
- The above version value is a shorthand for `^0.8.5` which means any version that is at least 0.8.5 but below 0.9.0.
- We can then run `cargo build` to get the `rand` crate and build our binary.
- When we use a dependency in our code, cargo automatically fetches the latest version of that dependency at build time by checking the `registry` which is a copy of `[crates.io](http://crates.io)` which is where the community publishes crates.
- When we include dependencies in the `[dependencies]` section, any crate not already downloaded are downloaded as well.
- Cargo also fetches other dependencies that the dependencies that we include depend upon.
- In this case, Cargo fetches `rand` version `0.8.5` or greater but less than `0.9.0`.
- To make sure that all dependencies remain in sync (for reproducible builds), cargo also creates a lock file with the actual version of dependencies as well as their transitive dependencies, called `Cargo.lock`.
- When we rebuild our project, Cargo looks at the lock file to determine the dependencies instead of going to the trouble of figuring out the dependencies for itself.
- Updating a crate is as simple as: `cargo update`.
- However, this does not update the crate to `0.9.0`. For this, we need to explicitly set the version to `0.9.0`.

### Generating

- We will use the `Rng` `trait` from the `rand` package.
- This trait defines the methods that the `rand` package implements.
    
    ```rust
    use rand::Rng;
    ```
    
- We can then, use the `thread_rng()` method to generate a random number and also specify the range:
    
    ```rust
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");
    ```
    
- Here,
    - `thread_rng()` gives us a random number generator local to the current thread and is seeded by the OS
    - `gen_range()` takes a range expression as an argument and generates a random number in the range
    - `1..=100` is the range expression of the form `start..=end` that is inclusive on the lower and upper bounds

<aside>
💡 We can view the documentation for all the crates our program uses by building a documentation server locally with `cargo doc --open`.

</aside>

## Comparing the Guess to the Secret Number

### Ordering

- To compare the guess with the random number generated, we will be bringing `std::cmp:Ordering` into our scope.
- The `Ordering` type is an `enum` and has the variants:
    - `Less`
    - `Greater`
    - `Equal`
    
    These are the three outcomes that are possible when you compare two values
    
    ```rust
    use std::cmp::Ordering;
    ...
    
    match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => println!("You win"),
            Ordering::Greater => println!("Too big"),
        }
    ```
    
- We use a `match` expression to evaluate the three possibilities.
- A `match` expression is made up of arms
- An *arm* consists of an expression to match against and the code that should be run if the value given to the `match` fits that arm’s pattern.
- Rust requires that you handle every possibility inside a match expression

### Conversion

- The above code, however fails to compile since the `guess` and the `secret_number` are of different types!
- The `guess` received from the user input is a `String` while the `secret_number` is a number specifically an `i32` (a 32-bit signed integer that Rust defaults to and into which any number from 1 to 100 fits).
- So, we need to convert the `guess` to a number. For this we, will use certain methods available on the `String` type.
    
    ```rust
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: i32 = guess.trim().parse().expect("Please type a number");
    ```
    
- Here, we are redefining/shadowing `guess`. This is acceptable in Rust. However, a good practice is to only use this for conversions like the above.
- The `trim()` method on the `String` instance will eliminate any whitespace at the beginning and end.
- This also trims the `\n` (newline character) at the end of the guess that the user inputs (in Linux, or equivalently `\r\n` in Windows).
- The `parse()` method converts the `String` to another type. Here, the target type is decided by the type annotation on the target variable, in this case `i32`.
- The `parse()` method may not always work since a string cannot always converted to another type.
- So, the method returns a `Result` type just like `read_line()` and has to be handled with an `expect()` method similarly.
- We can now run the program with `cargo run`

## Refining

- We can allow multiple guess with a loop and `break` the loop if the guess is correct
    
    ```rust
      	let total_guesses = 5;
        for num_guesses in 1..=total_guesses {
    				...
    				match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small"),
                Ordering::Greater => println!("Too big"),
                Ordering::Equal => {
                    println!("You win");
                    break;
                }
            }
            println!("Remaining guesses: {}", total_guesses - num_guesses);
    		}
    ```
    
- We can repeatedly ask for input if the input guess is not valid (instead of `expect`-ing that panics if the Result is an `Err`)
- The final program looks like this:
    
    ```rust
    use std::io;
    
    use rand::Rng;
    use std::cmp::Ordering;
    
    fn main() {
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
    
            let guess: i32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number");
                    continue;
                }
            };
    
            match guess.cmp(&secret_number) {
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
    ```
