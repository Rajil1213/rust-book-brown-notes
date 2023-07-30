# Project: Building a Command-Line Program
## Introduction

- We’ll make our own version of the classic program `grep` (**g**lobally search for a **r**egular **e**xpression and **p**rint)
- In its most simple use case, it searches for a specific string in a text file
- For this, it takes two arguments — the file and the string to search for
- It then, reads each line of the file, checks if the line contains the string argument, and prints those lines
- This project involves:
    - learning how to make a command-line tool that uses the terminal features that many other command-line tools use
    - reading the value or an environment variable to allow the user to configure the behavior of the tool
    - printing error messages to the standard error console (`stderr`) instead of the standard output (`stdout`)
- The version of `grep` we build will naturally be very simple (for a more advanced version, see `[ripgrep](https://github.com/BurntSushi/ripgrep)`)
- This project covers all concepts in the preceding lessons and introduces some new ones like closures, iterators, and trait objects that are expanded on in later chapters.

## Accepting Command Line Arguments

- Let’s start by creating a new project called `minigrep`:
    
    ```rust
    cargo new minigrep
    ```
    
- The first step, then is to allow the resulting binary to accepts its inputs as command-line arguments — the string and the file path
- In short, we want to be able to do this:
    
    ```rust
    cargo run -- search_string /path/to/file.txt
    ```
    
- While there are libraries in `[crates.io](http://crates.io)` that make this easier for us, we will implement this from scratch with the standard library since we are only learning

### Reading the Arguments

- To read the arguments, we’ll need to use the `std::env::args` function provided by the standard library
- This function returns an iterator of the command-line arguments — an iterator produces a series of values, and we can call `collect` method on them to turn it into a collection such as a `vector` that contains all the elements the iterator produces
    
    ```rust
    // src/main.rs
    use std::env;
    
    fn main() {
        let args: Vec<String> = env::args().collect();
        dbg!(args);
    }
    ```
    
    Here, we brings `std::env` into scope instead of `std::env::args` directly because it allows us to use other functions provided by the `std::env` package **and** avoid potential conflicts with a variable like `args` (that is fairly common)
    
    Also note that the `args` function panics if any argument with an invalid unicode is supplied (an alternative is to use `std::env::args_os` that uses `OsString` instead of `String` but the values for `OsString` differ on a per platform basis and are more complex to work with then `String` values)
    
- When we run the above with `cargo run -- arg1 arg2`, we get:
    
    ```rust
    ╰─λ cargo run -- arg1 arg2
       Compiling minigrep v0.1.0 (~/rust-book-brown-notes/minigrep)
        Finished dev [unoptimized + debuginfo] target(s) in 3.09s
         Running `target/debug/minigrep arg1 arg2`
    [src/main.rs:5] args = [
        "target/debug/minigrep",
        "arg1",
        "arg2",
    
    ```
    
    From the output, we can see that the first argument is the program itself (`target/debug/minigrep`) and the rest are the args that we supplied on the command line
    

### Saving the Argument Values

- We can simply get the argument values by indexing into the `args` vector:
    
    ```rust
    use std::env;
    
    fn main() {
        let args: Vec<String> = env::args().collect();
    
        let searchstring = &args[1];
        let filepath = &args[2];
    
        dbg!(&args); // pass reference to avoid moving out of args
        println!("searchstring = {searchstring}, filepath = {filepath}");
    }
    ```
    
- If we run the above with `cargo run -- search_for_this this/is/the/path.txt`, we get:
    
    ```rust
    [src/main.rs:9] &args = [
        "target/debug/minigrep",
        "search_for_this",
        "/this/is/the/path.txt",
    ]
    searchstring = search_for_this, filepath = /this/is/the/path.txt
    ```
    
- For now, we’ll not look at edge cases (for example, when no or less arguments are supplied than what we expect)

## Reading a File

- For the purposes of this program, let’s create a file at the root of our project called `poem.txt` with the following text by Emily Dickinson:
    
    ```
    I'm nobody! Who are you?
    Are you nobody, too?
    Then there's a pair of us - don't tell!
    They'd banish us, you know.
    
    How dreary to be somebody!
    How public, like a frog
    To tell your name the livelong day
    To an admiring bog!
    ```
    
