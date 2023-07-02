# Structs

## Introduction

- Used to structure related data
- A custom data type that lets you package together and name related values that make up a meaningful group
- Similar to Objects in OOP

## Defining and Instantiating

- Like tuples, they can hold values of different types
- Unlike tuples, these values can be named which gives them more flexibility
- Example:
    
    ```rust
    struct User {
    	active: bool,
    	username: String,
    	email: String,
    	sign_in_ocunt: u64,
    }
    
    fn main() {
    	let mut user1 = User {
    			email: String::from("someone@somewhere.com"),
    			username: String::from("someusername"),
    			active: true,
    			sign_in_count: 1,
    	},
    }
    ```
    
- The values are stored in either heap or stack depending upon the data type
- As shown above, the order of definition does not have to match the order of declaration
- To access fields in a struct, we use `dot` (`.`) notation:
    
    ```rust
    user1.email = String::from("anotherone@somewhere.com");
    ```
    
- Note that to change any field in a struct, the entire struct has to be mutable. Rust does not allow individual fields to be mutable/immutable.
- Also note that we are using the owned-type `String` instead of something like `&str` because we want each instance of this struct to own all its data and for that data to be valid for as long as the struct is valid.
- There is a way for structs to store references to data owned by something else but to do so requires the use of `lifetimes` (which are discussed much later).

### Field Init Shorthand

- Just like in languages like JavaScript, we can instantiate a field in a struct by condensing the form `<field_name>: <field_name>` with just `<field_name>,`. For example:
    
    ```rust
    fn build_user(email: String, username: String) -> User {
    	User {
    		active: true,
    		username, // :username not necessary
    		email,
    		sign_in_count: 1,
    	}
    }
    ```
    

### Creating a struct from another struct

- Again, just like in languages like JavaScript, we can spread a struct inside another struct and override certain fields that are different:
    
    ```rust
    let user2 = User {
    	email: String::from("someonedifferent@somewhere.com"),
    	..user1
    }
    // this is the same as:
    let user2 = User {
    	email: String::from("someonedifferent@somewhere.com"),
    	active: user1.active,
    	username: user1.username,
    	sign_in_count: user1.sign_in_count,
    }
    ```
    
- Note, however, that with this construct, the `user1` struct has now **moved** and cannot be used!
- This is the case because `email` is of `String` type that does not implement the `Copy` trait and therefore, is moved instead. Copying over types that do implement the `Copy` trait do **not** render `user1` unusable.

### Using Tuple Structs Without Names to Create Different Types

- Tuple structs have added meaning that the Struct type provides but without names associated with their fields — they just have types for their fields
- Example:
    
    ```rust
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    fn main() {
    	let black = Color(0, 0, 0);
    	let origin = Point(0, 0, 0);
    }
    ```
    
- Note that the two structs defined in the above example are of different types even though their underlying representation and values are the same.
- In all other aspects, they behave in the same way that tuples do.

### Unit-like Structs Without Any Fields

- We can also define a struct that does not have any fields
- This type of struct is called `unit-like struct` because the behave similarly to `()`, the unity type.
- Example:
    
    ```rust
    struct AlwaysEqual;
    
    fn main() {
    	let subject = AlwaysEqual;
    }
    ```
    
- The advantage of this type of struct is that we can add behavior to this type of struct without needing to define any explicit fields.
- For example, in the above, we can add a behavior to the struct such that two structs of the `AlwaysEqual` struct are always equal.

### Borrowing Fields of a Struct

