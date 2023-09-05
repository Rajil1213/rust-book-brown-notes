# Patterns and Matching

## Introduction

- *Patterns* are a special syntax for matching against the structure of type — both complex and simple
- A pattern consists of some combination of the following:
    - Literals
    - Destructured arrays, enums, structs or tuples
    - Variables
    - Wildcards
    - Placeholders
    
    For example: `x`, `(a, 3`, `Some(Color::Red)`
    
- In contexts in which patterns are valid, these components describe the shape of the data
- Our program then matches over this shape to decide the control flow
- To use a pattern, we compare it with some valuek and then, execute the corresponding code if there is a match
- This chapter covers:
    - All the valid places to use patterns
    - Difference between refutable and irrefutable patterns
    - Different kinds of pattern syntax

## All the Places Patterns Can Be Used

### `match` Arms

- Patterns can be used in the arms of `match` expressions with the syntax:
    
    ```rust
    match VALUE {
    	PATTERN => EXPRESSION,
    	PATTERN => EXPRESSION,
    	PATTERN => EXPRESSION,
    }
    ```
    
    For example:
    
    ```rust
    fn match_arms() {
        let x: Option<i32> = Some(1);
        let matched = match x {
            None => 0,
            Some(i) => i + 1,
        };
    
        assert!(matched > 1);
    }
    ```
    
- A requirement for `match` expressions is that they must be exhaustive — that is all valid patterns should have a corresponding match arm
- One way to accomplish this without having to check *every* possible value is to have a *catch-all* pattern for the last arm — an arm that matches any value can never fail and thus covers every possible case. This is seen in the above example, where `i` is a *catch-all* for all possible values of `i32`
- The particular pattern `_` will match anything, but it never binds to a variable, so it’s often used in the last match arm
- This pattern is useful when you want to ignore any value not specified. For example:
    
    ```toml
    fn match_arms() {
        let x: Option<i32> = Some(1);
        let matched = match x {
            None => false,
            Some(_) => true,
        };
    
        assert!(matched);
    }
    ```
    
### Conditional `if..let` Expressions

- `if..let` expressiosn can be used to shorten a `match` expression when we only wish to check a particular case
- We can also add a corresponding `else` block that contains code to run if the pattern in the `if...let` doesn’t match
- It’s also possible to mix and match `if..let`, `else..if`, and `else..if..let` expressions.
- This gives us more flexibility than a `match` expression
- Also, Rust doesn’t require that the conditions in a series `if..let`, `else..if`, `else..if..let` arms relate to each other
- For example:
    
    ```rust
    fn if_let_else(favorite_color: Option<&str>, is_tuesday: bool, age: Result<u8, ()>) -> String {
        if let Some(color) = favorite_color {
            format!("Using your favorite color: {} as your background", color)
        } else if is_tuesday {
            String::from("Tuesday is a green day")
        } else if let Ok(age) = age {
            if age > 30 {
                String::from("Using purple as the background color")
            } else {
                String::from("Using orange as the background color")
            }
        } else {
            String::from("Using blue as the background color")
        }
    }
    ```
    
- The above code passes the following tests:
    
    ```rust
    #[test]
    fn test_if_let_else() {
        struct TestCase<'a> {
            favorite_color: Option<&'a str>,
            is_tuesday: bool,
            age: Result<u8, ()>,
            expected: &'a str,
        }
    
        let test_cases: [TestCase; 5] = [
            TestCase {
                favorite_color: None,
                is_tuesday: false,
                age: Result::Err(()),
                expected: "Using blue as the background color",
            },
            TestCase {
                favorite_color: None,
                is_tuesday: false,
                age: Result::Ok(20),
                expected: "Using orange as the background color",
            },
            TestCase {
                favorite_color: None,
                is_tuesday: false,
                age: Result::Ok(31),
                expected: "Using purple as the background color",
            },
            TestCase {
                favorite_color: None,
                is_tuesday: true,
                age: Result::Err(()),
                expected: "Tuesday is a green day",
            },
            TestCase {
                favorite_color: Some("Black"),
                is_tuesday: false,
                age: Result::Err(()),
                expected: "Using your favorite color: Black as your background",
            },
        ];
    
        for test_case in test_cases {
            let result = if_let_else(
                test_case.favorite_color,
                test_case.is_tuesday,
                test_case.age,
            );
            assert_eq!(result, test_case.expected);
        }
    }
    ```
    