- Now, to read this text, we must use the `std::fs` module (that handles files)
- This module conveniently provides a `read_to_string` takes the `file_path` that opens the file, and returns a `std::io::Result<String>` of the file’s contents
- For now, we can add a temporary `println!` to inspect the contents of the text file
    
    ```rust
    use std::{env, fs};
    
    fn main() {
        let args: Vec<String> = env::args().collect();
    
        let searchstring = &args[1];
        let filepath = &args[2];
    
        let contents = fs::read_to_string(filepath).expect(&format!("{} does not exist", filepath));
    
        println!("searchstring = {searchstring}, filepath = {filepath}");
        println!("contents:\n{}", contents);
    }
    ```
    

## Refactoring

- At the present, our code has a few flaws
- The `main` function has multiple responsibilities: generally, functions are clearer and easier to maintain if each function is responsible for only one idea
- Another problem is that we are not handling errors gracefully, such as when not enough arguments are provided by the user, or the file could not be opened for some specific reason (such as permissions issues, file not being available, etc.)
- A third problem is that we have two “types” of variables — configuration variables like `searchstring` and `filepath`; and program (logic) variables like `contents`. If these are not separated, it becomes a hassle to maintain when either of these types of variables grows in number
- As the program grows, these tiny flaws become a big problem making it difficult to fix them cleanly
- It’s a good practice to refactor early on when developing a program because it is much easier to refactor smaller amounts of code

### Separation of Concerns for Binary Projects

- The organizational problem of allocated responsibilties for multiple tasks to the `main` function is common to many binary projects
- As a result, the Rust community has developed guidelines for splitting the separate concerns of a binary program when `main` starts getting large. This process has the following steps:
    1. Split your program into a `main.rs`  and a `lib.rs` and move your program’s logic to `lib.rs`
    2. As long as your command line parsing ogic is small, it can remain in `main.rs`
    3. When the command line parsing logic starts getting complicated, extract it from `main.rs` and move it to `lib.rs`.
- The responsibilties that remain in the `main` function after this process should be limited to the following:
    1. Calling the command line parsing logic with the argument values
    2. Setting up any other configuration
    3. Caling a `run` function in `lib.rs`
    4. Handling the error if `run` returns an error
- This process separates the logic and the actual binary that runs the program.
- As discussed earlier, it also allows us to test our program’s logic
- The code that remains in the `main.rs` program will be small enough to verify it by just reading it.

#### Extracting the Argument Parser

- Let’s extract the part of the program that parses the input arguments and returns the `searchstring` and `filepath` into a separate function
- Our `main` function will then call this function to get these values:
    
    ```rust
    use std::{env, fs};
    
    fn main() {
        let args: Vec<String> = env::args().collect();
        let (searchstring, filepath) = parse_args(&args);
        let contents = fs::read_to_string(filepath).expect(&format!("{} does not exist", filepath));
    
        println!("searchstring = {searchstring}, filepath = {filepath}");
        println!("contents:\n{}", contents);
    }
    
    fn parse_args(args: &[String]) -> (&str, &str) {
        let searchstring = &args[1];
        let filepath = &args[2];
    
        (searchstring, filepath)
    }
    ```
    
- Note that we are passing the args vector into the `parse_args` function that accepts a reference to a `String` slice and returns references to two `str`’s
- The `main` function need not know how the `searchstring` and `filepath` are being extracted from the `args` vector that it is passing to the `parse_args` function

#### Grouping Configuration Values

