# Error Handling

## Introduction

- Rust requires you to acknowledge the possibility of an error and take some action before your code can compile.
- Rust groups errros into two major categories: *recoverable* and *unrecoverable*
- For some errors (like wrong input), you might just want to re-prompt the user and retry the operation
- For some errorrs, you might just want to stop the program as it might be the symptom of a bug (such as out-of-bounds indexing)
- Most languages do not distinguish between these two, and insted use *exceptions* to handle both
- Instead of *exceptions*, Rust has the `Resut<T, E>` type for recoverable errros and `panic!` that stop program execution when it encounters a non-recoverable ones.

## Unrecoverable Errors with `panic!`

- Rust provides the `panic!` macro for cases when bad things happen in your code, and you cannot do anything about it
- There are two ways to cause panic in practice:
    - Taking an action that causes our code to panic (such as accessing an array past the end) or,
    - Explicitly calling the `panic!` macro
- By default, these panics will print a failure message, unwind, clean up the stack, and quit.
- Via an environment variable `RUST_BACKTRACE` (with value `1`), we can also have Rust display the call stack when a panic occurs to make it easier to track down the cause of the panic

### Unwinding vs Aborting

- When a panic occurs, the program starts to walk back the stack and cleans up the data from each function it encounters
- However, this is a lot of work and so, Rust allows you to choose the alternative of immediately *aborting*, which ends the program without cleaning up
- The memory that was being used by the program should then be cleaned up by the operating system itself
- Resorting to `aborting` instead of `unwinding` will reduce the size of the resulting binary
- We can perform this switch by adding `panic = 'abort'` to the `profile` section of your `Cargo.toml` file:
    
    ```toml
    [profile.release]
    panic = 'abort'
    ```
    

### Backtrace

- Take the following program:
    
    ```rust
    fn main() {
        let a = vec![1; 100];
        a[100];
    }
    ```
    
- Here, we are trying to access the 101th element in an array that only contains 100 elements.
- This results in a runtime panic:
    
    ```rust
    cargo run
       Compiling demo v0.1.0 (~/rust-book-brown-notes/demo)
        Finished dev [unoptimized + debuginfo] target(s) in 0.69s
         Running `target/debug/demo`
    thread 'main' panicked at 'index out of bounds: the len is 100 but the index is 100', src/main.rs:3:5
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrac
    ```
    
- The error tells us exactly what we expect!
- In C, when we do a similar thing, an undefined behavior occurs — you would get whatever is at the location in memory that would correspond to that element in the data structure, even though the memory doesn’t belong to that structure
- This is called `buffer overread` and can lead to security vulnerabilities
- Rust protects against these vulnerabilities by stopping the execution of the program when such an error occurs
- When we use the environment variable `RUST_BACKTRACE` as suggested in the above error messag, we get a more verbose output:
    
    ```rust
    RUST_BACKTRACE=1 cargo run
        Finished dev [unoptimized + debuginfo] target(s) in 0.07s
         Running `target/debug/demo`
    thread 'main' panicked at 'index out of bounds: the len is 100 but the index is 100', src/main.rs:3:5
    stack backtrace:
       0: rust_begin_unwind
                 at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:578:5
       1: core::panicking::panic_fmt
                 at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/panicking.rs:67:14
       2: core::panicking::panic_bounds_check
                 at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/panicking.rs:162:5
       3: <usize as core::slice::index::SliceIndex<[T]>>::index
                 at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/slice/index.rs:261:10
       4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
                 at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/slice/index.rs:19:9
       5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
                 at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/alloc/src/vec/mod.rs:2691:9
       6: demo::main
                 at ./src/main.rs:3:5
       7: core::ops::function::FnOnce::call_once
                 at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/ops/function.rs:250:5
    note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    ```
    
- A `backtrace` is a list of all the functions that have been called to get to this point.
- The key to reading the backtrace is to start from the top and read until you see files you wrote — that’s where the problem originated
- To see this backtrace, the debug symbols must be enabled which are indeed enabled by default when building with `cargo build` or `cargo run` without the `--release` flag.

### Quiz

1. What is the name of the environment variable you should set to `1` to see the backtrace of a panic?
    - Ans
        
        `RUST_BACKTRACE`
        
