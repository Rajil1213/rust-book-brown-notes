# Enums and Pattern Matching

## Introduction

- *Enums* or *enumerations* allow you to define a type enumerating its possible `variants`.
- This chapter includes:
    - how to use an enum to encode meaning along with data,
    - the `Option` enum which expresses that a value can be either something or nothing,
    - the `match` expression that makes it easy to run different code for different values of an enum, and
    - the `if..let` construct that lets us handle enums in a convenient and concice form

## Defining an Enum

- Enums give you a way of saying that a value is one of a possible set of values
- For example, if we are working with IP Addresses, it can be either v4 or v6
- Since these are the only two variants, we can enumerate all possible variants:
    
    ```rust
    enum IpAddrKind {
    	V4,
    	V6,
    } 
    ```
    
- Here, `IpAddrKind` is a custom data type that we can use elsewhere in our code.

### Enum Values

- We can create instances of each of the two variants of `IpAddrKind` like this:
    
    ```rust
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    ```
    
- As shown above, the variants are namespaced under their identifier, and we use a double colon (`::`) to separate the two.
- Both `four` and `six` defined above are now of the same type: `IpAddrKind`
- We can define a funciton that takes an `IpAddrKind`:
    
    ```rust
    fn route(ip_kind: IpAddrKind) {}
    ```
    
- We can pass both `four` and `six` to the above function
- We can combine enums and structs to create powerful constructs that allow us to store data:
    
    ```rust
    fn main() {
        #[derive(Debug)]
        enum IpAddrKind {
            V4,
            V6,
        }
    
        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }
    
        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };
    
        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
    
        println!("Home Address = {} of kind: {:?}", home.address, home.kind);
        println!(
            "Loopback Address = {} of kind: {:?}",
            loopback.address, loopback.kind
        );
    }
    ```
    
    The above will output:
    
    ```rust
    Home Address = 127.0.0.1 of kind: V4
    Loopback Address = ::1 of kind: V6
    ```
    
- However, it might be more convenient to store the data directly on the enum because in the above case, the variants of a defined type as well
- A V4 address has four components each of which can be a number from 0 to 255
- A V6 address can be a `String`
- We can represent this with:
    
    ```rust
    fn main() {
        #[derive(Debug)]
        enum IpAddrKind {
            V4(u8, u8, u8, u8),
            V6(String),
        }
    
        let home = IpAddrKind::V4(127, 0, 0, 1);
        let loopback = IpAddrKind::V6(String::from("::1"));
    
        println!("Home = {:?}, Loopback = {:?}", home, loopback);
    }
    ```
    
    This products the following output:
    
    ```rust
    Home = V4(127, 0, 0, 1), Loopback = V6("::1")
    ```
    
    The above would not have been possible with a struct as we want differnt types of `addresses` (`tuple` and `String`) based on the `kind`.
    
- In fact, we can put any data in an enum. The standard library actually puts structs for `Ipv4Addr` and `Ipv6Addr` in the enum for `IpAddr`

### A More Complex Enum

- Let’s take another example of an `enum`, defined as follows:
    
    ```rust
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    ```
    
- Here we have four different variants with four different types:
    - `Quit` - a unit-type enum
    - `Move` - a struct-type enum with named fields
    - `Write` - a `String` type enum
    - `ChangeColor` - a tuple type enum
- We could have just defined four structs with the above types but it would have been very difficult for us to write a function that accepts any of the above types:
    
    ```rust
    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct
    ```
    

### Methods on Enums

- Just like for structs, we can define method on enums with the `impl` block:
    
    ```rust
    impl Message {
    	fn call(&self) {
    		// method body
    	}
    }
    
    let m = Message::Write(String::from("hello"));
    m.call();
    ```
    
- The method uses `&self` to borrow to get he value/variant that we called the method on.

### The `Option` Enum over `null`