- At the moment, we are returning a tuple, but then we immediately break that tuple into individual parts again — a sign that perhaps we are not using the right abstraction
- Another indicator that shows there is room for improvement is that the `parse_config` function name implies that we are returning values that are both related to some configuration value. This relationship between the returned values is not being conveyed by our present code
- So, we will use a struct to encapsulate these return values and name them according to their meaning/purpose.
    
    ```rust
    use std::{env, fs};
    
    fn main() {
        let args: Vec<String> = env::args().collect();
        let config = parse_args(&args);
        println!(
            "searchstring = {}, filepath = {}",
            config.searchstring, config.filepath
        );
    
        let contents = fs::read_to_string(config.filepath).expect("provided filepath does not exist");
    
        println!("contents:\n{}", contents);
    }
    
    struct Config {
        searchstring: String,
        filepath: String,
    }
    
    fn parse_args(args: &[String]) -> Config {
        let searchstring = args[1].clone();
        let filepath = args[2].clone();
    
        Config {
            searchstring,
            filepath,
        }
    }
    ```
    
    Note that we move the `Config` out of the `parse_args` function and use it in `main`. For this, we need `Config` to own the `searchstring` and `filepath` which are now cloned from `args`. This is the simplest approach albeit inefficient (and frowned upon in large projects). If we are to use references, we will need to update the struct with:
    
    ```rust
    struct Config<'a> {
        searchstring: &'a String,
        filepath: &'a String,
    }
    
    fn parse_args(args: &[String]) -> Config {
        let searchstring = &args[1];
        let filepath = &args[2];
    
        Config {
            searchstring,
            filepath,
        }
    }
    ```
    
    In `main`, we cannot use the `config.filepath` in `expect` anymore because it is moved in the call to `read_to_string` (we can opt for borrowing in the call to `read_to_string`). For the same reason, we move the `println!` lines above the `let contents = ...` line.
    

#### Creating a Constructor for `Config`

- Now that we have a `parse_config` function that basically returns a `Config` object, it might be worthwhile to create a constructor that for the `Config` struct — which is a more idiomatic approach
    
    ```rust
    impl Config {
        fn new(args: &[String]) -> Self {
            let searchstring = String::from(args[1]);
            let filepath = String::from(args[2]);
    
            Self {
                searchstring,
                filepath,
            }
        }
    }
    ```
    
    We can call this in `main` with:
    
    ```rust
    let config = Config::new(&args);
    ```
    

#### Improving the Error Message

- We can add a check in the `new` method to make sure that the `args` slice is long enough for us to extract the required values
- For now, we can simply panic if there are fewer than 2 arguments supplied during the program call
- This is not the most elegant approach because we would like to panic only for programming errors and not usage errors
    
    ```rust
    impl Config {
        fn new(args: &[String]) -> Self {
            // first => program_name, second, third => arguments
            if args.len() < 3 {
                panic!("not enough arguments");
            }
    				...
    ```
    
- This provides a better error message than Rust’s default index out-of-bound panic that would have been triggered when indexing into a shorter `args` slice

#### Returning a `Result` instead of Calling `panic!`

- Instead of panicking, we can return a `Result` that will contain the `Config` instance in the `Ok` case and describe the problem in the `Err` case
- We might also want to update the function name from `new` to `build`
- With this approach, the `build` method can communicate with its caller (`main`) using the `Result`
    
    ```rust
    impl Config {
        fn build(args: &[String]) -> Result<Self, &str> {
            // first => program_name, second, third => arguments
            if args.len() < 3 {
                return Err("not enough arguments");
            }
    
            let searchstring = args[1].clone();
            let filepath = args[2].clone();
    
            Ok(Self {
                searchstring,
                filepath,
            })
        }
    }
    ```
    

#### Calling `Config::build` and Handling Errors

- We now need our `main` function to handle the `Result` from `Config::build`
- The way to do this is to call `unwrap_or_else` that “closes over” the returned `Err`:
    
    ```rust
    fn main() {
        let args: Vec<String> = env::args().collect();
        let config = Config::build(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        });
    		
    		...
    ```
    
- Here, the `unwrap_or_else` allows us to define a custom, non-`panic!` error handling
- If the result is an `Ok` value, it unwraps its contents
- However, if the result is an `Err` value, the method calls the code in the *closure* which is an anonymous function we define and pass as an argument to `unwrap_or_else`
- `closures` will be covered in detail later. For now, all that the above construct enables is the use of the wrapped `error` value (that appears inside the vertical pipes `| |`)
- In this case, the if there is an error, the program exits with an error code of `1` instead of panicking

