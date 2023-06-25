# Common Programming Concepts

## Introduction

- Common programming concepts and constructs and how they look like in Rust
- Covers:
    - Variables
    - Basic Types
    - Functions
    - Comments
    - Control Flow

## Variables

### Mutability

- Variables are **immutable by default**
- However, you still have the option to make them mutable with the `mut` keyword
- Take for example, the following program:
    
    ```rust
    fn main() {
    	let x = 6;
    	println!("The value of x = {x}")
    	x = 5;
    	println!("The value of x = {x}")
    }
    ```
    
- The above program does not compile and produces the following error message:
    
    ```rust
    error[E0384]: cannot assign twice to immutable variable `x`
     --> src/main.rs:4:5
      |
    2 |     let x = 6;
      |         -
      |         |
      |         first assignment to `x`
      |         help: consider making this binding mutable: `mut x`
    3 |     println!("The value of x = {x}");
    4 |     x = 5;
      |     ^^^^^ cannot assign twice to immutable variable
    
    For more information about this error, try `rustc --explain E0384`.
    error: could not compile `variables` (bin "variables") due to previous error
    ```
    
- The compiler tells us that we need to use the `mut` keyword if you wish to mutate the variable `x`. We can also run `cargo --explain <error_code>` for a brief explanation of the compilation issue.
- So, to fix the code, we can simply change our declaration of x to:
    
    ```rust
    let mut x = 5;
    ```
    
- With that our code compiles:
    
    ```rust
    Compiling variables v0.1.0 (/Users/rajil/courses/rust/rust-book-brown-notes/variables)
        Finished dev [unoptimized + debuginfo] target(s) in 1.03s
         Running `target/debug/variables`
    The value of x = 6
    The value of x = 5
    ```
    

### Constants

- Like immutable variables, Rust also allows us to define `constant`s that are simply values bound to a variable and are not allowed to change
- There are a few differences between variables and constants:
    - You cannot use `mut` with constants and use the `const` keyword instead of `let`.
    - The type of the value **must** be annotated
    - Constants can be declared in any scope including the *global* scope making them useful for values that many parts of the code need to know about.
    - Constants can only be set to a constant expression, not the result of a value that can only be computed at runtime.
- Example:
    
    ```rust
     const SECONDS_IN_A_DAY: u32 = 24 * 60 * 60;
    ```
    
- Constants are valid for the entire time a program runs, within the scope in which they are declared

<aside>
💡 Naming hardcoded values used throughout your program as constants is useful n conveying the meaning of that value to future maintainers of the code. It also helps to have only one place in your code you would need to change if the hardcoded value needs to be updated in the future.

</aside>

### Shadowing

- We can redeclare a variable in a given scope.
- When a new variable with the same name as the previous variable is declared within the same scope, we say that the new variable shadows the old one.
- The compiler in the latter parts of the code only cares about the second declaration i.e, the second declaration overshadows the first.
- Example:
    
    ```rust
    let y = 10;
    
        let y = y + 1;
    
        {
            let y = y * 2;
            println!("The value of y inside this scope is: {y}")
        }
    
        println!("The value of y outside is: {y}")
    ```
    
    This outputs:
    
    ```rust
    The value of y inside this scope is: 22
    The value of y outside is: 11
    ```
    
- Shadowing is different than `mut` because we require `let` to redeclare/shadow a variable; otherwise we will run into a compilation error
- We can also redeclare variables with a different type:
    
    ```rust
    let spaces = "    ";
        let spaces = spaces.len();
    
        println!("The number of spaces is {spaces}");
    ```
    
- This spares us the trouble of naming the variables `spaces_str` and `spaces_num`.
- The above is not possible with `let mut` because the type of the variable cannot be mutated.

## Data Types

### Introduction

- Every variable in Rust is of a certain data type so that Rust knows how to work with it.
- Rust is statically typed which means that it must know the types of all variables at compile time.
- The compiler is smart enough to infer the type of variable we want to use based on the value.
- In cases where many different types are possible, we must explicitly add the type annotation such as in our guessing game:
    
    ```rust
    let guess: u32 = "42".parse().expect("not a number");
    ```
    
    Removing the type annotation (`: u32`) would result in a compilation error `[E0282]`.
    

### Scalar Types

- They represent a single value.
- Rust has four primary scalar types: `integers`, `floating-point numbers`, `Booleans` and `characters`.

#### Integer