- `Option` is another enum provided by the standard library
- It encodes the very commo scenario wherein a value can either be something or nothing.
- For example, getting the first value from a list could return something or nothing (if the list is empty). Encoding this concept in the type system itself allows the compiler to enforce additional safety measures and good coding practices.
- Another key idea in Rust is that it doesn’t include the `null` type that many other languages have. This is a design choice as trying to work with a value that is supposed to be `not-null` but is actually `null` is a common kind of error
- Instead of having `null`, Rust encodes the concept of a value being present or absent via its `Option<T>` enum defined as follows:
    
    ```rust
    enum Option<T> {
    	None,
    	Some(T),
    }
    ```
    
- In fact, this is such a useful enum that it is included in the `prelude` itself and does not need to be brought into the scope of your program.
- The `None` and `Some<T>` variants are also included in the `prelude` so that you don’t have to namespace them with `Option` to be able to use them.
- `<T>` in the above definition is the generic type parameter to be discussed later. For now, it simply means that the `Some` variant of the `Option` enum can hold one piece of data of any type and that each concrete type that gets used in the place of `T` makes the overall `Option<T>` type a different type.
- Examples:
    
    ```rust
    let some_numer = Some(5); // type Option<i32>
    let some_char = Some('e'); // type Option<char>
    let absent_number: Option<i32> = None; // annotation necessary as type cannot be inferred from `None`
    ```
    
- So, why is this better than having `null`?
    - This is because Rust won’t let us compile a code that is of the `Option<T>` type without us explicitly checking both the `Some<T>` and `None` cases. This does not compile:
        
        ```rust
        let x: i8 = 2;
        let y: Option<i8> = Some(5);
        
        let sum = x + y;
        ```
        
        This results in the following error:
        
        ```rust
        Compiling enums v0.1.0 (file:///projects/enums)
        error[E0277]: cannot add `Option<i8>` to `i8`
         --> src/main.rs:5:17
          |
        5 |     let sum = x + y;
          |                 ^ no implementation for `i8 + Option<i8>`
          |
          = help: the trait `Add<Option<i8>>` is not implemented for `i8`
          = help: the following other types implement trait `Add<Rhs>`:
                    <&'a i8 as Add<i8>>
                    <&i8 as Add<&i8>>
                    <i8 as Add<&i8>>
                    <i8 as Add>
        ```
        
    - The above error message simply means that Rust cannot add `i8` and `Option<i8>` types. This in turn, is because Rust is sure about the `i8` type being valid but not about the `Option<i8>` type which *can* be a `None` variant.
    - This safeguard has the implication that any value that can be `null` should be annotated with `Option<T>` and if a value is annotated by `Option<T>`, Rust enforces that you explicity check for the `None` case!
- So, how *do* you extract the `T` from `Some(T)`. For this, we can use the `match` construct.

### Quiz

- Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    fn foo(x: &i32) { 
      println!("{x}");
    }
    fn main() {
      let x = null;
      foo(x);
    }
    ```
    
    ⇒ This **DOES NOT** compile as there is no `null` type in Rust
    
- Consider these two representations of a `Result` type that contains a value `T` if a computation succeeds, or an error `E` if it fails.
    
    ```rust
    struct Result1<T, E> {
      ok: Option<T>,
      err: Option<E>,
    }
    enum Result2<T, E> {
      Ok(T),
      Err(E)
    }
    ```
    
    The enum `Result2` is considered more idiomatic than the struct `Result1` in Rust. Which statement below is **NOT** a valid reason why?
    
    ⇒ The struct contains `Option` types, which are only intended to wrap structs
    
    **Context**: It's perfectly fine to have structs contain `Option` types as fields. But if your data structure has invariants like "exactly one of two optional fields should be `Some`", then that invariant is better ensured by use of an enum.
    

## The `match` Control Flow Construct

- The `match` construct in Rust allows you to compare a value against a series of patterns and execute code based on which pattern matches
- A pattern can be a:
    - Literal
    - Variable name
    - Wildcard, and many other (more on them on a later chapter)
- The power of `match` comes from the expressiveness of the patterns themselves and the fact that the compiler confirms/requires that all possible cases are handled.
- Values go through the each pattern that matches
- Example:
    
    ```rust
    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    
    fn value_in_cents(coin: &Coin) -> u8 {
       // as opposed to `if` `match` can operate on any type
        match coin {
            Coin::Penny => 1, // this is an arm
            Coin::Nickel => 5, // `=>` separates the arm and the code to run
            Coin::Dime => 10,  // one arm is separated from the other with a comma
            Coin::Quarter => {
                println!("You've got a quarter!");
                25
            } // code associated with each arm is an expression which "can" be wrapped in curly braces as well
        }
    }
    
    fn main() {
        let coin = Coin::Dime;
        let cents = value_in_cents(&coin);
    
        println!("{:?} = {cents} cents", coin);
    }
    ```
    
    The above outputs:
    
    ```rust
    Dime = 10 cents
    ```
    
    If we were to remove one of the possible variants from the `match` construct (say, `Nickel`), we’ll get the following error:
    
    ```rust
    error[E0004]: non-exhaustive patterns: `&Coin::Nickel` not covered
      --> src/matching.rs:10:11
       |
    10 |     match coin {
       |           ^^^^ pattern `&Coin::Nickel` not covered
       |
    note: `Coin` defined here
      --> src/matching.rs:4:5
       |
    2  | enum Coin {
       |      ----
    3  |     Penny,
    4  |     Nickel,
       |     ^^^^^^ not covered
       = note: the matched value is of type `&Coin`
    help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
       |
    14 ~         Coin::Quarter => 25,
    15 ~         &Coin::Nickel => todo!(),
       |
    ```
    
    ### Patterns that bind to values
    
    - Within a match expression, we can also extract the value associated with an enum
    - Let’s say we redefine the `Quarter` variant with another enum for US states. Each state from 1999 to 2008 minted quarters with different designs. So, when we sort our loose change, we’ll also call out the name of the state associated with the `Quarter`:
        
        ```rust
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
                Coin::Penny => 1,
                Coin::Nickel => 5, 
                Coin::Dime => 10,  
                Coin::Quarter(state) => {
                    println!("You've got a quarter from {:?}!", state);
                    25
                } 
            }
        }
        
        fn main() {
        		let coin = Coin::Quarter(UsState::Arizona);
            let cents = value_in_cents(&coin);
        
            println!("{:?} = {cents} cents", coin);
        }
        ```
        
    
    ### Matching with `Option<T>`
    
    - We can extract the concrete value in the same way we extracted the state from the quarter for `Option<T>` enum:
        
        ```rust
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(value) => Some(value + 1),
            }
        }
        
        fn main() {
        		let five = Some(5);
            let five_plus_one = plus_one(five);
            let none = plus_one(None);
        
            println!("five plus one = {:?}", five_plus_one);
            println!("none plus one = {:?}", none);
        }
        ```
        
        The above will output:
        
        ```rust
        five plus one = Some(6)
        none plus one = None
        ```
        
    
    ### Catch-all Patterns and the `_` Placeholder
    
    - We can add a pattern that catches all remaining values in the last arm as follows:
        
        ```rust
        let dice_roll: u8 = 4;
        match dice_roll {
        	1 => do_something(),
        	6 => do_something_else(),
        	value => move(value);
        }
        ```
        
        Here, we did not need to exhaust all possible values for `dice_roll`, because the last arm in the `match` expression catches all cases not handled in the previous arms
        
        Technically, this arm can be used at the beginning in which case, all other cases will be ignored and the compiler provides a warning.
        
    - There are cases when we do not want to use the `catch-all` value in which case, we can use the `_` :
        
        ```rust
        let dice_roll: u8: 1;
        match dice_roll {
        	1 => do_something(),
        	6 => do_something_else(),
        	_ => reroll(); // reroll does not use the value, so we use `_` instead
        } 
        ```
        
    
    ### How matches interact with Ownership
    
    - Like for any expression, `match` will take ownership of the variable passed to it depending upon whether the value associated with it implements the `Copy` trait.
    - For example, let’s define an `Option<T>` enum on a `String` type:
        
        ```rust
        let opt: Option<String> = Some(String::from("some string")); // opt: RO, opt@Some.0: RO
        
        match opt {
        	Some(s) => println!("Some: {}", s); // this call to Some(s) takes ownership (move)
        	None => println!("None");
        } // opt:null, opt@Some.0:null
        
        println("{:?}", opt); // opt has moved; error occurs
        ```
        
    - To get around this transferance of ownership, we can pass in a reference to the `match` expression
        
        ```rust
        match &opt {
        	Some(s) => println!("Some: {}", s), // here, Some borrows `s`, this is actually `Some(&s)`
        	None => println!("Some"),
        }
        
        println!("{:?}", opt);
        ```
        
    - Rust will `push down` the reference from the outer enum `&Option<String>` to the inner field `&String`. Therefore, `s` has type `&String` and `opt` can be used after the `match`.

### Quiz

- Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    enum Location {
      Point(i32),
      Range(i32, i32)
    }
    fn main() {
      let l: Location = Location::Range(0, 5);
      let n = match l {
        Location::Point(_) => -1,
        Location::Range(_, n) => n,
        Location::Range(0, _) => 0,
        _ => -2
      };
      println!("{n}");
    }
    ```
    
    ⇒ This program **DOES** compile with output: `5`. The first match from top-to-down executes, in this case the second arm.
    