```rust
cargo run -- search_for_this           
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep search_for_this`
Problem parsing arguments: not enough arguments
```

### Extracting Logic From `main`

- So far, we have factored out the configuration part of our original program.
- Now, for the logical part, we will create a separate `run` function. For the present, all this function does is take the `Config` and read the contents of the specified file path
    
    ```rust
    fn run(config: Config) {
        let contents = fs::read_to_string(config.filepath).expect("provided filepath does not exist");
    
        println!("contents:\n{}", contents);
    }
    ```
    

#### Returning Errors from the `run` Function

- Just like in the earlier case, we might want to supply a friendlier error message to the user instead of just panicking
- For this, we will return a `Result` type from the `run` function:
    
    ```rust
    use std::{env, error::Error, fs, process};
    ...
    
    fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.filepath)?;
    
        println!("contents:\n{}", contents);
        Ok(())
    }
    ```
    
- The above contains three significant changes:
    1. The return type has been changed to `Result<(), Box<dyn Error>>`.
        - The previous unit return type `()` has been preserved in the `Ok` case
        - For the error type, the *trait* object `Box<dyn Error>` has been used. For now, this simply means that it returns an error that implements the `Error` trait, but we don’t have to specify what that particular type will be. This gives us the flexibility to rturn error values that may be of different types in different error cases. `dyn` is short for `dynamic`
    2. The `expect` method call has been replaced with the error propagation syntax `?` that retuns the error value to the caller
    3. The `run` function returns `Ok(())` in the success case. The wrapped value is the unit type `()`. This syntax is the idiomatic way of indicating that we are using this function only for its side effects; it doesn’t return a value we need
- With these changes, we now need to change our `main` function, specifically how `run` is called (to handle the `Err` case)

#### Handling Errors Returned from `run` in `main`

- We can use the same `unwrap_or_else` logic from before or we can use the `if...let` construct.
- Here, `if..let` is preferable because we don’t have anything to `unwrap` in the same way as we did for `Config::build`

```rust
if let Err(err) = run(config) {
    println!("Application error: {err}");
    process::exit(1);
}
```

### Splitting Code into Library Crate

- Move all the code that isn’t in the `main` function to `src/lib.rs`
- The methods need to be marked with `pub` to make them accessible from the `main` function
- To use these functions, we still need to bring them into scope using the `crate`'s name since the `lib` file is a bit special:
    
    ```rust
    // src/lib.rs
    use std::{error::Error, fs};
    
    pub struct Config {
        searchstring: String,
        filepath: String,
    }
    
    impl Config {
        pub fn build(args: &[String]) -> Result<Self, &str> {
            ...
        }
    }
    
    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        ...
    }
    ```
    
    ```rust
    // src/main.rs
    use minigrep;
    use std::{env, process};
    
    fn main() {
        let args: Vec<String> = env::args().collect();
        let config = minigrep::Config::build(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        });
    
        // use if..let because there is nothing to unwrap
        if let Err(err) = minigrep::run(config) {
            println!("Application error: {err}");
            process::exit(1);
        };
    }
    ```
    

## Developing With Test-Driven Development (TDD)

- We’ll use the test-driven development approach to add the actual testing logic to our `minigrep` binary
- Here are the steps:
    1. Write a test that fails and run it to make sure it fails for the reason you expect
    2. Write or modify just enough code to make the new test pass
    3. Refactor the code you just added or changed and make sure the tests continue to pass
    4. Repeat!
- Writing the tests before you write code that makes the test pass helps to maintain high test coverage throughout the process

### Writing a Failing Test

- Let’s add a `tests` module in the `src/lib.rs` file:
    
    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn one_result() {
            let searchstring = "duct";
            let contents = "\
    Rust:
    safe, fast, productive.
    Pick three.";
    
            assert_eq!(vec!["safe, fast, productive."], search(searchstring, contents));
        }
    }
    ```
    
- The above tests the `search` function that we have yet to implement
- This function when run with the `query` and `contents` values shown above, should return the second line of `contents` (since it contains `duct`)
- However, the test does not compile yet because the `search` function does not exist yet
- According to the principles of TDD, we’ll add just enough code for the above to compile:
    
    ```rust
    fn search<'a>(searchstring: &'a str, contents: &'a str) -> Vec<&'a str> {
        vec![]
    }
    ```
    