- The downside of `if..let` expressions is that the compiler does not check for exhaustiveness the same way it does for `match` expressions
- Even if we were to remove any conditions from our `if..let..else` ladder, the compiler would be perfectly fine with it

### `while..let` Conditional Loops

- This pattern allows a loop to run so long as the pattern matches:
    
    ```rust
    #[test]
    fn test_while_let() {
        let mut stack = Vec::new();
    
        stack.push(1);
        stack.push(2);
        stack.push(3);
    
        let expected = stack.len();
    
        let mut actual: usize = 0;
        while let Some(top) = stack.pop() {
            actual += 1;
        }
    
        assert_eq!(actual, expected);
    }
    ```
    
- Here, the loop terminates when there are no more elements in the `stack` vec to pop — at which point a `None` variant is returned and the `while..let` terminates

### `for` Loops

- In a `for` loop, the variable that directly follows the keyword `for` is a pattern
- For example in `for x in y`, `x` is the pattern
- A `for` loop can also be used to destructure a tuple as part of the `for` loop:
    
    ```rust
    let v = vec!['a', 'b', 'c'];
    
    let mut indices = Vec::new();
    let mut values = Vec::new();
    for (index, value) in v.iter().enumerate() {
        indices.push(index);
        values.push(*value);
    }
    
    assert_eq!(indices, vec![0, 1, 2]);
    assert_eq!(values, v);
    ```
    
    Here, `(index, value)` in the `for` loop destructures the tuple returned from `enumerate()` into its components
    
### `let` Statements

- Consider this straightforward variable assignment:
    
    ```rust
    let x = 5;
    ```
    
- Every time we’ve used this statement, we’ve been using patterns!
- More formally, a `let` statement looks like this:
    
    ```rust
    let PATTERN = EXPRESSION;
    ```
    
- The variable name is a just a particularly simple form of a pattern
- Rust compares the expression against the pattern and assigns any names it finds
- So, in `let x = 5;`, `x` is a pattern that means “bind what matches here to the variable `x`"
- Because the name `x` is the whole pattern, this pattern effectively means “bind everything to the variable `x`, whatever the value is”
- To see the pattern more clearly, see:
    
    ```rust
    let (x, y, z) = (1, 2, 3);
    ```
    
    Here, the pattern is `(x, y, z)` and the expression is `(1, 2, 3)`. Rust sees that the pattern matches the expression and so, it matches `x` with `1`, `y` with `2` and `z` with `3`.
    
    This match would fail, for example, for the statement: `let (x, y) = (1, 2, 3)` because the pattern contains two variables but the expression contains three of them
    
- In case the pattern does not match, we could either ignore the one or more of the values in the tuple using `_` or `..`
- If the problem is that we have too many variables in the pattern, the solution is to make the types match by removing the extra variables

### Function Parameters

- Function parameters can also be patterns
- For example:
    
    ```rust
    fn foo(x: i32) {
    	// function body
    }
    ```
    
    Here, the `x` is a pattern!
    
- Just like with `let`, we could match a tuple in a function’s arguments to the pattern:
    
    ```rust
    fn get_coordinates(&(x, y): &(i32, i32)) -> (i32, i32) {
        (x, y)
    }
    
    let point = (1, 2);
    let (x, y) = get_coordinates(&point);
    
    assert_eq!(x, 1);
    assert_eq!(y, 2);
    ```
    
    Here, we destructure the arguments being passed to `get_coordinates` as they arrive
    
- We can also use patterns in closure parameter lists in the same way as in function parameter lists, because closures are similar to functions

### Quiz

1. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    fn main() {
        let mut v = vec![(1, 2), (3, 4)].into_iter();
        let mut sum = 0;    
        while let Some(t) = v.next() {
            let (_, n) = t;
            sum += n;
        }
        println!("{sum}");
    }
    ```
    
    - Ans

        This program **DOES** compile with the output `6`

        Here, the `let` pattern matches with the tuple in `t` and so, `n` holds the second element of each tuple. The `sum` variable contains the sum of these second elements, which comes out to be `6` i.e, `2 + 4`

        Note that, this could be simplified to:

        ```rust
        while let Some((_, n)) = v.next() {
        	/* ... */
        }
        ```

## Refutability: Whether a Pattern Might Fail to Match

- Patterns come in two forms: refutable and iirefutable
- Patterns that will match for any possible value passed are irrefutable
- For example, `let x = 5;` is an irrefutable pattern because `x` matches anything and so, cannot fail to match
- Patterns that can fail to match for some possible value are refutable
- Some examples of refutable patterns are:
    - In the expression `if let Some(x) = a_value`, then `Some(x)` is refutable because the value in `a_value` can be a `None` variant that does not match with `Some(x)`
    - In the expression `if let &[x, ..] = a_slice`, then `&[x, ..]` is refutable because the value in `a_slice` can have 0 elements that does not match `&[x, ..]` that requires at least 1 element to be present in the slice expression
- Function parameters, `let` statements, and `for` loops can only accept irrefutable patterns because the program cannot do anything meaningful when values don’t match
- The `if..let` and `while..let` expressions accept refutable and irrefutable patterns, but the compiler warns against irrefutable patterns because by definition they’re intended to handle possible failures
- In general, you shouldn’t have to worry about the distinction between refutable and irrefutable patterns — however, you do need to be familiar with the concept of refutability so you can respond to error messages when/if they occur
- In these case, we either need to change the pattern or the construct that is using the pattern
- An example where we use a refutable pattern where Rust expects an irrefutable one is:
    
    ```rust
    let Some(x) = some_optional_value;
    ```
    
    If `some_optional_value` is a `None` variant, this pattern fails to match. So, the pattern is refutable. However, `let` statements can only accept irrefutable patterns because there is nothing valid that the code can do with the `None` value
    
    So, Rust complains at compile time:
    
    ```rust
    $ cargo run
       Compiling patterns v0.1.0 (file:///projects/patterns)
    error[E0005]: refutable pattern in local binding: `None` not covered
     --> src/main.rs:3:9
      |
    3 |     let Some(x) = some_option_value;
      |         ^^^^^^^ pattern `None` not covered
      |
      = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
      = note: for more information, visit https://doc.rust-lang.org/book/ch18-02-refutability.html
    note: `Option<i32>` defined here
     --> /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/core/src/option.rs:518:1
      |
      = note: 
    /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/core/src/option.rs:522:5: not covered
      = note: the matched value is of type `Option<i32>`
    help: you might want to use `if let` to ignore the variant that isn't matched
      |
    3 |     let x = if let Some(x) = some_option_value { x } else { todo!() };
      |     ++++++++++                                 ++++++++++++++++++++++
    help: alternatively, you might want to use let else to handle the variant that isn't matched
      |
    3 |     let Some(x) = some_option_value else { todo!() };
      |                                     ++++++++++++++++
    
    For more information about this error, try `rustc --explain E0005`.
    error: could not compile `patterns` due to previous error
    ```
    
- Because we didn’t cover (and couldn’t cover) every valid value with the pattern `Some(x)`, Rust rightfully produces a compiler error
- If we have a refutable pattern where an irrefutable pattern is needed, we can fix it by changing the code that uses the pattern.
- Here, as the compiler suggests, we can use the `if..let` construct instead of `let`:
    
    ```rust
    if let Some(x) = some_optional_value {
    	prinltn!("{}", x);
    }
    ```
    
    Here, we’ve given the code an out and is perfectly valid
    
- Conversely, if we use an irrefutable pattern with `if..let`, we’ll get a compiler warning:
    
    Code:
    
    ```rust
    if let x = 4 {
    	println!("{}", x);
    };
    ```
    
    Warning:
    
    ```rust
    $ cargo run
       Compiling patterns v0.1.0 (file:///projects/patterns)
    warning: irrefutable `if let` pattern
     --> src/main.rs:2:8
      |
    2 |     if let x = 5 {
      |        ^^^^^^^^^
      |
      = note: this pattern will always match, so the `if let` is useless
      = help: consider replacing the `if let` with a `let`
      = note: `#[warn(irrefutable_let_patterns)]` on by default
    
    warning: `patterns` (bin "patterns") generated 1 warning
        Finished dev [unoptimized + debuginfo] target(s) in 0.39s
         Running `target/debug/patterns`
    5
    ```
    
- So, match arms must use refutable patterns in all of its arms except the last arm that should match any remaining values with an irrefutable pattern
- Rust does allow us to use an irrefutable pattern in a `match` with only one arm but this is not particularly useful and can be replaced with a `let`.

### Quiz

1. Consider pattern-matching on an expression of some type `T`. Which of these statements best describes the difference between a refutable and an irrefutable pattern?
    - Ans

        Refutable patterns do not match some value of type `T`, while irrefutable patterns match all values of type `T`

        **Context**: A pattern is refutable if there exists some value of the expected type that isn't matched by the pattern.

2. Consider the following program:
    
    ```rust
    let x: &[(i32, i32)] = &[(0, 1)];
    ```
    
    Which of the following are **refutable** patterns for `x`?
    
    - Ans
        - `&[(x, y)]`
        - `&[(x, y), ..]`

        A slice does not have a fixed length, so any pattern which asserts that `x` must have at least one element is refutable.

## Pattern Syntax

### Matching Literals

```rust
fn matching_literals(x: i32) -> String {
    match x {
        1 => String::from("Got 1"),
        2 => String::from("Got 2"),
        _ => format!("Got {x}, which is greater than 2"),
    }
}
```

### Matching Named Variables

```rust
fn matching_named_variables(x: Option<i32>, y: i32) -> String {
    match x {
        Some(50) => String::from("Got 50"),
        // a new scope is created here,
        // where the named literal `y` is different than in the fn param
        Some(y) => format!("Matched y = {y}"),
        // here, `y` is the same as the param
        _ => format!("Default case, x = {:?}, y = {y}", x),
    }
}
```

Here are the expected values for various sets of input:

```rust
let test_cases = [
    TestCase {
        x: Some(50),
        y: 10,
        expected: "Got 50",
    },
    TestCase {
        x: Some(10),
        y: 20,
        expected: "Matched y = 10",
    },
    TestCase {
        x: None,
        y: 30,
        expected: &format!("Default case, x = {:?}, y = 30", Option::<i32>::None),
    },
];
```

### Multiple Patterns

```rust
fn multiple_patterns(x: i32) -> String {
    match x {
        1 | 2 => String::from("one or two"),
        3 => String::from("three"),
        _ => String::from("none of 1, 2 and 3"),
    }
}
```

The above matches the following tests:

```rust
struct TestCase<'a> {
    x: i32,
    expected: &'a str,
}

let test_cases = [
    TestCase {
        x: 1,
        expected: "one or two",
    },
    TestCase {
        x: 2,
        expected: "one or two",
    },
    TestCase {
        x: 3,
        expected: "three",
    },
    TestCase {
        x: 5,
        expected: "none of 1, 2 and 3",
    },
];
```

### Matching Ranges of Values with `..=`

```rust
fn matching_ranges(x: i32) -> String {
    match x {
        // equivalent to 1 | 2 | 3 | 4 | 5
        1..=5 => String::from("one through five"),
        _ => String::from("something else"),
    }
}
```

The above passes the following test:

```rust
#[test]
fn test_matching_ranges() {
    let mut counter = 1;
    while counter < 6 {
        let got = matching_ranges(counter);
        counter += 1;
        assert_eq!(got, "one through five");
    }

    let got = matching_ranges(0);
    assert_eq!(got, "something else");

    let got = matching_ranges(6);
    assert_eq!(got, "something else");
}
```

The above pattern can also be used for `char` values such as `'a'..='j'`

### Destructuring to Break Apart Values

#### Destructuring Structs

```rust
struct Point {
    x: i32,
    y: i32,
}
let p = Point { x: 0, y: 2 };

let Point { x: a, y: b } = p;
assert_eq!(0, a);
assert_eq!(2, b);
```

The above example shows that the names of the variables in the pattern don’t have to match the field names of the struct. However, it’s common to match the variable names to the field names to make it easier to remember which variables came from which fields.

Rust provides a shorthand for the pattern: `let Point { x: x, y: y } = p;` as it involves a lot of duplication:

```rust
let Point { x, y } = p; // instead of, `let Point { x: x, y: y } = p;`
```

This is particularly useful when using a `match` expression:

```rust
match p {
    Point { x, y: 0 } => println!("On the x axis at {x}"),
    Point { x: 0, y } => println!("On the y axis at {y}"),
    Point { x, y } => {
        println!("On neither axis: ({x}, {y})");
    }
}
```

#### Destructuring Enums

This is a common pattern that we have seen before:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }
}
```

#### Destructring Nested Structs and Enums

We can also go a level deeper when matching structs and enums:

```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsla(i32, f64, f64, f64),
}

enum Message {
    Quit,
    ChangeColor(Color),
}

fn matching_nested_structs_and_enums(msg: Message) -> String {
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            format!("changing color to red {r}, green {g}, and blue {b}")
        }
        Message::ChangeColor(Color::Hsla(h, s, l, a)) => {
            format!("changing color to hue {h}, saturation {s}, lightness {l}, and alpha {a}")
        }
        _ => String::from("doing something else"),
    }
}
```

The above passes the following tests:

```rust
let test_cases = [
    TestCase {
        msg: Message::ChangeColor(Color::Rgb(10, 20, 30)),
        expected: "changing color to red 10, green 20, and blue 30",
    },
    TestCase {
        msg: Message::ChangeColor(Color::Hsla(10, 0.1, 0.2, 0.3)),
        expected: "changing color to hue 10, saturation 0.1, lightness 0.2, and alpha 0.3",
    },
    TestCase {
        msg: Message::Quit,
        expected: "doing something else",
    },
];
```

#### Destructuring Structs and Tuples

```rust
let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
```

### Ignoring Values in a Pattern

#### Ignoring an Entire Value with `_`

```rust
fn foo(_: i32, y: i32) {
	println!("only using the y param: {y}");
}
```

#### Ignoring Parts of a Value with  a Nested `_`

```rust
let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
```

Here, the use case is that a user should not be allowed to override a setting with a custom setting if the custom setting is already set

#### Ignoring Unused Variable by Starting Its Name with `_`

```rust
let _x = 5;
let y = 10;

println!("y = {y}");
```

Here, `_x` is unused but is fine since the variable name is prepended with `_`. The compiler lets this pass *without* a warning.

Note that even an unused variable still binds to a value:

```rust
let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    println!("{:?}", s);
```

Here, `_s` takes ownership of `s` and so the above code fails. To fix this, we can use `_` which does not bind to the value

#### Ignoring Remaining Parts of a Value with `..`

```rust
struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
```

The above approach is quicker than:

```rust
Point {x, y: _, z: _} => println!("x is {}", x},
```

We can also ignore values in the middle:

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}
```

Note that the usage of `..` must be unambiguous. For example, the following fails because it is not clear which value `second` should bind to:

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        },
    }
}
```

However, this works:

```rust
(_, second, ..) => {
	println!("second value is {second}");
}
```

### Extra Conditionals with Match Guards

- A *match guard* is
    - an additional `if` condition,
    - specified after the pattern in a `match` arm
    - that must alos match for that arm to be chosen
- These are useful when expressing complex ideas than a pattern alone allows
- Example:
    
    ```rust
    fn match_guards(num: Option<i32>) -> String {
        match num {
            Some(x) if x % 2 == 0 => format!("The number {x} is even"),
            Some(x) => format!("The number {x} is odd"),
            None => format!("Number not found"),
        }
    }
    ```
    
    The above passes the following test cases:
    
    ```rust
    let test_cases = [
        TestCase {
            num: Some(2),
            expected: "The number 2 is even",
        },
        TestCase {
            num: Some(1),
            expected: "The number 1 is odd",
        },
        TestCase {
            num: Option::<i32>::None,
            expected: "Number not found",
        },
    ];
    ```
    
- This gives us additional capabilities than what a simple match expression would provide.
- A downside is that when match guards are involved, Rust does not check for exhaustiveness
- This pattern also allows us to now compare the wrapped `Some` value with a variable in the outer scope of the `match` expression:
    
    ```rust
    let x = Some(5);
    let y = 10;
    
    match x {
    	Some(50) => println!("Got 50"),
    	Some(n) if n == y => println!("Matched y = {y}"),
    	_ => println!("Default case, x = {:?}", x),
    }
    ```
    
    Since `n == y` is not a pattern, it does not create a new variable and therefore, does not shadow any variable from the outer scope
    
- We can also use the *or* operator `|` in a match guard to specify multiple patterns — the match guard will then apply to all the patterns:
    
    ```rust
    let x = 4;
    let y = false;
    
    match x {
    	4 | 5 | 6 if y => println!("yes"),
    	_ => println!("no");
    }
    ```
    
    The match arm: `4 | 5 | 6 if y` means that this pattern will match if the value of `x` is one of 4, 5 or 6, *and*  the value of `y` is `true`.
    
### `@` Bindings

- The *at* operator `@` lets us create a variable that holds a value at the same time as we’re testing that value for a pattern match
- For example:
    
    ```rust
    enum Hello {
        Id { id: i32 },
    }
    
    fn at_bindings(hello: Hello) -> String {
        match hello {
            Hello::Id {
                id: id_variable @ 3..=7,
            } => format!("Found an id in range: {id_variable}"),
            Hello::Id { id: 10..=12 } => String::from("Found an id in another range"),
            Hello::Id { id } => format!("Found some other id: {id}"),
        }
    }
    ```
    
- The first arm of the above `match` expression checks whether the `id` value inside the enum struct lies within a range. The `id_variable` captures the value that falls in that pattern
- The second arm shows the case where the at binding is not used. In this arm body, it is not possible to access the internal `id` member while specifying a range because the code going into that pattern doesn’t know what the actual value is (whether it is 10, 11 or 12)
- The third arm shows the case where we are not constraining the range, so we can access the `id` value
- Using `@` lets us test a value and save it a variable within one pattern

### Quiz

1. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    fn main() {
        let x = (0, 1);
        match x {
            (_, y) if y == 0 => println!("A"),
            (0, _) => println!("B"),
            _ => println!("C")
        }
    }
    ```
    
    - Ans

        This program **DOES** compile with the output: `B`

        **Context**: The first branch matches `y = 1`, which means `y != 0`, so the pattern fails. The second branch requires that `x == 0`, which is true, so the pattern succeeds.

2. Consider the following program:
    
    ```rust
    let a = [(0, 1)];
    let ?? = a;
    ```
    
    Which of the following are valid patterns that could replace the `??` placeholder?
    
    - Ans
        - `[(n, ..)]`
        - `[..]`
        - `_`

        **Context**: The pattern `(_, n)` is not valid because `a` is an array of tuples, not a tuple itself.

3. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    fn main() {
        let x = Some(&[0, 1]);
        match x {
            Some(&[.., 1, ..]) => println!("A"),
            Some(&[0, 1]) | None => println!("B"),
            _ => println!("C")
        }
    }
    ```
    
    - Ans

        This program **DOES NOT** compile

        **Context**: It is not valid to use `..` on both sides of a value, since it is ambiguous which array element is being referred to.

4. Say we have a `Point` type with the following definition:
    
    ```rust
    struct Point {
        x: i32,
        y: i32
    }
    ```
    
    Given the following pattern:
    
    ```rust
    Point { x: x @ 0 ..= 5, y } if x + y != 0
    ```
    
    Which of the following values will match this pattern?
    
    - Ans
        - `Point { x: 0, y: 1 }`
        - `Point { x: 5, y: -4 }`

        **Context**: This pattern specifies that `x` must be between 0 and 5 (inclusive), and that `x + y != 0`. Therefore `(5, -4)` and `(0, 1)` are valid, while `(-1, 2)` and `(3, -3)` are not.