2. Which of the following is NOT a good reason to use a panic?
    - Ans
        
        The program has reached an error state which should be communicated to a caller function
        

## Recoverable Errors with `Result`

- Most errors aren’t serious enough to require the program to stop entirely
- For example, if an error occurs because a file that your program expects does not exist, you might want to create the file instead of just crashing
- For this purpose, Rust provides the `Result<T, E>` enum with two variants `Ok(T)` and `Err(E)` where
    - `T` represents the type of the value that will be returned in a failure case within the `Ok` variant, and
    - `E` represents the type of the error that will be returned in a failure case within the `Err` variant
- Example:
    
    ```rust
    let file_result = File::open("this_file_does_not_exist.txt");
    
    match file_result {
        Ok(_) => println!("File opened successfully!"),
        Err(e) => println!("An error occurred: {}", e.to_string()),
    }
    ```
    
    The above outputs the following:
    
    ```rust
    An error occurred: No such file or directory (os error 2)
    ```
    
    We can of course choose to `panic!` here instead of printing out the error message
    

### Matching on Different Errors

- The above code will print out an error message no matter what the type of the error is
- Sometimes, we may need to handle erros differently based on the type of the error
- For example, we might want to create a file if the file does not exist but if the failure occurs because we do not have sufficient permissions to open the file, we might want to `panic!` as there is nothing much we can do.
- To demonstrate this, let’s create a file called `unreadable.txt` that has no permissions (we can do this in Linux or MacOS with `chmod 000 unreadable.txt`). Then, we can try running the following code:
    
    ```rust
    use std::{fs::File, io::ErrorKind};
    
    fn try_to_open_file(path: &str) -> File {
        let file_result = File::open(path);
    
        let my_file = match file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create(path) {
                    Ok(f) => f,
                    Err(e) => panic!("could not create file: {}, {:?}", path, e),
                },
                other_error => panic!(
                    "an error occurred while opening file: {}, {:?}",
                    path, other_error
                ),
            },
        };
    
        return my_file;
    }
    
    fn main() {
        let creatable_file_path = "non_existing_file.txt";
        let unreadable_file_path = "unreadable.txt";
    
        try_to_open_file(creatable_file_path);
        try_to_open_file(unreadable_file_path);
    }
    ```
    
- Note the `try_to_open` function:
    - This function tries to open the file at the specified path.
    - If the operation is successful and `file_result` has the `Ok(file)` variant, the file simply returned.
    - If the `Err` variant is returned, we now match on the kind of error returned by the `error.kind()` method call.
    - Then, if the error occurs due to the file not being present, w etry to create the file. This operation might also error, so we match on the Result type returned by the `create` method. If the result is an `Ok` variant, we return the created file and if it errors, we panic.
    - If error kind is not that the file does not exist, then, there might be some other error that is preventing us from reading the file.
- When we run the above program, it should create the file `non_existing_file.txt` and for `unreadable.txt`, it will panic with:
    
    ```rust
    thread 'main' panicked at 'an error occurred while opening file: unreadable.txt, PermissionDenied', src/recoverable.rs:13:28
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    ```
    

### Quiz

1. Which of these statements best describes why `File::open` returns a `Result` and not an `Option`?
    - Ans
        
        Because `Result` can represent why an operation failed, and file opening can fail for many reasons
        
        **Context**: `Option` can just represent *that* an operation has failed, but `Result` can explain *why* the operation has failed.
        

### Shortcuts for `panic` on `Error`

- Using `match` works well but the resulting syntax isunnecessarily verbose and doesn’t always communicate the intentions well
- As alternatives, we have two methods: `unwrap` and `expect`

#### `unwrap`

- The `unwrap` method unwraps the `Result` enum and panics if an error occurs or returns the wrapped `Ok` value if it exists.
- For example:
    
    ```rust
    fn try_to_open_file_with_unwrap(path: &str) -> File {
        File::open(path).unwrap()
    }
    ```
    
    The above function tries to open a file and return it but if an error occurs it panics. This is much more succinct than using a `match` expression:
    
    ```rust
    match File::open(path) {
    	Ok(f) => f,
    	Err(e) => panic!("an error occurred: {:?}", e),
    }
    ```
    
    The above code fails with the following error message if the `path` does not exist:
    
    ```rust
    thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/recoverable.rs:24:22
    ```
    
    The panic message as you can see is neatly formatted as well!
    

