# Automated Testing
## Introduction

- Correctness in a program is the extent to which the code does what we intend it to do
- Correctness itself is complex and not easy to prove
- Rust’s type system does part of the job but cannot catch everything
- As such, Rust supports writing automated software tests
- For example, if we are to write a function that adds two numbers by accepting these two numbers, Rust can check if the inputs are indeed numbers and returns a number but it cannot check whether the logic we write within the program is correct
- This chapter discusses the mechanics of Rust’s testing facilities:
    - Annotations and macros available when writing tests
    - The default behavior and options provided for running your tests
    - How to organize tests into unit tests and integration tests

## How to Write Tests

- Tests are functions that verify that non-test code is functioning in the expected manner
- The body of a test function performs the following actions:
    - Set up any needed data or state
    - Run the code you want to test
    - Assert the results are what you expect

### Anatomy of a Test Function

- A function that’s annotated with the `test` attribute
- Attributes are metadata about pieces of Rust code (for example, the `derive` attribute we used with structs)
- To change a function into a test function we use the test attribute: `#[test]` on the line before the `fn` declaration
- When we run the tests with the `cargo test` command, Rust builds a test runner binary that runs the annotated functions and reports on whether each test function passes or fails.
- When we create a Rust library with `cargo new --lib`, it creates a `src/lib.rs` file that contains the basic scaffolding for a test so that we do not need to look up the exact structure and syntax every time you start a new project.
- We can create as many test functions and modules as we want.
- Let’s see what a test function looks like by creating a new library: `cargo new --lib demo`
- This creates a new file called `src/lib.rs` with the following contents:
    
    ```rust
    pub fn add(left: usize, right: usize) -> usize {
        left + right
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn it_works() {
            let result = add(2, 2);
            assert_eq!(result, 4);
        }
    }
    ```
    
- Note the following:
    - `#[test]`
        - this attribute (annotation) indicates that this is a test function so that the test runner knows to treat this function as a test
        - we might also have other functions not annotated with `#[test]` that perform some kind of setup for “actual” tests
    - `assert_eq!`
        - In the function body, this macro asserts that the `result` (which contains the output of calling `add` on the inputs `2` and `2`) equals 4
- When we run the test with `cargo test`, we get:
    
    ```rust
    Running unittests src/lib.rs (target/debug/deps/demo-1d741307a5a5a022)
    
    running 1 test
    test tests::it_works ... ok
    
    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    
       Doc-tests demo
    
    running 0 tests
    
    test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    ```
    
- The output shows that:
    - Rust ran the test,
    - the name of the generated test function was `it_works` and the result of the test was `ok`,
    - the overall summary was `test result: ok` which means that all tests passed,
    - the result details include `1 passed; 0 failed; 0 ignored, 0 measured; 0 filtered out`indicates the relevant statistics for the test where
        - `passed` represent the test functions that passed
        - `failed` represents the test functions that failed
        - `ignored` represens the test functions that were not run (as they were ignored)
        - `measured` represents the benchmarking tests that were run (not stable yet)
        - `filtered out` represents the test functions that were not run because they did not clear certain filters when running the test,
    - a section for `Doc-tests` is present for tests that were present in the documentation (currently absent) — Rust can compile any code examples that appear in our API documentation

### Customizing the Test

- Let’s rename the function `it_works` to `exploration` and update the function body:
    
    ```rust
    fn exploration() {
        assert_eq!(2 + 2, 4)
    }
    ```
    
- When we run `cargo test` here, we get:
    
    ```rust
    running 1 test
    test tests::**exploration** ... ok
    
    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    
       Doc-tests demo
    
    running 0 tests
    
    test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    ```
    
- The output now shows `exploration` instead of `it_works`. As before, the test passes.
- Now, let’s write a failing test:
    
    ```rust
    fn another() {
        panic!("this test fails!");
    }
    
    ```
    
- When we run the test, we get:
    
    ```rust
    running 2 tests
    test tests::exploration ... ok
    test tests::another ... FAILED
    
    failures:
    
    ---- tests::another stdout ----
    thread 'tests::another' panicked at 'this test fails!', src/lib.rs:15:9
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    
    failures:
        tests::another
    
    test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    
    error: test failed, to rerun pass `--lib`
    ```
    