- When we run the test, we expect it to fail, and indeed, that is what happens when we run `cargo test`:
    
    ```rust
    ---- tests::one_result stdout ----
    thread 'tests::one_result' panicked at 'assertion failed: `(left == right)`
      left: `["safe, fast, productive."]`,
     right: `[]`', src/lib.rs:48:9
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    
    failures:
        tests::one_result
    
    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    ```
    

### Writing Code to Pass the Test

- To pass the test, we need to implement the `search` function
- This function should iterate through each line of the `contents` and check if that line contains the search string. An example could be the following:
    
    ```rust
    fn search<'a>(searchstring: &'a str, contents: &'a str) -> Vec<&'a str> {
        let mut result: Vec<&str> = vec![];
    
        for line in contents.lines() {
            if line.contains(searchstring) {
                result.push(line);
            }
        }
    
        result
    }
    ```
    
- With the above, the test should now pass:
    
    ```rust
    running 1 test
    test tests::one_result ... ok
    
    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    
         Running unittests src/main.rs (target/debug/deps/minigrep-d474e455f4cd9c36)
    
    running 0 tests
    
    test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    
       Doc-tests minigrep
    
    running 0 tests
    
    test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    ```
    
- With this, we can now integrate the `search` function within our `run` function. The run function will iterate on the returned result from `search` printing out one matched line at a time
    
    ```rust
    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.filepath)?;
    
        let matching_lines = search(&config.searchstring, &contents);
    
        for matching_line in matching_lines {
            println!("{matching_line}");
        }
    
        Ok(())
    }
    ```
    
- Now, when we run the program, we get the desired results:
    
    ```rust
    ╰─λ cargo run -- body poem.txt           
        Finished dev [unoptimized + debuginfo] target(s) in 0.00s
         Running `target/debug/minigrep body poem.txt`
    I'm nobody! Who are you?
    Are you nobody, too?
    How dreary to be somebody!
    
    ╰─λ cargo run -- search_for_this poem.txt
        Finished dev [unoptimized + debuginfo] target(s) in 0.00s
         Running `target/debug/minigrep search_for_this poem.txt`
    ```
    

## Working with Environment Variables

- We will add another functionality to our `minigrep` that allows searching for a string in case-insensitive manner
- Whether this search takes place in a case-insensitive or a case-sensitive manner will depend on the value of an environment variable

### Case-Insensitive Test

- As before, we will first write a test for case-insenstive and case-sensitive searching:
    
    ```rust
        #[test]
        fn case_sensitive() {
            let searchstring = "duct";
            let contents = "\
    Rust:
    safe, fast, productive.
    Pick three
    Duct tape.";
    
            assert_eq!(
                vec!["safe, fast, productive."],
                search(searchstring, contents)
            );
        }
    
        #[test]
        fn case_insensitive() {
            let searchstring = "duct";
            let contents = "\
    Rust:
    safe, fast, productive.
    Pick three
    Duct tape.";
    
            assert_eq!(
                vec!["safe, fast, productive.", "Duct tape."],
                search(searchstring, contents)
            );
        }
    ```
    
- When we run the test, the `case_insensitive` search fails because we haven’t yet implemented it:
    
    ```rust
    running 3 tests
    test tests::case_sensitive ... ok
    test tests::one_result ... ok
    test tests::case_insensitive ... FAILED
    
    failures:
    
    ---- tests::case_insensitive stdout ----
    thread 'tests::case_insensitive' panicked at 'assertion failed: `(left == right)`
      left: `["safe, fast, productive.", "Duct tape."]`,
     right: `["safe, fast, productive."]`', src/lib.rs:91:9
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    ```
    

### Implementation

- Now, we can implement the `case_insensitive_search` function:
    
    ```rust
    fn case_insensitive_search<'a>(searchstring: &'a str, contents: &'a str) -> Vec<&'a str> {
        let mut result: Vec<&str> = vec![];
    
        let searchstring = searchstring.to_lowercase();
        for line in contents.lines() {
            if line.to_lowercase().contains(&searchstring) {
                result.push(line)
            }
        }
    
        result
    }
    ```
    