#### `expect`

- The `expect` method on the other hand allows us to define an error message as well for the `panic!` macor that is called underneath.
- For example:
    
    ```rust
    fn try_to_open_file_with_expect(path: &str) -> File {
        File::open(path).expect(&format!("{path} must exist in this project"))
    }
    ```
    
    The above will panic if the file does not exist:
    
    ```rust
    thread 'main' panicked at 'non_existing_file.txt must exist in this project: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/recoverable.rs:28:22
    ```
    
- In production-ready code, most Rustaceans choose to use the `expect` method rather than the `unwrap` method as the former helps us provide more context to the error message.

### Propagating Errors

- When an error occurs in a piece of code, you might want to defer the error handling to the calling code instead of handling the error in the code itself.
- This is called `error propagation` and gives more control the calling code where there be more information or logic that dictates how the error should be handled that what. you have available in the context of your code.
- This can be achieved by simply returning the `Err` variant:
    
    ```rust
    use std::fs::File;
    use std::io::{self, Read};
    
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");
    
        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
    
        let mut username = String::new();
    
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }
    ```
    
    Notice here that the function returns a `Result` that the caller has to now handle.
    
    As for the error, at each point in the `match` expression, we are returning the error variant `Err(e)`!
    
- The above pattern is so common that Rust provides a syntactic sugar to help out using the `?` operator

### The `?` Operator

- With the `?` operator, the above code simplifies to:
    
    ```rust
    fn read_username_from_file(path: &str) -> Result<String, io::Error> {
        let mut username_file = File::open(path)?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
    ```
    
- The `?` handles returning of the `Ok` value or propagating the `Err` variant.
- We can even shorten this code further with:
    
    ```flow
    fn read_username_from_file_succinct(path: &str) -> Result<String, Error> {
        let mut username = String::new();
    
        File::open(path)?.read_to_string(&mut username)?;
    
        Ok(username)
    }
    ```
    
- This task is so common that the standard library provides a method that encapsulates all of the above with:
    
    ```rust
    use std::fs;
    use std::io;
    
    fn read_username_from_file_with_fs(path: &str) -> Result<String, io::Error> {
    	fs::read_to_string(path)
    }
    ```
    

#### Use case for `?`

- The `?` operator performs an early return if an error occurs.
- This means the returned error type must match the error kind defined in the `Result<T, E>` returned by the function that is trying to propagate the error
- So, for our code to compile, the error being propagated must of the same type as the error we are trying to return with `?`.
- With the match expression, we have an opportunity to wrap the error in some way so that it complies with the function signatures. Since we don’t have this option with the `?` operator, we need to be mindful.

### Quiz

1. Given an arbitrary expression `e` of type `Result<T, E>`, which code snippet best represents how `e?` is translated?
    - Ans
        
        ```rust
        match e {
          Ok(x) => x,
          Err(err) => { return Err(err); }
        }
        ```
        
        **Context**: If `e` is a `Result`, then `e?` extracts the value inside the `Ok` if possible, otherwise returning the `Err` out of the current function.
        
2. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    // assume hello.txt has the contents "will"
    fn read_username_from_file() -> Option<String> {
      let mut username_file = File::open("hello.txt")?;
      let mut username = String::new();
      username_file.read_to_string(&mut username)?;
      Some(username)
    }
    fn main() {
      println!("{}", read_username_from_file().unwrap()); 
    }
    ```
    
    - Ans
        
        This program **does not** compile.
        
        **Context**: `File::open` returns a `Result`, but the return type of `read_username_from_file` expects an `Option`. Therefore it is invalid to use the `?` operator until the `Result` has been converted to an `Option` (e.g. with the `Result::ok` method).
        

## To `panic!` or Not to `panic!`

- When we create a function that panics when an error occurs, we are making a decision on behalf of the caller that the error is unrecoverable and should crash the program.
- When we create a function that instead returns a `Result` type, we are allowing the caller control over how to handle the error — the caller may decide to panic after all and turn our recoverable error to an unrecoverable error or may choose to handle the error in a way that is appropriate to their use case.
- For examples, prototypes, tests, it is more appropriate to just `panic`.
- While in other cases, returning the `Result` type may be more appropriate

### Examples, Prototypes and Tests

- In all these cases, we care more about the `happy` path than how to handle specific errors
- There is still room for this later on when shipping the code (by checking the markers namely, `expect` and `unwrap` calls) but that may not be the priority when dealing with examples or prototypes
- For tests, we want the panic to occur when a test case fails because it provides as clear an indicator of failure as normal error handling and failing would.

### Cases When You Have More Information than the Compiler

- There are cases where you are certain that `panic!` does not occur at runtime but the compiler is not aware of this
- For example, take the following code:
    
    ```rust
    use std::net::IpAddr;
    
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    ```
    
- We know in this case that the `home` variable will get a vaild `IpAddr` `Ok` value but the compiler cannot guarantee this
- So, it is fine to panic if an error occurs here both because it should never really panic and because if it does panic someone must have messed up the code

### Guidelines for Error Handling

#### `panic`

- it is advisable to have your code panic when it’s possible that your code could end up in a bad state
- A *bad state* is when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values, or missing values are passed to your code — plus one or more of the following:
    - The bad state is something that is unexpected, as opposed to something that will likely happen occasionallly, like a user entering data in the wrong format
    - Your code after this point needs to rely on not being in this bad state, rather than checking for the problem at every step
    - There isn’t a good way to encode this information in the types you use.
- If someone calls your code and passed in values that don’t make sense, it’s best to return an error if you can, so that the user of the library can decide what they want to do in that case
- In cases wehre continuing could be insecure or harmful, the best choice might be to call `panic!` and alert the person to the bug in theri code so they can fix it during development
- `panic!` may also be appropriate when you’re calling external code that is out of your control and it returns an invalid staet that you have no way of fixing.
- When your code performs an operation that could put a user at risk if it’s called using invalid values, you code should verify the values are valid and panic if the values are not valid. This is mainly for safety reasons. For example, the standard library panics if you are accessing an array element at an out-of-bounds index.
- Function often have contracts — their behavior is only guaranteed if the inputs meets particular requirements. Panicking when this contract is violated makes sense because a contract violation always indicates a caller-side bug and it’s not a kind of error you want the calling code to have to explicitly handle. This contract should be explained in the API documentation for the function
- These *contract-violation* checks are most effectively guaranteed by leveraging the type system. For example, using the `u32` type as the parametere so that the compiler can reject negative values being passed to the function.

#### Not to `panic`

- However, when a failure is expected, it’s more appropriate to return a `Result` than to make a `panic!` call. For example, a parser being given malformed data or an HTTP request returning a status that indicates you have hit a rate limit.

### Creating Custom Types for Validation

- We can also create our own types that confirm to the contract set out by our function.
- For example, take the guessing game from chapter two where we are expect the input to be between 1 and 100
- While we can handle the error by re-prompting the user to enter another number if the input number does not lie within these bounds like so:
    
    ```rust
    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
        continue;
    }
    ```
    
    there is a better way.
    
- We can create a new type that only accepts numbers between 1 and 100 because ensuring that this condition is not violated at every step of the way can get tedious (although not applicable in this case)
- So, we can create a new struct type that encapsulates the guess and define methods on it that have these guarantees in place:
    
    ```rust
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
    ```
    
- We can then update our API documentation to tell users what to expect

### Quiz

1. A Rust programmer is designing a library for writing command-line interfaces. As a part of this library, they are implementing a function to parse command-line flags provided by a user. Which implementation would be most appropriate for this domain?
    
    ```rust
    fn parse_flag_v1(flag: &str) -> Result<String, String> {
      match flag.strip_prefix("--") {
        Some(no_dash) => Ok(no_dash.to_string()),
        None => Err(format!("Invalid flag {flag}"))
      }
    }
    fn parse_flag_v2(flag: &str) -> String {
      match flag.strip_prefix("--") {
        Some(no_dash) => no_dash.to_string(),
        None => panic!("Invalid flag {flag}")
      }
    }
    ```
    
    - Ans
        
        `parse_flag_v1`
        
        **Context**: Here, the programmer would likely want to use a *recoverable* error (the `Result`). If a CLI user passes an incorrectly formatted flag, then the CLI library might want to provide additional help like displaying the possible set of flags. A panic would force the application to only show the panic message, and would probably be a worse user experience.