- An integer is a number without a fractional component to it.
- Integer types can be signed (prefix `i`) or unsigned (prefix `u`) ending with their bit size: `8`, `16`, `32`, `64`, `128` or `size`
- Signed means that the number can be negative. These are stored with `two's complement` representation.
- Each signed variant of size `n` can store numbers from $`2^{n-1}`$ to $`2^{n-1} - 1`$, inclusive.
- The `size` variants depend upon the architecture of the computer the program is running on. For example, on a 64-bit architecture, it means the integer is of the 64-bit variant.
- Integer literals can be written in various forms:
    - With a type annotation: `57u8` since `57` can be of many types
    - Visually separated with `_` for readability: `2_999_999` (same as `2999999`)
    - Hex: `0xfff`
    - Octal: `0o777`
    - Binary: `0b111_000` (same as `0b111000`)
    - Byte (`u8` only): `b'A'`
- Integer types default to `u8`.

**Integer Overflow**

- An integer overflow occurs when a variable receives a value that exceeds it set size. For example, trying to store 257 into a variable of type `u8` that can only store numbers from 0 to 255.
- When compiling in `debug` mode, Rust includes checks for these but not when compiling for `release` mode.
- With the `release` build, when an overflow occurs, Rust performs two’s complement wrapping such that in the above example, 256 becomes 0, 257 becomes 1 and so on.
- This leads to unexpected behavior and is considered an error.
- To handle these errors we can use these families of methods provided by the standard library for primitive numeric types:
    - Wrap all modes with the `wrapping_*` methods such as `wrapping_add`
    - Return the `None` value if there is overflow with the `checked_*` methods
    - Return the value and a boolean indicating whether there was an overflow with the `overflowing_*` methods.
    - Saturate the value’s minimum or maximum values with the `saturating_*` methods.