- Now, insted of `ok`, the output says `FAILED`
- The output also shows two new sections, including:
    - the detailed reason for each test failure (in this case, a panic with the string `this test fails!`
    - the names of  individual tests that failed (in this case, `another`)

### Useful Macros

#### `assert!`

- There are some helpful macros that help us in writing tests, one of them being the `assert_eq!` macro that we have seen above
- There is also the `assert!` macros that accepts an argument that must evaluate to a `boolean`
- The resulting test passes if the argument evaluates to `true` and fails otherwise
- To demonstrate, let’s test the `can_hold` function from `Chapter 5`:
    
    ```rust
    // src/lib.rs
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    ```
    
- We can test the `can_hold` method with:
    
    ```rust
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
    
        let smaller = Rectangle {
            width: 7,
            height: 6,
        };
    
        assert!(larger.can_hold(&smaller));
    }
    ```
    
- We can also test the case that the `smaller` one cannot hold the `larger` one:
    
    ```rust
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
    
        let smaller = Rectangle {
            width: 7,
            height: 6,
        };
    
        assert!(!smaller.can_hold(&larger));
    }
    ```
    
- If we break the implementation of the `can_hold` method (perhaps by replacing the `>` sign with `<`), we’ll get a failing test.

#### `assert_eq!` and `assert_ne!`

- This macro as we have seen in previous examples, tests the equality of its `left` and `right` parameters.
- Its counter part `assert_ne!` tests that the inequality of its `left` and `right` parameters.
- When the assertions fail, these macros print their arguments using `debug` (`:?`) formatting
- The arguments should also be comparable to test for equality
- As such, the arguments that we pass to these macros should implement the `PartialEq` and `Debug` traits
- As both these traits are derivable, implementing these traits is as simple as adding the `#[derive(PartialEq, Debug)]` to the top of our struct definitions

### Adding Custom Failure Messages

- We can add additional arguments to the above macros
- Any such optional arguments will be passed to the `format!` macro underneath (using the `+` operator)
- So, we can pass any number of extra arguments that confirm to the format expected by the `format!` macro
- For example:
    
    ```rust
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // should return "Hello, Carol"
        // greeting (`Hello`) may change, so only test input
    		assert!(
    		    result.contains("Carol"),
    		    "Greeting did not contain the name, value was {}",
    		    result
    		);
    }
    ```
    
- If the test fails, we get the following:
    
    ```rust
    test tests::greeting_contains_name ... FAILED
    
    failures:
    
    ---- tests::greeting_contains_name stdout ----
    thread 'tests::greeting_contains_name' panicked at 'Greeting did not contain the name, value was Hello Hello!', src/lib.rs:56:9
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    
    failures:
        tests::greeting_contains_name
    ```
    

### Checking for Panics with `should_panic`

- To test a function that should panic under a given condition, we add the `should_panic` attribute to the test function
- For example:
    
    ```rust
    pub fn panic_if_greater(input: i32, limit: i32) -> String {
        if input > limit {
            panic!("Input cannot be greater than {limit}, received {input}");
        }
    
        format!("Congratulations, you stayed under the limit, received {input}!")
    }
    
    #[cfg(test)]
    mod tests {
    	use super::*;
    
    	#[test]
      #[should_panic]
      fn greater_than_limit() {
          let value = 60;
          let limit = 40
    
          panic_if_greater(value, limit);
      }
    }
    ```
    
    The above test passes because the underlying function `panic_if_greater` panics!
    
    The test would fail if we swap the positions of `value` and `limit` in the function call:
    
    ```rust
    test tests::smaller_cannot_hold_larger ... ok
    
    failures:
    
    ---- tests::greater_than_limit stdout ----
    note: test did not panic as expected
    
    failures:
        tests::greater_than_limit
    ```
    
- Tests that use `should_panic` are imprecise because the function being tested might panic for a different reason that what we expect.
- To make these kinds of tests more precise, we can add an optional `expected` parameter to the `should_panic` attribute that checks for the expected panic message and fails if the test panics with a message that does not contain the message that is expected
- For example:
    
    ```rust
    #[test]
    #[should_panic(expected = "Input cannot be greater than")]
    fn greater_than_limit() {
        let value = 60;
        let limit = 40;
    
        panic_if_greater(value, limit);
    }
    ```
    
    Here, we do not need to pass the exact panic message (which is dynamic). The above test still passes because our panic message contains the the substring in the `expected` parameter
    

### Using `Result<T, E>` in Tests

- So far, all our test functions panic when they fail.
- We can also write test functions whose return type is a `Result` enum that resolves to an `Error` variant when the test fails
- For example:
    
    ```rust
    #[test]
    fn adder_works() -> Result<(), String> {
        if adder(2, 3) == 5 {
            Ok(())
        } else {
    				Err(String::from(format!(
    				    "two plus three does not equal five, got {}!",
    				    adder(2, 3)
    				)))
        }
    }
    ```
    
- If we were to break the implementation of the `adder` function, we get the following test result:
    
    ```rust
    test tests::adder_works ... FAILED
    
    failures:
    
    ---- tests::adder_works stdout ----
    Error: "two plus three does not equal five, got 7!"
    
    failures:
        tests::adder_works
    ```
    
- Using the `Result` enum as the return value of tests allows us to use the `?` operator in the body of tests so as to propagate errors if we wish to fail if any operation within the test returns an `Err` variant.
- The `#[should_panic]` attribute cannot be used on tests that return a `Result` enum
- To assert that an operation returns an `Err` variant, **don’t** use the question mark operator but instead that the `Err` varaitn is returned:
    
    ```rust
    assert!(value.is_err())
    ```
    

### Quiz

1. What is the annotation you add to a function to indicate that it's a unit test?
    - Ans
        
        `#[test]`
        
        **Context**: This informs the cargo testing harness to treat the function as a test and not library code.
        
2. Let's say you have a function with the type signature:
    
    ```rust
    fn f(x: usize) -> Result<usize, String>;
    ```
    
    And you want to test that `f(0)` should return `Err(_)`. Which of the following is NOT a valid way to test that?
    
    - Ans
        
        ```rust
        #[test]
        #[should_err]
        fn test() -> Result<usize, String> {
          f(0)
        }
        ```
        
        **Context**: `should_err` does not exist in Rust — any test that returns a `Result` must return `Ok` to pass.
        

## Controlling How Tests Are Run

- Just like `cargo run` compiles the code to get a resulting binary, `cargo test` compiles the code in test mode and runs the resulting test binary
- The default behavior is to
    - run all the tests in parallell,
    - capture output generated during the test runs,
    - prevent the output from being displayed, and
    - make it easier to read the output releated to the test results
- We can specify commandline arguments to change this default behavior
- There are tests that go to `cargo test` and those that go to the resulting test binary. To separate the two, we use `--`.
- For example, `cargo test --help` displays help text for the cargo test command and `cargo test -- --help` displays the help for the resulting test binary

### Running Tests in Parallel or Consecutively

- Because tests are running in parallel using threads, you must make sure your tests don’t depend on each other or on any shared state (including a shared environment, such as the current working directly or environment variables)
- You need to make sure that one test does not interfere with another test and create false positives.
- If you want to have fine-grained control over the number of threads (setting it to 1 if you do not want parallelism), you can use `test-threads` flag on the test binary:
    
    ```rust
    cargo test -- --test-threads=1
    ```
    

### Showing Function Output

- If any given test passes, `cargo test` captures and suppresses anything in the standard output
- For example, if a test containing `println!` call passes, we will not see the result being displayed
- If a test fails, we will see the anything printed to stdout along with the failure message
- If we want to see the printed values, we can use the `show-output` flag:
    
    ```rust
    cargo test -- --show-output
    ```
    
- The above will also cause the `successes` to be shown as well as the panic message (whether or not it is expected):
    
    ```rust
    running 5 tests
    test tests::adder_works ... FAILED
    test tests::larger_can_hold_smaller ... ok
    test tests::smaller_cannot_hold_larger ... ok
    test tests::greeting_contains_name ... ok
    test tests::greater_than_limit - should panic ... ok
    
    successes:
    
    ---- tests::greater_than_limit stdout ----
    thread 'tests::greater_than_limit' panicked at 'Input cannot be greater than 40, received 60', src/lib.rs:19:9
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    
    successes:
        tests::greater_than_limit
        tests::greeting_contains_name
        tests::larger_can_hold_smaller
        tests::smaller_cannot_hold_larger
    
    failures:
    
    ---- tests::adder_works stdout ----
    Error: "two plus three does not equal five, got 7!"
    
    failures:
        tests::adder_works
    
    test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    ```
    

### Running a Subset of Tests by Name

- Sometimes running a full test suite can be time-consuming when we want to actually test for a particular case
- We can run a subset of tests by passing the pattern of the name(s) of the test that we want to run as arguments to `cargo test`
    
    ```rust
    cargo test <test name pattern>
    ```
    
- For example:
    
    ```rust
    ╰─λ cargo test smaller 
        Finished test [unoptimized + debuginfo] target(s) in 0.00s
         Running unittests src/lib.rs (target/debug/deps/demo-1d741307a5a5a022)
    
    running 2 tests
    test tests::larger_can_hold_smaller ... ok
    test tests::smaller_cannot_hold_larger ... ok
    
    test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s
    ```
    
- Test that do not match the specified pattern are counted towards the `fitered out` number
- We can also filter by a module’s name instead of the name of the test. If the specified pattern matches the name of the module, all the tests in the module are run

### Ignoring Some Tests Unless Specifically Requested

- Sometimes, some specific tests can be very time-consuming to execute and we might want to ignore them unless we explicitly specify them
- So, instead of having to specify a large number of tests that we do want to run, Rust allows us to ignore some tests
- To do this, we add the `#[ignore]` attribute below the `#[test]` attribute:
    
    ```rust
    #[test]
    #[ignore]
    fn adder_works() -> Result<(), String> {
        if adder(2, 3) == 5 {
            Ok(())
        } else {
            Err(String::from(format!(
                "two plus three does not equal five, got {}!",
                adder(2, 3)
            )))
        }
    }
    ```
    
- When we run `cargo test` now, we get:
    
    ```rust
    running 5 tests
    test tests::adder_works ... ignored
    test tests::smaller_cannot_hold_larger ... ok
    test tests::larger_can_hold_smaller ... ok
    test tests::greeting_contains_name ... ok
    test tests::greater_than_limit - should panic ... ok
    
    test result: ok. 4 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s
    
       Doc-tests demo
    
    running 0 tests
    
    test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    ```
    
    The `adder_works` test now counts towards the `ignored` tests
    
- To run the ignored test, we pass the `ignored` flag to the test binary:
    
    ```rust
    cargo test -- --ignored
    ```
    
    This will run only the ignored tests
    
- To run all the tests including those that are ignored, you can use the `include-ignored` flag:
    
    ```rust
    cargo test -- --include-ignored
    ```
    

### Quiz

1. When running `cargo test` with no additional configuration, which of the following actions may not work correctly if done by multiple tests?
    - Ans
        
        Writing text to a single file
        
        **Context**: Tests are run in parallel by default, so actions which are not thread-safe (like writing to a single file) may cause a race condition.
        
2. Consider a program with the following unit test:
    
    ```rust
    #[test]
    fn test_the_logger() { /* ... */ }
    #[test]
    fn test_the_database() { /* ... */ }
    #[test]
    fn test_logger_and_database() { /* ... */ }
    ```
    
    What is the *shortest* string you can pass to `cargo test <the_string>` such that *only* `test_the_logger` and `test_the_database` are executed?
    
    - Ans
        
        `h`
        
        **Context**: The shortest substring string that is not contained in `test_logger_and_database` but is contained in `test_the_logger` and `test_the_database` is "h" (the middle character of "the").
        

## Test Organization

- Testing is a complex discipline and as such, there can be many ways to organize tests
- Rustaceans typically think of tests in terms of *unit tests* and *integration tests*

### Unit Tests

- Small and more focussed
- Tests a module in isolation at a time
- Can test a modules private interfaces
- These tests are put in the `src` directory in each file with the code that is being tested
- The convention is to create a module called `tests` in each file to contain the test functions and to annotated the module with `cfg[test]`

#### The `tests` Module and `#[cfg(test)]`

- The `#[cfg(test)]` annotation on the tests module tells Rust to compile and run the test code only when you run `cargo test`
- This saves compile time when you only want to build the library and saves space in the resulting compiled artifact because the tests are not included
- `cfg` stands for *configuration* and tells Rust that the following item should only be included given a certain configuration option
- In this case the configuration option is `test`
- This includes any helper functions within the `tests` module even though not annotated with `#[test]`

#### Testing Private Functions

- Some languages make it very difficult or even impossible to test private functions
- There’s debate within the testing community about whether or not private functions should be tested directly
- Regardless of the testing ideology you adhere to, Rust’s privacy rules do allow you to test private functions
- For example:
    
    ```rust
    // src/lib.rs
    fn internal_adder(left: i32, right: i32) -> i32 {
        left + right
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
    	
    		#[test]
        fn internal_adder_works() -> Result<(), String> {
            if internal_adder(2, 3) == 5 {
                Ok(())
            } else {
                Err(String::from(format!(
                    "two plus three does not equal five, got {}!",
                    adder(2, 3)
                )))
            }
        }
    }
    ```
    
    Here, we are testing a private function `internal_adder` (not marked with `pub`). This requires no special syntax because `tests` is just another module and items in child modules can use items in their ancestor modules
    
    In this test, we bring all of the `test` module’s parent’s items into scope with `use super::*` and then the test can call `internal_adder`.
    

### Integration Tests

- Entirely external to your library
- They use our library in the same way any other code would, which means they can only call functions that are part of your library’s public API
- Units of code that work well on their own could have problems when integrated, so test coverage of integrated code is important as well
- For integration tests, we first need a `tests` directory at the top level of the `project` directory next to `src`

#### The `tests` directory

- `cargo` looks for integration test files in this directory within which we can create as many test files as we want
- Cargo will compile each of the files as an individual crate
- The overall directory structure would look like this:
    
    ```rust
    adder
    ├── Cargo.lock
    ├── Cargo.toml
    ├── src
    │   └── lib.rs
    └── tests
        └── integration_test.rs
    ```
    
- Inside the `integration_test.rs` file:
    
    ```rust
    use demo;
    
    #[test]
    fn adder_works() {
        assert_eq!(demo::adder(2, 3), 5);
    }
    ```
    
- As this is an external package, we need to bring the function being tested into scope with the `use <package_name>`.
- We do not need to annotate any code with `#[cfg(test)]` because Rust treats the files (and hence, the code) inside the `tests` directory specially and compiles files in this directory only when we run `cargo test`
- When we run `cargo test` now, we get three sections:
    
    ```rust
    running 6 tests
    test tests::adder_works ... ignored
    test tests::internal_adder_works ... ok
    test tests::smaller_cannot_hold_larger ... ok
    test tests::larger_can_hold_smaller ... ok
    test tests::greeting_contains_name ... ok
    test tests::greater_than_limit - should panic ... ok
    
    test result: ok. 5 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s
    
         Running tests/integration_test.rs (target/debug/deps/integration_test-0a1c2381ef0e41b2)
    
    running 1 test
    test adder_works ... ok
    
    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    
       Doc-tests demo
    
    running 0 tests
    
    test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    ```
    
    Here, there are three sections:
    
    1. Unit Tests
    2. Integration Tests
    3. Doc Tests
- These tests are run in order and if any of the test fails in any of the sections, the subsequent tests are not run i.e., if any unit test fails, the integration tests are not run
- To run all the tests in a particular integration test file, use the `--test` argument of `cargo test`:
    
    ```rust
    cargo test --test integration_test
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
         Running tests/integration_test.rs (target/debug/deps/integration_test-0a1c2381ef0e41b2)
    
    running 1 test
    test adder_works ... ok
    
    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    ```
    
    The above runs only the tests in the `tests/integration_test.rs` file
    

#### Submodule in Integration Tests

- As the number of tests grow, you might want to group test functions by the functionality they’re testing
- As each file in the `tests` directory is compiled as its own separate crate, which is useful for creating separate scopes to more closely imitate the way end users will be using your crate
- However, this means files in the `tests` directory don’t share the same behavior as files in `src` do
- This divergence becomes more pronounced when we start adding helper functions/modules inside the `tests` directory
- For example, if we create `tests/common.rs` and place a function named `setup` in it, we can add some code to `setup` that we want to call from multiple test functions in multiple test files
- When we run `cargo test` now, we will get a new section called for `common.rs` that has nothing to do with actual testing:
    
    ```rust
    $ cargo test
       Compiling adder v0.1.0 (file:///projects/adder)
        Finished test [unoptimized + debuginfo] target(s) in 0.89s
         Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)
    
    running 1 test
    test tests::internal ... ok
    
    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    
         Running tests/common.rs (target/debug/deps/common-92948b65e88960b4)
    
    running 0 tests
    
    test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    
         Running tests/integration_test.rs (target/debug/deps/integration_test-92948b65e88960b4)
    
    running 1 test
    test it_adds_two ... ok
    
    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    
       Doc-tests adder
    
    running 0 tests
    
    test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    ```
    
- Note the following section:
    
    ```rust
         Running tests/common.rs (target/debug/deps/common-92948b65e88960b4)
    
    running 0 tests
    ```
    
- Here, 0 tests are run from the `tests/common.rs` file
- Having `common` appear in the test results with `running 0 tests` displayed for it may not be what we wanted
- To prevent this, we change the directory structure from `tests/common.rs` to `tests/common/mod.rs`
- This tells Rust to not treat the `common` module as a test file
- The files in subdirectories of the `tests` directory don’t get compiled as separate crates or have sections in the test output

#### Integration Tests for Binary Crates

- Only library crates expose functions that other crates can use; binary crates are meant to be run on their own and so we can’t create an integration test for them with the approach shown above
- This is one of the reasons Rust projects that provide a binary have a straightforward `src/main.rs` file that calls logic that livfes in the `src/lib.rs` file.
- With this approach, integration tests can test logic in the `lib.rs` file
- If these tests work, then we can be confident that the small amount of code in the `src/main.rs` file works as well.

#### Quiz

1. Which of the following is NOT a reason to wrap unit tests in `#[cfg(test)] mod tests { ... }`?
    - Ans
        
        It gives your tests access to private functions
        
        **Context**: All unit tests in a given file have access to that file's private functions, regardless of being in a `mod tests` or not.