- Rust’s borrow-checker will track ownership at both the struct-level and the field-level.
- Example:
    
    ```rust
    struct Point { x: i32, y: i32 };
    
    let mut p = Point { x: 0, y: 0 }; // p: RWO, p.x: RWO, p.x: RWO
    
    let x = &mut p.x; // p: null, p.x: null, p.y: RWO, x: RO, *x: RW
    
    *x += 1; // p: RWO, p.x: RWO, p.y: RWO after scope of x end
    
    println!("{}, {}", p.x, p.y); 
    ```
    
    Here, when we borrow `p.x`, both `p` and `p.x` temporarily lose all their permissions while `p.y` does not. So, if we try to use `p` while it is being borrowed (move `println!` before `*x += 1;`, we get an error:
    
    ```rust
    error[E0502]: cannot borrow `p.x` as immutable because it is also borrowed as mutable
      --> src/main.rs:10:24
       |
    8  |     let x = &mut p.x;
       |             -------- mutable borrow occurs here
    9  |
    10 |     println!("{}, {}", p.x, p.y);
       |                        ^^^ immutable borrow occurs here
    11 |     *x += 1;
       |     ------- mutable borrow later used here
       |
       = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
    ```
    

### Quiz

- Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    struct Point {
      x: i32,
      y: i32,
    }
    fn main() {
      let mut a = Point { x: 1, y: 2 };
      a.x += 1;
      let b = Point { y: 1, ..a };
      a.x += 1;
      println!("{}", b.x);
    }
    ```
    
    The program does compile. `b` has a shallow copy of `a` (primitives). So, the output is `2` (value of `a.x` before initialization of `b`)
    
- Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    struct Point {
      x: i32,
      y: i32,
    }
    fn main() {
      let mut p = Point { x: 1, y: 2 };
      let x = &mut p.x;
      let y = &mut p.y;
      *x += 1;
      *y += 1;
      println!("{} {}", p.x, p.y);
    }
    ```
    
    The program compiles with the output: `2 3`. Rust understands that `.x` refers to a different object than `.y`, so it is valid to take simultaneous mutable references to both fields.
    

## An Example Program Using Structs

- Let’s write a program that calculates the area of a rectangle.
    
    ```rust
    fn main() {
    		let width = 10u32;
        let height = 20u32;
    
        let area = calculate_area(width, height);
        println!("Area of the rectange = {width} x {height} = {area}");
    }
    
    fn calculate_area(width: u32, height: u32) -> u32 {
        width * height
    }
    ```
    
- The above program has a design issue in the `calculate_area` signature in that it is not clear from the signature alone that the two parameters it receives are in fact related (i.e, they are the dimensions of the same object namely a rectangle)
- One way to show that these values are related is to use a tuple for `width` and `height`:
    
    ```rust
    fn calculate_area(dimensions: (u32, u32)) -> u32 {
    	dimensions.0 * dimensions.1
    }
    ```
    
    - This version of the code makes it clear that the two parameters are related but we end up losing the meaning of these parameters because tuples do not allow naming of its members.
    - If we want to know exactly which one is the height and which one is the width, we’d have to remember their corresponding indices, which is not ideal.
- A much better way to do this is to use structs instead:
    
    ```rust
    struct Rect {
        width: u32,
        height: u32,
    }
    
    fn calculate_area(rect: &Rect) -> u32 {
        rect.height * rect.width
    }
    ```
    
    - In the above program, it is clear that the function accepts two **related** parameters as well what those two parameters represent.
    - Also note that we are accepting a reference to the struct and not the copy of it as it is not necessary to own the struct for the sake of the function.

### Adding Useful Functionality with Derived Traits

- It would be useful to print an instance of a `Rect` while debugging our program and see the values for all of its fields.
- In this case, the `println!` macro fails us. If we try to use it, we will get the following error:
    
    ```rust
    error[E0277]: `Rect` doesn't implement `std::fmt::Display`
    ```
    
- Primitive types like an `int32` have the `Display` trait implemented on them since there is only one way to output these primitive types
- For structs, it is a lot less clear how they should be formatted and so `Display` is not implemented on them by default.
- There is another trait called `Debug` that we *can* use. To use this, we need to use `:?` inside the curly braces in the `println!` macro. However, structs do not implement this trait either.
- However, Rust *does* include functionality to print out debugging information but we have to explicitly opt-in to make that functionality available for our struct.
- To do this, we add the outer attribute `#[derive(Debug)]` just before the struct definition:
    
    ```rust
    #[derive(Debug)]
    struct Rect {
        width: u32,
        height: u32,
    }
    
    fn main() {
    
    		let rect = &Rect {
            height: 10u32,
            width: 20u32,
        };
    
        let area = rectangle::calculate_area(rect);
        println!("Area of the rectange {:?} = {area}", rect);
    }
    ```
    
    This will output:
    
    ```rust
    Area of the rectange Rect { width: 20, height: 10 } = 200
    ```
    
- An alternative is to use the `dbg!` macro that:
    - takes ownership of an expression (as opposed to `println!` that takes a reference)
    - prints the file and line number of where that `dbg!` macro was invoked along with the resultant value of that expression, and
    - returns the ownership of the value
    
    Example:
    
    ```rust
    		let scale = 2;
        let rect = &Rect {
            height: dbg!(10u32 * scale),
            width: 20u32,
        };
    
        let area = rectangle::calculate_area(rect);
        println!("Area of the rectange {:?} = {area}", rect);
    ```
    
    The above outputs:
    
    ```rust
    [src/main.rs:21] 10u32 * scale = 20
    Area of the rectange Rect { width: 20, height: 20 } = 400
    ```
    

## Method Syntax

- Methods are similar to *functions*
- However, methods are defined within the context of a struct (or an enum or a trait object) and their first parameter is always `self`.

### Defining Methods

- Let’s change the area function as a method of the `Rect` struct:
    
    ```rust
    #[derive(Debug)]
    struct Rect {
        width: u32,
        height: u32,
    }
    
    impl Rect { // methods are defined within this impl block
        fn area(&self) -> u32 { // &self is short for self: &self
            self.height * self.width
        }
    }
    
    ```
    
    We can now calculate the area simply with:
    
    `rect.area()`
    
- A method can take ownership of `self`, or borrow `self` immutably or mutably
- Here, we use `&self` for the same reason we used `&Rect` before.
- A method taking ownership of `self` is rare except in cases where the method transforms `self` and we need to prevent access to it from the caller until the transformation is complete.
- The main reason for using methods instead of functions that take a struct as input is code organization. By defining functions that apply on the struct within an `impl` block on the struct, we are making it easy for other users of our struct to find functions that they can apply on our struct.

### Naming Methods

- We can create methods with the same name as one of the fields in the struct
- Rust is smart enough to know that if we are trying to call the property with parenthesis, it is a method call.
- The following code is perfectly valid:
    
    ```rust
    impl Rect {
    	fn width(&self) -> bool {
    		self.width > 0
    	}
    }
    ```
    
- Note that, we can use `self.width` directly (instead of `(*self).width` **because Rust has a feature called `automatic referencing and dereferencing` by virtue of which Rust automatically adds in **`&`, `&mut`, or *`*`* so that the call matches the method signature.

### Method with More Parameters

- Let’s define a method called `can_hold` that checks if another rectangle can fit within itself.
- This method should accept the other Rectangle as a parameter to it:
    
    ```rust
    		pub fn can_hold(&self, other: &Rect) -> bool {
            self.width >= other.width && self.height >= other.height
        }
    ```
    
    We can then use it as:
    
    ```rust
    		let scale = 2;
        let rect = Rect {
            height: dbg!(10u32 * scale),
            width: 20u32,
        };
    
        let another_rect = Rect {
            height: 10,
            width: 5,
        };
    
    		println!(
            "{:?} can hold {:?} ? {}",
            rect,
            another_rect,
            rect.can_hold(&another_rect) // `&rect` is inferred
        );
        println!(
            "{:?} can hold {:?} ? {}",
            another_rect,
            rect,
            another_rect.can_hold(&rect) 
        );
    ```
    
    The above will output:
    
    ```rust
    Rect { width: 20, height: 20 } can hold Rect { width: 5, height: 10 } ? true
    Rect { width: 5, height: 10 } can hold Rect { width: 20, height: 20 } ? false
    ```
    

### Associated Functions

- All functions inside an `impl <Type>` block are called *associated functions* as they are associated with the `Type`.
- We can define associated functions that do not have `self` as the first parameter and are thus, **not** methods.
- Such associated functions are generally used as `constructors` that return a new instance of the type:
    
    ```rust
    // associated fn that is **not** a method
        pub fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    ```
    
- We can use the above as follows:
    
    ```rust
    let square_from_rect = Rect::square(10);
        println!(
            "Area of the square {:?} is {}",
            square_from_rect,
            square_from_rect.area()
        );
    ```
    
    The call to `square` is namespaced by the struct: the `::` syntax is used for both associated functions and namespaces created by modules (discussed later)
    

### Multiple `impl` Blocks

- We can have multiple `impl` blocks that implement different associated functions
- This is a perfectly valid syntax although not necessary:
    
    ```rust
    // methods are defined within this impl block
    impl Rect {
        // &self is short for self: &self
        // self is an alias for the type impl block is for
        // in this case `Rect`
        fn area(&self) -> u32 {
            self.height * self.width
        }
    
        fn can_hold(&self, other: &Rect) -> bool {
            self.width >= other.width && self.height >= other.height
        }
    }
    
    impl Rect {
        // associated fn that is **not** a method
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }
    ```