- For example:
    
    ```rust
    use std::io;
    
    fn main() {
       const NUM1: u8 = 10;
    	 const NUM2: u8 = 250;
    
       let total: u8 = NUM1 + NUM2;
    
       println!("{NUM1} + {NUM2} = {total}");
    }
    ```
    
    The above code does not compile and results in:
    
    ```rust
    ╰─ cargo run
       Compiling datatypes v0.1.0 (~/rust-book-brown-notes/datatypes)
    error: this arithmetic operation will overflow
     --> src/main.rs:6:21
      |
    6 |     let total: u8 = NUM1 + NUM2;
      |                     ^^^^^^^^^^^ attempt to compute `10_u8 + 250_u8`, which would overflow
      |
      = note: `#[deny(arithmetic_overflow)]` on by default
    
    error: could not compile `datatypes` (bin "datatypes") due to previous error
    ```
    
- We can “fix” this with:
    
    ```rust
    fn main() {
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
    ```
    
    This produces the output:
    
    ```rust
    10 + 250 = 4
    10 + 250 overflowed
    10 + 250 = 4 with overflow = true
    10 + 250 = 255
    ```
    
    With the values changed to `10` and `240`, the output is:
    
    ```rust
    10 + 240 = 250
    10 + 240 = 250
    10 + 240 = 250 with overflow = false
    10 + 240 = 250
    ```
    

#### Floating-Point

- Rust has two type of floating points: `f32` and `f64`
- `f64` is the default as it is almost as fast as `f32` but has more precision.
- All floating-point values are signed.
- The basic operations such as addition (`+`), subtraction (`-`), multiplication (`*`), division (`*`) and modulo division (`%`) are supported
- When two integers are divided, the result is a truncated integer and not a float.

#### Boolean

- One byte in size
- Two possible values: `true` or `false`

#### Character

- Most primitive alphabetic type
- Example:
    
    ```rust
    fn main() {
        let c = 'z';
        let z: char = 'ℤ'; // with explicit type annotation
        let heart_eyed_cat = '😻';
    }
    ```
    
- Single quotes is a must
- Four bytes in size

### Compound Types

- Can group multiple values into one type

#### Tuples

- General way of grouping a number of values of various types into one compound type
- They have a fixed length: once declared cannot grow or shrink
- We create a tuple by using a comma-separated list of value in parentheses.
- Each position in the tuple has a type each of which do not have be the same.
    
    ```rust
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    ```
    
- A tuple can also be destructured
    
    ```rust
    let (x, y, z) = tup;
    ```
    
- Each element of a tuple can be accessed directly by using a period (`.`) and the index of the value:
    
    ```rust
    let five_hundred = tuple.0;
    ```
    
- A tuple without any value exists and has a special name called `unit` represented by `()`.
- Expressions explicitly return the `unit` value if they do not return any other value.

#### Arrays

- Array is a collection of multiple values of the **same type.**
- They have a fixed length
- We declare them with comma-separate values enclosed by square brackets: `[]`.
- An array is allocated on the stack
- Example:
    
    ```rust
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    ```
    
    Here, `i32` is the type and `5` is the length of the array.
    
- We can also declare an array that has the same value repeated `n` times:
    
    ```rust
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    ```
    
- Elements in an array can be accessed with square brackets that enclose the index:
    
    ```rust
    println!("the first element is {a[0]}")
    ```
    
- If the array index is greater than the length of the array, Rust will panic preventing any unsafe memory access (as in the case of other low-level languages like C).

## Functions

- We can declare functions with the `fn` keyword
- The `main` function is special in that it is the entrypoint to your code.
- Rust uses `snake_case` as the conventional style for function and variable names.
- Special variables that can be passed to a function call as part of the function signature are called `parameters`.
- Technically, the concrete values are called `arguments`
- A function body can contain any valid Rust code

### Statements vs Expressions

- In Rust, these are distinct
- Statements do not return anything. Example:
    
    ```rust
    let x = 5;
    ```
    
- Expressions returns a value that a variable can bind to:
    
    ```rust
    x + 1
    ```
    
    Expressions do not end with a semi-colon. If it does, it becomes a statement.
    
- Example:
    
    ```rust
    fn main() {
    	let y = {
    		let x = 3;
    		x + 1 // this whole block is an expression
    	};
    	println!("The value of y is: {y}");
    }
    ```
    

### Functions with Return Values

- We do not need to name return values but we do need to annotate their type with the arrow (`→`) syntax
    
    ```rust
    fn five() -> i32 {
    	5
    }
    ```
    
- Here, there are no statements just an expression!
- And `i32` is the return value.
- Example:
    
    ```rust
    fn five() -> i32 {
        5
    }
    
    fn plus_one(input: i32) -> i32 {
    		// this expression returns!
        input + 1
    }
    
    pub fn test_functions() {
        let x = five();
        println!("five() -> {x}");
        println!("plus_one(five()) -> {}", plus_one(x))
    }
    ```
    
- Adding `;` after `input + 1` causes a compilation error because the statement `x+1;` returns a unit `()` while the function expects an `i32`.

## Comments

- Comments in Rust are preceded by `//`
- For multi-line comments, we need to include `//` on each line.
- Comments can be written in-line with the code but it is a general convention to add a comment before the line being commented upon.

## Control Flow

### Conditionals (`if`)

- An `if` expression allows us to branch our code depending upon conditions
- Blocks of code associated with an expression are termed `arms`
- We can also include an `else` block for the code to execute when the condition is not met.
- Rust also supports the `else if` construct for multiple condition ladders
- The expression that the `if` evaluates on **must** be a bool.
    
    ```rust
    fn main() {
        let number = 6;
    
        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }
    }
    ```
    
- We can also use `if` in a `let` statement:
    
    ```rust
    let greater_or_equal = if a >= b { a } else { b };
    ```
    
    Note that the [two] return expressions must have the same type.
    

### Loops

- Rust has three kinds of loops: `loop`, `while`, `for`

#### `loop`

- Executes a block of code indefinitely
- We can break out of the loop with the `break;` statement invoked inside the `loop` body.
- We can also use `continue` inside the body to skip over any remaining code
    
    ```rust
    let mut a = 0;
    loop {
    	a += 1;
    
    	if a > 10 {
    		break;
    	}
    
    	if a % 2 == 0 {
    		continue;
    	}
    
    	println!(a);
    }
    ```
    
    The above prints odd numbers from 1 to 10.
    
- We can also return from a `loop` which is normally used when retying an operation:
    
    ```rust
    let result = loop {
    	counter += 1;
    	
    	if counter == 10 {
    		break counter * 2; // breaks and returns
    	}
    }
    ```
    
- We can also add `labels` to loops to distinguish between loops when `break`-ing or `continue`-ing:
    
    ```rust
    fn main() {
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;
    
            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }
    
            count += 1;
        }
        println!("End count = {count}");
    }
    ```
    

#### `while`

- This construct with loop based on a condition:
    
    ```rust
    let mut count = 10;
    while count > 0 {
    	println!("{count}");
    	count -= 1;
    }
    ```
    
- This loop will run until the value of `count` becomes 0.

#### `for`

- This construct allows us to loop over a collection (`array`, `range expression`, etc.)
    
    ```rust
    for number in (1..=10).step_by(2).rev() {
    	println("{number}");
    }
    ```
    
    This is the equivalent of the `while` loop above but is a bit nicer.
    
    The expression `(1..=10)` defines a range from 1 to 10 inclusive
    
    `step_by(2)` adds a step of 2 i.e, the numbers generated by the range expression are: `1,3,5,7,9` that have a difference of two between two consecutive values.
    
    `rev()` simply reverses the order i.e., from 9 to 1 instead of 1 to 9.