- Consider this method implemented for the `Option` type:
    
    ```rust
    impl<T> Option<T> {
      fn unwrap_or(self, other: T) -> T {
        match self {
          Some(t) => t,
          None => other
        }
      }
    }
    ```
    
    Which sentence best describes the behavior of this function?
    
    ⇒ Returns the object inside `self` if it exists, and `other` otherwise
    
- Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    #[derive(Debug)]
    enum Either {
      Left(usize),
      Right(String)
    }
    fn main() {
      let x = Either::Right(String::from("Hello world"));
      let value = match x {
        Either::Left(n) => n,
        Either::Right(s) => s.len()
      };
      println!("{x:?} {value}");
    }
    ```
    
    ⇒ This program **does not** compile.
    
    **Context**: The match arm `Either::Right(s)` moves the field `s`, so `x` cannot be used in the `println`.
    
- Consider these two implementations of a function to decrement an unsigned number twice.
    
    ```rust
    fn decr_twice_v1(n: u32) -> Option<u32> {
      match n {
        0 => None,
        1 => None,
        n2 => Some(n2 - 2)
      }
    }
    fn decr_twice_v2(n: u32) -> Option<u32> {
      if n == 0 {
        None
      } else if n == 1 {
        None
      } else {
        Some(n - 2)
      }
    }
    ```
    
    The functions have the same behavior for:
    
    ⇒ All inputs
    

## Concise Control Flow with `if let`

- This pattern allows you to to combine `if` and `let` statements to handle values that match one pattern while ignoring the rest.
- This allows us to replace patterns like this:
    
    ```rust
    let config_max = Some(3u8);
    match config_max {
    	Some(max) => println!("The configured max is: {max}");
    	_ => (),
    }
    ```
    
    with simply:
    
    ```rust
    let config_max = Some(3u8);
    // if let <pattern> = <expression>
    // as opposed to match <expression> { <pattern> => ... }
    if let Some(max) = config_max {
    	println!("The configured max is {max}");
    }
    ```
    
- This results in less typing, less boilerplate and less indentation
- However, as a side effect, we lose the exhaustive checking that `match` enforces.
- We can also use an `else` statement with `if let` if we care about the `catch-all` case as well:
    
    ```rust
    let mut count = 0;
    match coin {
    	Coin::Quarter(state) => println!("State quarter from {:?}", state),
    	_ => count += 1,
    }
    ```
    
    becomes:
    
    ```rust
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
    	println!("State quarter from {:?}", state);
    } else {
    	count += 1;
    }
    ```
    

### Quiz

- Which control flow construct would be most idiomatic to use in the following function?
    
    ```rust
    enum Location {
      Point(i32),
      Range(i32, i32)
    }
    fn print_range_max(loc: &Location) {  
      // print the second field of Range, if loc is a Range
    }
    ```
    
    ⇒ `if..let` since we only care about 1 pattern
    
- Which control flow construct would be most idiomatic to use in the following function?
    
    ```rust
    enum Location {
      Point(i32),
      Range(i32, i32)
    }
    fn get_start(loc: &Location) -> i32 { 
      // return the first field of Range or the only field of Point  
    }
    ```
    
    ⇒ `match` (according to the book but one can argue for `if..let..else` as well?)