- Here, we convert each line and the search string to lowercase before checking whether the converted line contains the converted search string. This disregards the case of both the line and search string.
- Now, when we re-run the test by replacing the original `search` function with `case_insenstive_search`, all our tests pass again.

### Case Insensitive Config

- Now, we need to call the `case_insensitive_search` from the `run` function
- But this must be called only when we want the search to be case-insensitive.
- Let’s add an `ignore_case` configuration to our `Config` struct to control this behavior:
    
    ```rust
    pub struct Config {
        searchstring: String,
        filepath: String,
        ignore_case: bool,
    }
    ```
    
- We can then invoke the `case_insensitive_search` function if the `ignore_case` config is set to `true`:
    
    ```rust
    let matching_lines = match config.ignore_case {
        true => case_insensitive_search(&config.searchstring, &contents),
        false => search(&config.searchstring, &contents),
    };
    ```
    

### Building Config with Environment Variable

- Now, we need to build the config by taking into account the value of the environment variable
- Let’s assume that we need the `IGNORE_CASE` environment variable to be set in order for `ignore_case` to be true
- Then, in our build method, we can use the `std::env::var` function:
    
    ```rust
    use std::env;
    ...
    
    impl Config {
        pub fn build(args: &[String]) -> Result<Self, &str> {
            // first => program_name, second, third => arguments
            if args.len() < 3 {
                return Err("not enough arguments");
            }
            const IGNORE_CASE_ENV_KEY: &str = "IGNORE_CASE";
    
            let searchstring = args[1].clone();
            let filepath = args[2].clone();
            let ignore_case = env::var(IGNORE_CASE_ENV_KEY).is_ok();
    
            Ok(Self {
                searchstring,
                filepath,
                ignore_case,
            })
        }
    }
    ```
    
- With this, we can now run the binary:
    
    ```rust
    ╰─λ cargo run -- bOdy poem.txt
        Finished dev [unoptimized + debuginfo] target(s) in 0.00s
         Running `target/debug/minigrep bOdy poem.txt`
    
    ─λ IGNORE_CASE="" cargo run -- bOdy poem.txt
        Finished dev [unoptimized + debuginfo] target(s) in 0.00s
         Running `target/debug/minigrep bOdy poem.txt`
    I'm nobody! Who are you?
    Are you nobody, too?
    How dreary to be somebody!
    
    ╰─λ IGNORE_CASE= cargo run -- bOdy poem.txt  
        Finished dev [unoptimized + debuginfo] target(s) in 0.00s
         Running `target/debug/minigrep bOdy poem.txt`
    I'm nobody! Who are you?
    Are you nobody, too?
    How dreary to be somebody!
    ```
    
    Note that based on our implementation, we only need the environment variable to be set instead of also requiring it to have a proper value. The `env::var` function returns a Result type: `Result<String, VarError>`. With `IGNORE_CASE=`, the `var` function returns `Ok("")`.
    

## Writing Error Messages to StdErr instead of StdOut

- The distinction between `stderr` and `stdout` messages allows users of our binary to redirect output and errors to appropriate locations
- The `println!` macro is only capable of printing to the standard output, so we need to use something else to print the output to `stderr`
- This something is the `eprintln!` macro!
- Now, we can replace `println!` with `eprintln!` when displaying error messages:
    
    ```rust
    use minigrep;
    use std::{env, process};
    
    fn main() {
        let args: Vec<String> = env::args().collect();
        let config = minigrep::Config::build(&args).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        });
    
        // use if..let because there is nothing to unwrap
        if let Err(err) = minigrep::run(config) {
            eprintln!("Application error: {err}");
            process::exit(1);
        };
    }
    ```
    
- With this, we can get the error message (in Linux and linux-like systems with):
    
    ```bash
    cargo run 2>error.txt ## 2 represents stderr in linux
    ```
    
    When we run the above, the error message gets written into `error.txt`. The actual contents would look something like:
    
    ```rust
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
         Running `target/debug/minigrep`
    Problem parsing arguments: not enough arguments
    ```
