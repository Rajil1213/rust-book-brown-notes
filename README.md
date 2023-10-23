# Advanced Features

## Introduction

- This chapter covers aspects of the language you might run into every once in a while, but may not use every day
- The features in this chapter are useful in very specific situations
- The chapter covers:
    - **Unsafe Rust**: how to opt out of some of Rust’s safety guarantees and take responsibility manually to uphold those guarantees
    - **Advanced Traits**: associated types, default type parameters, fully qualified syntax, supertraits and the newtype pattern in relation to traits
    - **Advanced Types**: more about the newtype pattern, type aliases, the never type, and dynamically sized types
    - **Advanced Functions and Closures**: function pointers and returning closures
    - **Macros**: ways to define code that defines more code at compile time

## Unsafe Rust

### Introduction

- Rust has a second language hidden inside it that does not enforce the safety guarantees that we have seen thus far — called *unsafe Rust*
- Unsafe Rust works just like Rust but it gives us extra superpowers
- It exists because static analysis, by nature, is conservative — when the compiler tries to determine whether or not a code upholds the guarantees, it’s better to reject some valid programs than to accept some invalid programs
- Although the code *might* be okay, in some cases, the Rust compiler doesn’t have enough information to be confident and will reject the code.
- In such cases, we can use *unsafe* Rust, essentially telling Rust to trust us
- Be warned, however, that you should use it at your own risk: if you use unsafe code incorrectly, problems can occur due to memory unsafety, such as null pointer dereferencing
- Another reason for its existence is that computer hardware is inherently unsafe — if Rust didn’t allow some unsafe operations, you couldn’t do certain tasks

### Unsafe Superpowers

- To switch to unsafe Rust, we use the `unsafe` keyword and then start a new block that holds the unsafe code.
- You can take five actions in unsafe Rust that you can’t in safe Rust, which we call `unsafe superpowers`:
    - Dereference a raw pointer
    - Call an unsafe function or method
    - Access or modify a mutable static variable
    - Implement an unsafe trait
    - Access fields of `union`s
- Note that `unsafe` doesn’t turn off the borrow checker or disable any other of Rust’s safety checks: if you use a reference in unsafe code, it will still be checked.
- It only gives you access to these five features that are then not checked by the compiler for memory safety
- It also doesn’t mean that the code inside an `unsafe` block is inherently unsafe — the intent is that the programmer will ensure code inside an `unsafe` block will access memory in a valid way
- Mistakes can occur as they do, but by requiring these five unsafe operations to be done within an `unsafe` block, we’ll know that any errors related to memory safety must be within an `unsafe` block
- A best practice is to wrap unsafe code in a safe API (parts of the standard library behave this way)

### Dereferencing a Raw Pointer

- Unsafe rust has two new types called raw pointers that are similar to references
- As with references, they can be mutable or immutable — `*const T` and `*mut T`
- The asterisk here is not a dereference, it’s part of the type name
- In this context, immutable means that the pointer cannot be directly assigned to after being dereferenced
- They are different from references and smart pointers in that they:
    - Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
    - Aren’t guaranteed to point to valid memory
    - Are allowed to be null
    - Don’t implement any automatic cleanup
- The upside to the lack of Rust’s memory safety guarantees is that we get greater performance and the ability to interface with another language, or hardware
- We can create raw pointers as follows:
    
    ```rust
    let mut num = 5;
    
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    ```
    
- We don’t need an `unsafe` block here but we do need it if we are ever to dereference these raw pointers.
- We've created raw pointers by using `as` to cast an immutable and a mutable reference into their corresponding raw pointer types.
- Because we created them directly, the references are guaranteed to be valid -- but we can't make that assumption about just any raw pointer.
- To demonstrate, take the following:
    
    ```rust
    let address = 0x012345usize;
    let r = address as *const i32;
    ```
    
- The above code creates a raw pointer to an arbitrary location in memory -- there may or may not be data at this location. So this kind of access becomes inherently unsafe.
- To dereference the raw pointers created previously (with valid references), we need an `unsafe` block:
    
    ```rust
    unsafe {
     println!("r1 is {}", *r1);
     println!("r2 is {}", *r2);
    }
    ```
    
- Creating a pointer itself does no harm -- it is when we try to access the value that it points to that we might end up dealing with an invalid value.
- Note also that we created both an immutable and a mutable reference to the same value -- this would not have been possible within the constraints of safe rust.
- One use case for using raw pointers is when interfacing with C code, another case is when building safe abstractions that the borrow checker doesn't understand.

### Calling an Unsafe Function or Method

- Unsafe functions and methods look exactly as regular functions and methods but they have an extra `unsafe` before the rest of the definition
- The `unsafe` keyword indicates that the function has requirements we need to uphold when we call this function.
- By calling an unsafe function within an unsafe block, we’re saying that we’ve read this function’s documentation and take responsibility for upholding the function’s contracts
- An example:
    
    ```rust
    unsafe fn dangerous() {}
    
    unsafe {
    	dangerous();
    }
    ```
    
- Although the above function does nothing unsafe, marking it as such disallows using it outside an unsafe block. If we do, Rust will error out:
    
    ```rust
    error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
     --> src/main.rs:4:5
      |
    4 |     dangerous();
      |     ^^^^^^^^^^^ call to unsafe function
      |
      = note: consult the function's documentation for information on how to avoid undefined behavior
    
    For more information about this error, try `rustc --explain E0133`.
    error: could not compile `unsafe-example` due to previous error
    ```
    
- Bodies of unsafe functions are basically unsafe blocks, so to perform other unsafe operations within an unsafe function, we don’t need to add another unsafe block.
- Note that it is good practice to include a `## Safety` section in the docs for an unsafe API:
    
    ```rust
    /// Does something unsafe.
    ///
    /// ## Safety
    ///
    /// Use at your own risk. 😈
    pub unsafe fn dangerous() -> Result<(), String> {
        Ok(())
    }
    ```
    
#### Creating a Safe Abstraction over Unsafe Code

- Just because a function contains some unsafe code doesn’t mean we need to mark the entire function as unsafe
- In fact, wrapping unsafe code in a safe function is a common abstraction
- As an example, we will try to implement the `split_at_mut` method on strings from the standard library. For the sake of simplicity, we will implement it as a function that acts on `i32` slices
- Our naive approach would land us on the following implementation:
    
    ```rust
    pub fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
    
        assert!(mid <= len);
    
        (&mut values[..mid], &mut values[mid..])
    }
    ```
    
- However, Rust would reject this with the following error:
    
    ```rust
    error[E0499]: cannot borrow `*values` as mutable more than once at a time
      --> src/lib.rs:27:31
       |
    22 | pub fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
       |                             - let's call the lifetime of this reference `'1`
    ...
    27 |     (&mut values[..mid], &mut values[mid..])
       |     --------------------------^^^^^^--------
       |     |     |                   |
       |     |     |                   second mutable borrow occurs here
       |     |     first mutable borrow occurs here
       |     returning this value requires that `*values` is borrowed for `'1`
    
    For more information about this error, try `rustc --explain E0499`.
    error: could not compile `safetyoff` (lib) due to previous error
    ```
    
- Somewhat understandably, Rust’s borrow checker doesn’t know that we are borrowing different parts of the same slice — it just knows that we are borrowing twice mutably from the same slice!
- This is in fact, a safe operation that Rust’s borrow checker isn’t smart enough to elide.
- This is the perfect place for us to flex our Rust superpowers. However, simply wrapping the “erroneous” code in a `unsafe` block will not silence the borrow checker
- To get our implementation working, we will also need to reach for some raw pointers and some unsafe helper functions:
    
    ```rust
    pub fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr: *mut i32 = values.as_mut_ptr();
    
        assert!(mid <= len);
    
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
    ```
    
- We have already seen that slices are just pointers to some data along with the length of the slice.
- The `as_mut_ptr` method allows us to access the raw pointer associated with the slice — it’s return type being the raw pointer: `*mut i32` (as the `values` slice is itself mutable)
- In the unsafe block, the `from_raw_parts_mut` function takes a raw pointer and a length, and it creates a mutable slice out of it that has the size of `mid`.
- We then `add` an offset on the raw pointer to get a raw pointer that starts at `mid` and we create a slice whose length is equal to the remaining elements in the `values` slice.
- The function `from_raw_parts_mut` is unsafe because it takes a raw pointer and must trust that this pointer is valid.
- The `add` method is also unsafe because it must trust that the offset location is also a valid pointer.
- Here, we have created a safe abstraction over an unsafe piece of code — one that we can guarantee is in fact, a safe operation

#### Using `extern` Functions to Call External Code

- Sometimes, your Rust code might need to interact with code written in another language
- For this, Rust has the keyword `extern` that facilitates the creation and use of a *Foreign Function Interface (FFI)*
- An FFI is a way for a programming language to define functions and enable a different (foreign) programming language to call those functions
- Functions declared within `extern` blocks are inherently unsafe as Rust cannot provide safety guarantees on foreign function implementations
- An example usage of the `abs` function from C’s FFI is shown below:
    
    ```rust
    extern "C" {
    	fn abs(input: i32) -> i32;
    }
    
    fn main() {
    	unsafe {
    		println!("Absolute value of -3 according to C: {}", abs(-3));
    	}
    }
    ```
    
- Within the `extern` block, we list the names and signatures of external functions from another language we want to call
- The `"C"` part defines which *application binary interface (ABI)* the external function uses: the ABI defines how to call the function at the assembly level
- The `"C"` ABI is the most common and follows the C programming language’s ABI.
- Inversely, it is also possible to call Rust from other languages using the `extern` keyword (instead of creating an `extern` block) and specify the ABI to use just before the `fn` keyword for the relevant function
- We also need to add the `[no_mangle]` annotation to tell Rust compiler not to mangle the name of this function so that it retains its original name
    - *Mangling* is when a compiler changes the name we’ve given a function to a different name that contains more information for other parts of the compilation process to consume but is less human readable.
- An example is as follows:
    
    ```rust
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }
    ```
    
- Note that the usage of `extern` keyword does not require `unsafe`.

### Access or Modifying a Mutable Static Variable

- A static variable is the equivalent of a global variable in other languages
- Due to Rust’s ownership rules, global variables can be a bit problematic — for example, if two threads are accessing the seem mutable global variable, it can cause a data race
- So accessing/modifying a mutable static variable is `unsafe` in Rust, while accessing an immutable one is safe.
- We can declare a static variable with:
    
    ```rust
    static HELLO_WORLD: &str = "Hello, world!";
    ```
    
    The convention is to name these with `SCREAMING_SNAKE_CASE`
    
- Note that static variables can only store values with a `static` lifetime, and we do not need to annotate the lifetime explicitly
- Immutable static variables are similar to constants but have a subtle difference in that values in a static variable have a fixed address in memory — using the value will always access the same data
- Constants, however, are allowed to duplicate their data whenever they’re used
- Here’s how we can mutate static variables:
    
    ```rust
    static mut COUNTER: u32 = 0;
    
    fn add_to_count(inc: u32) {
    	unsafe {
    		COUNTER += inc;
    	}
    }
    
    fn main() {
    	add_to_count(3);
    	
    	unsafe {
    		println!("COUNTER: {}", COUNTER);
    	}
    }
    ```
    
    Note that we need an `unsafe` block to both mutate and access the static variable.
    
### Implementing an Unsafe Trait

- A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify
- We define an unsafe trait with the following syntax:
    
    ```rust
    unsafe trait Foo {
    	// methods go here
    }
    
    unsafe impl Foo for i32 {
    	// method implemenation go here
    }
    
    fn main() {}
    ```
    
- As an example, recall the `Sync` and `Send` traits for the standard library. The compiler implements these traits automatically if our types are composed entirely of `Send` and `Sync` types
- If we want to mark a type that is not `Send` or `Sync` (such as raw pointers) as `Send` or `Sync`, we must use `unsafe`
- For such types, Rust cannot verify thread-safety on its own. So, we must do that manually and mark the types as such.

### Accessing Fields of a Union

- A `union` is similar to a `struct`, but only one declared field is used in a particular instance at one time.
- These are primarily used to interface with unions in C code.
- Accessing fields of a union is unsafe in Rust because Rust cannot guarantee the type of the data currently being stored in the union instance.
- More on `union`s in the [Rust Reference](https://doc.rust-lang.org/reference/items/unions.html).

### Quiz

1. Which of the following are "superpowers" that Rust enables inside an `unsafe` block?
    - Ans
        - Calling a function marked as `unsafe`
        - Dereferencing a raw pointer
2. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    fn main() {
        let mut v = Vec::with_capacity(4);
        for i in 0 .. 3 { 
            v.push(i); 
        }
        let n = &v[0] as *const i32;
        v.push(4);
        println!("{}", unsafe { *n });
    }
    ```
    
    - Ans

        This program does compiler, and outputs: `0`

        **Context**: This program is dangerous! It compiles correctly and executes without issue because `Vec` has enough capacity such that `v.push(4)` does not resize it. However, if the capacity were 3, then `n` would point to deallocated memory.

3. Which of the following are situations where using `unsafe` code (or a safe wrapper around `unsafe` code) is an idiomatic method for working around the borrow checker?
    - Ans
        - Getting two mutable references to disjoint indices in an array
        - Allowing values to be uninitialized when they are not being read
        - Having a reference to one field of a struct sit in another field of the same struct

        **Context**: Two mutable references to disjoint indices is reasonable because the borrow checker doesn't understand when indices are disjoint. See: `[slice::split_at_mut](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.split_at_mut)`.

        Allowing values to be uninitialized is reasonable because the borrow checker adopts a conservative policy that values must be always initialized, but it's sometimes useful to work with uninitialized values. See `[MaybeUninit](https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html)` and the `[take_mut](https://docs.rs/take_mut/0.2.2/take_mut/index.html)` crate.

        Having a struct hold a reference to itself is reasonable because the borrow checker cannot reason about the lifetime of self-references. See the `[std::pin](https://doc.rust-lang.org/stable/std/pin/index.html)` module.

        However, returning a pointer to a stack-allocated variable is *never* valid to do with unsafe code. The only appropriate workaround is to use garbage collection, e.g. returning an `Rc<T>` instead of `&T`.

## Advanced Traits

### Specifying Placeholder Types in Trait Definitions with Associated Types

- *Associated Types* connect a type placeholder with a trait such that the trait method definitions can use these placeholder in their signatures
- The implementor of the trait will specify the concrete type to be used instead of the placeholder type for the particular implementation
- This allows us to define a trait that uses some types without needing to know exactly what those types are *until* the trait is implemented
- While most of the features in this chapter are rarely needed, associated types fall somewhere in the middle (less used that features in other chapters, but more used than features in this chapter)
- One example is the `Iterator` trait with the associated type named `Item` that stands in for the type of the values the type implementing the `Iterator` trait is iterating over:
    
    ```rust
    pub trait Iterator {
    	type Item;
    	
    	fn next(&mut self) -> Option<Self::Item>;
    }
    ```
    
    Here,
    
    - The `type Item` is a placeholder, and the `next` method’s definition shows that it will return values of the type `Option<Self::Item>`
    - Implementors of the trait will specify the concrete type:

        ```rust
        impl Iterator for Counter {
        	type Item = u32;
        
        	fn next(&mut self) -> Option<Self::Item> {
        		...
        	}
        }
        ```

- The above seems similar to the generics syntax:
    
    ```rust
    pub trait Iterator<T> {
    	fn next(&mut self) -> Option<T>;
    }
    ```
    
- The difference is that when using generics, we must annotate the types in each implementation, since we can also implement `Iterator<String> for Counter` or any other type, we could have multiple implementations of `Iterator` for `Counter`
- In other words, when a trait has a generic parameter, it can be implemented for a type multiple times, changing the concrete types of the generic type parameters each time
- When we use the `next` method on `Counter`, we would have to provide type annotations to indicate which implementation of `Iterator` we want to use
- With associated types, we don’t need to annotate types because we can’t implement a trait on a type multiple times.
- That is, there can only be one implementation of `Iterator` associated with `Counter`
- Associated types also become a part of the trait’s contract: implementors of the trait must provide a type to stand in for the associated type behavior
- Documenting the associated type in the API documentation is a good practice

### Default Generic Type Parameters and Operator Overloading

- When we use generic type parameters, we can specify a default concrete type for the generic type
- This eliminates the need for implementors of the trait to specify a concrete type if the default type works
- This is done with the `<PlaceholderType=ConcreteType>` syntax
- A great example of where this technique is useful is with *operator overloading*, in which you customize the behavior of an operator (such as `+`) in particular situations
- While Rust doesn’t allow us to define our own operators or overload arbitrary operators, you can overload the operations and corresponding traits listed in `std::ops`:
    
    ```rust
    use std::ops::Add;
    
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    impl Add for Point {
        type Output = Point;
    
        fn add(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }
    
    #[cfg(test)]
    mod tests {
    
        use super::*;
    
        #[test]
        fn adding_points_works() {
            assert_eq!(
                Point { x: 1, y: 2 } + Point { x: 2, y: 1 },
                Point { x: 3, y: 3 }
            );
        }
    }
    ```
    
- Here, the `add` method adds the `x` values of two `Point` instances and `y` values of two `Point` instances to create a new `Point` instance
- The `Add` trait in the standard library has associated type called `Output` that determins the type returned from the `add` method
- The definition of `Add` looks as follows:
    
    ```rust
    #[doc(alias = "+")]
    pub trait Add<Rhs = Self> {
        /// The resulting type after applying the `+` operator.
        #[stable(feature = "rust1", since = "1.0.0")]
        type Output;
    
        /// Performs the `+` operation.
        ///
        /// ## Example
        ///
        /// ```
        /// assert_eq!(12 + 1, 13);
        /// ```
        #[must_use = "this returns the result of the operation, without modifying the original"]
        #[rustc_diagnostic_item = "add"]
        #[stable(feature = "rust1", since = "1.0.0")]
        fn add(self, rhs: Rhs) -> Self::Output;
    }
    ```
    
    The interesting part here is the `<Rhs = Self>` syntax after the trait name `Add` — this syntax is called the `default type parameters`.
    
- The `Rhs` generic type parameter defines the type of the `rhs` parameter in the `add` parameter
- If we do not specify a type for `Rhs`, it will default to `Self` which will be the type we will be implementing `Add` on
- Now, let’s try to implement `Add` such that `Rhs` is not the same as `Self`.
- We’ll demonstrate this by adding two tuple structs: `Millimeters` and `Meters` both of which wrap a `u32` ⇒ this thin wrapping of an existing type in another struct is known as the `newtype` pattern:
    
    ```rust
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Millimeters(u32);
    struct Meters(u32);
    
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;
    
        fn add(self, rhs: Meters) -> Self::Output {
            Millimeters(self.0 + rhs.0 * 1000)
        }
    }
    ```
    
- Here, to add `Millimeters` with `Meters`, we specify `Rhs` to be of type `Meters` — the thing we want to add to our `Self`, `Millimeters`
- This overrides the default `Rhs` type of `Self`, allowing us to add dissimilar types together
- This allows us to extend trait functionality such that we can have multiple trait implementations — acting on different generics

### Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

- Nothing in Rust prevents a trait from having a method with the same name as another trait’s method, nor does Rust prevent you from implementing both traits on the same type
- Furthermore, it is also possible to implement a method with the same name as the method available on the trait that it implements
- The catch here is that when calling on these methods with the same name, you need to tell Rust which method you are calling exactly
- For example:
    
    ```rust
    trait Pilot {
        fn fly(&self) -> String;
    }
    
    trait Wizard {
        fn fly(&self) -> String;
    }
    
    struct Human;
    
    impl Pilot for Human {
        fn fly(&self) -> String {
            format!("This is your captain speaking!")
        }
    }
    
    impl Wizard for Human {
        fn fly(&self) -> String {
            format!("Up!")
        }
    }
    
    impl Human {
        fn fly(&self) -> String {
            format!("*waves arms furiously*")
        }
    }
    ```
    
- Here, we use three different methods on the `Human` struct all named `fly`!
- If we were to just call `fly` on an instance of `Human`, we would get the method implemented directly on the `Human`:
    
    ```rust
    #[test]
    fn same_named_methods_work() {
        let human: Human = Human {};
        assert_eq!(human.fly(), "*waves arms furiously*".to_string());
    }
    ```
    
- To call on the other methods, we need to specify the exact trait that we want to invoke `fly` from:
    
    ```rust
    #[test]
    fn same_named_methods_work() {
        let human: Human = Human {};
        assert_eq!(human.fly(), "*waves arms furiously*".to_string());
        assert_eq!(
            Pilot::fly(&human),
            "This is your captain speaking!".to_string()
        );
        assert_eq!(Wizard::fly(&human), "Up!".to_string());
    }
    ```
    
- Rust was able to figure out that the `fly` method in our first invocation belonged to the native `Human` method because it had a self parameter.
- Because the `fly` method takes a `self` parameter in each of the above implementations, Rust was able to figure out the right implementation to invoke.
- In the absence of a `self` parameter, Rust is not able to infer the right type. Take for example, the following:
    
    ```rust
    trait Animal {
        fn baby_name() -> String;
    }
    
    struct Dog;
    
    impl Dog {
        fn baby_name() -> String {
            format!("Spot")
        }
    }
    
    impl Animal for Dog {
        fn baby_name() -> String {
            format!("puppy")
        }
    }
    ```
    
    Here, the `baby_name` method does not take a self parameter.
    
- We can invoke the `baby_name` method directly defined on `Dog` using: `Dog::baby_name()` but how do we invoke the method defined in `Animal`?
- We *could* use: `Animal::baby_name()` but that would result in a compilation error:
    
    ```rust
    error[E0790]: cannot call associated function on trait without specifying the corresponding `impl` type
       --> src/lib.rs:113:20
        |
    55  |     fn baby_name() -> String;
        |     ------------------------- `Animal::baby_name` defined here
    ...
    113 |         assert_eq!(Animal::baby_name(), "puppy");
        |                    ^^^^^^^^^^^^^^^^^ cannot call associated function of trait
        |
    help: use the fully-qualified path to the only available implementation
        |
    113 |         assert_eq!(<Dog as Animal>::baby_name(), "puppy");
        |                    +++++++       +
    
    For more information about this error, try `rustc --explain E0790`.
    ```
    
    This makes sense because the `Animal` trait could have been implemented on multiple types. So, we need to be more specific, as shown in the `help` instructions above:
    
    ```rust
    #[test]
    fn same_named_methods_without_self_work() {
        assert_eq!(Dog::baby_name(), "Spot");
        assert_eq!(<Dog as Animal>::baby_name(), "puppy");
    }
    ```
    
    This is called a fully qualified syntax
    
- In general, the fully qualified syntax looks like the following:
    
    ```rust
    <Type as Trait>::function(receiver_if_method, next_arg, ...);
    ```
    
    We could have used this syntax with the `fly` method above as well, but we can ignore any boilerplate that Rust can infer on its own
    
### Using Supertraits to Require One Trait’s Functionality Within Another Trait

- Sometimes, we might write a trait definition that depends on another trait: for a type to implement the first trait, we might want to require the type to also implement the second trait
- This might also be so that we can make use of the associated items of the second trait
- In this scenario, the trait that our trait relies on is called a `supertrait` of our trait
- For example, let’s say we have a trait that displays a value in such a way that it is framed by asterisks:
    
    ```rust
    **********
    *        *
    * (1, 3) *
    *        *
    **********
    ```
    
    This value can be anything but essentially, must implement the `Display` trait from the standard library — required to print out the value.
    
- So, we require that any type that implements our trait to first implement the `Display` trait. We add this constraint as so:
    
    ```rust
    // Super trait is fmt::Display
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
    
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }
    
    impl Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    
    impl OutlinePrint for Point {}
    ```
    
    Here, we need to first implement `Display` on `Point` before we can implement our `OutlinePrint` trait. Without the implementation of `Display`, we will get the following error:
    
    ```rust
    rustc: `Point` doesn't implement `std::fmt::Display`
    the trait `std::fmt::Display` is not implemented for `Point`
    in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead [E0277]
    ```
    
### Using the Newtype Pattern to Implement External Traits on External Types

- We’e already come across the *orphan rule* which states that we’re only allowed to implement a trait on a type if either the trait or the type are local to our crate
- We *can* get around this with the *newtype pattern*, which involves creating a new type in a tuple struct
- The tuple struct will have one field and be a thin wrapper around the type we want to implement a trait for
- This wrapper will then be local to our crate allowing us to implement an arbitrary trait on it
- The term *Newtype* originates fro the Haskell programming language
- There is no runtime penalty of using this pattern, and the wrapper type is elided at compile time
- As an example, let’s implement the Display Trait on the `Vec<T>` type:
    
    ```rust
    // Newtype pattern
    struct Wrapper(Vec<String>);
    
    impl Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
    ```
    
    ```rust
    #[test]                                                                                          
    fn newtype_works() {                                                                             
        let wrapper_vec = Wrapper(vec!["a".to_string(), "b".to_string(), "c".to_string()]); 
        println!("{wrapper_vec}");                                                                   
    }
    ```
    
    When we run the above test, we get the following output:
    
    ```rust
    [a, b, c]
    ```
    
- The downside of this pattern is that we lose all of the functionality of the underlying type
- To get back these functionalities, we would have to implement the methods ourselves, delegating the actual implementation to the underlying type.
- One solution would be to implement the `Deref` trait on the wrapper to return the underlying type which would then allow us to access the methods on the underlying type by dereferencing the wrapper
- We might also choose to implement a subset of methods directly on the Wrapper so as to allow only some of the methods in our crate.

### Quiz

1. Recall the definition of the `Add` trait:
    
    ```rust
    trait Add<Rhs=Self> {
        type Output;
        fn add(self, rhs: Rhs) -> Self::Output;
    }
    ```
    
    Which of the following best describes why `Output` is an associated type, while `Rhs` is a type parameter?
    
    - Ans

        A type `T` should be addable to many other types `S`, but a given `T + S` operation should always have a single output type

        **Additional Context**: It is true that an associated type cannot currently have a default, but that is not the main motivating factor for having `Rhs` be a type parameter in the `Add` trait.

2. Which of the following best describes why `Rhs` is a type parameter to the trait `Add` rather than the function `add`? That is, why is `Add` not designed like this:
    
    ```rust
    trait Add {
        type Output;
        fn add<Rhs>(self, rhs: Rhs) -> Self::Output;
    }
    ```
    
    - Ans

        If `Rhs` were a function-level type parameter, then the definition of `add` could not assume any structure to `Rhs`

3. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    mod inner {
        pub trait A {
            fn f(&self) -> usize { 0 }
        }
        pub trait B {
            fn f(&self) -> usize { 1 }
        }
        pub struct P;
        impl A for P {}
        impl B for P {}
    }
    fn main() {
        use inner::{P, B};    
        println!("{}", P.f());    
    }
    ```
    
    - Ans

        This program **DOES** compile with the output: `1`.

4. Consider implementing a trait `Trait` for a type `T`. In which of the following situations do you need to wrap `T` in a newtype?
    - Ans

        `Trait` is defined in an external crate and `T` is defined in an external crate

## Advanced Types

### Using the `Newtype` Pattern for Type Safety and Abstraction

- Besides the use case of getting around the *orphan rule* as discussed above, the *newtype pattern* has some other use cases such as:
- Statically enforcing that values are never confused and indicating the units of a value
    - An example of this is an early example where we use `Millimeters` and `Meters` structs to wrap `u32` values in a newtype
    - This ensures that a function that accepts `Millimeters` does not accept `Meters` although the underlying type in both cases is the same.
- Abstracting away some implementation details of a type
    - The new type can expose a public API that is different from the API of the private inner type
- Hiding internal implementation
    - We could provide a `People` type to wrap a `HashMap<i32, String>` that stores a person’s ID associated with their name
    - Code using `People` would only interact with the public API we provide, such as a method to add a name string to the `People` collection; that code wouldn’t need to know that we assign an `i32` ID to names internally
    - This also provides a way for us to achieve encapsulation in a lightweight way

### Creating Type Synonyms with Type Aliases

- Rust allows us to declare a *type alias* to given an existing type another name.
- For this, we use the `type` keyword. For example:
    
    ```rust
    type Kilometers = i32;
    ```
    
- This is different from the new type `Kilometers(i32)` because it does not create a separate, new type. `Kilometers` will be treated in the same way as `i32` is:
    
    ```rust
    let x: i32 = 5;
    let y: Kilometers = 10;
    
    assert_eq!(5 + 10, x + y);
    ```
    
- This also means that we won’t get strict type checking as with the *newtype* pattern — a function that accepts an `i32` will also accept a `Kilometers` type
- The benefit/use case of this pattern is to factor out long types. For example:
    
    ```rust
    // *thunk* is a wrod for code to be evaluated at a later time
    type Thunk = Box<dyn Fn() + Send + 'static>;
    
    let f: Thunk = Box::new(|| println!("hi"));
    
    fn takes_long_type(f: Thunk) {
    	...
    }
    
    fn returns_long_type() -> Thunk {
    	...
    }
    ```
    
- Choosing meaningful aliases for long types can make the code more readable
- Type aliases are often used with the `Result<T, E>` type for reducing repetition
- For example, the `std::io` library uses the type alias:
    
    ```rust
    type Result<T> = std::result::Result<T, std::io::Error>;
    ```
    
    which allows function definitions to be written succinctly as:
    
    ```rust
    pub trait Write {
    	fn write(&mut self, buf: &[u8]) -> Result<usize>;
    	...
    }
    ```
    
- This gives a convenient syntax and better ergonomics as underneath the hood, it is still a `Result<T, E>`

### The Never Type that Never Returns

- Rust has a special type, that in type theory lingo, is called the *empty type* because it has no values
- In Rust, it is called the `never` type because it is a stand-in for the return type of a function that never returns.
- For example:
    
    ```rust
    // the function bar returns never
    fn bar() -> ! {
    	...
    }
    ```
    
- These types of functions are called *diverging functions*
- We can’t ever create values of `!`, so it’s not possible for `bar` to return anything
- The use case for this type is a bit subtle
- For example, we discussed in previous chapters that the all match arms must return the same type. An example is the guessing game:
    
    ```rust
    let guess: u32 = match guess.trim().parse() {
    	Ok(num) => num,
    	Err(_) => continue,
    };
    ```
    
- How were we then, allowed to return a `u32` from one arm and have another arm end in `continue`?
- The answer is that `continue` expression has the never type!
- Because the second arm cannot return a value, Rust decides that `guess` has the type `u32`.
- The formal way of describing this behavior is that expressions of type `!` can be *coerced* into any other type
- The never type is also useful with the `panic!` macro:
    
    ```rust
    impl<T> Option<T> {
    	pub fn unwrap(self) -> T {
    		match self {
    			Some(val) => val,
    			None => panic!("called `Option::unwrap()` on a `None` value"),
    		}
    	}
    }
    ```
    
- Another expression with the never type is an infinite `loop`:
    
    ```rust
    loop {
    	print!("on and on...");
    }
    ```
    
    This loop never ends so it can never return anything. This would not be true if there was a `break` statement inside the `loop`.
    
### Dynamically Sized Types and the `Sized` Trait

- Rust needs to know certain details about its types, such as how much space to allocate for a value of a particular type
- This can get confusing when considering dynamically sized types (or *DST*s or *unsized* types)
- An example of a dynamically sized type is the `str` type (not to be confused with `&str`)
- We can’t know how long a `str` is until runtime, meaning we can’t create a variable of type `str`, nor can we take an argument of type `str`:
    
    ```rust
    // this fails to compile
    let s1: str = "Hello, world";
    let s2: str = "How's it going?";
    ```
    
- Rust needs to know how much memory to allocate for any value of a particular type, and all values of a type must use the same amount of memory
- If Rust allowed the above code, both `s1` and `s2` would need to take up the same amount of space (as they’re both of the same type, `str`)!
- So, we make both `s1` and `s2` of the type `&str` instead of `str`
- The reference type `&str` only stores the starting position and the length of the slice
- As such, we can know the size of an `&str` value at compile time: it’s twice the length of a `usize`
- This is, in general, the way in which DSTs are used in Rust: they have an extra bit of metadata that stores the size of the dynamic information
- The golden rule of DSTs is that we always need to put values of a DST behind a pointer of some kind
- So, `str` can be combined with `Box<str>`, `Rc<str>` or any other pointer type
- We have also seen this with another DST, namely `traits` when we used `&dyn Trait` (or `Box<dyn Trait>`)
- To work with DSTs, Rust provides the `Sized` trait to determine whether or not a type’s size is known at compile time — this trait is automatically implemented for everything whose size is known at compile time
- Rust also automatically adds a bound on generics as well:
    
    ```rust
    fn generic<T>(t: T) {
    	...
    }
    // is actually
    fn generic<T>(t: T)
    where
    	T: Sized
    {
    	...
    }
    ```
    
- To opt out of this restriction, we can use:
    
    ```rust
    fn generic<T>(t: &T) 
    where 
    	T: ?Sized
    {
    	...
    }
    ```
    
- A trait bound on `?Sized` means “`T` may or may not be `Sized`" and this notation overrides the default that generic types must have a known size at compile time.
- The `?Trait` syntax with this meaning is only available for `Sized`
- Also note that we switched the type from `T` to `&T` because the type might not be `Sized` and so we need to use it behind some kind of pointer

### Quiz

1. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    fn expect_none(x: Option<i32>) -> ! {
        match x {
            Some(n) => panic!("Expected none, found Some({n})"),
            None => ()
        }
    }
    fn main() {
        println!("{:?}", expect_none(None));
    }
    ```
    
    - Ans

        This program does not compile because the second arm of the match expression returns `unit` type which is not allowed in a function that returns `never`

2. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    fn is_equal<T: Eq>(t1: &T, t2: &T) -> bool {
      t1 == t2
    }
    fn main() {
      println!("{}", is_equal("Hello", "world"));
    }
    ```
    
    - Ans

        This program does not compile.

        **Context**: The call to `is_equal` passes values of type `&str`. That means `T = str`. However, `str` is only allowed to be used in such a generic function if `T` is marked as `?Sized`.

## Advanced Functions and Closures

### Function Pointers

- In addition to closures, we can also pass regular functions *to* functions!
- This technique is useful when we want to pass a function we have already defined to a function rather than reimplementing the function as a closure
- Functions coerce to the type `fn` (not to be confused with the `Fn` closure trait)
- The `fn` type is called a *function pointer*
- The syntax for specifying that a parameter is a function pointer is similar to that of closures:
    
    ```rust
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }
    
    pub fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    
    #[cfg(test)]
    mod tests {
    
        use super::*;
    
        #[test]
        fn do_twice_works() {
            let x = 0;
            assert_eq!(2, do_twice(add_one, x));
        }
    }
    ```
    
- Unlike closures, `fn` is a type rather than a trait, so we specify `fn` as the parameter type directly rather than declaring a generic type parameter with one of the `Fn` traits a a trait bound
- Function pointers implement all three of the closure traits namely, `Fn`, `FnMut`, `FnOnce` — meaning that you can always  pass a function pointer where a function closure is expected. For example:
    
    ```rust
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();
    
    // or,
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();
    
    // Also,
    enum Status {
        Value(u32),
        Stop,
    }
    
    // the variant is also an initializer function
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    ```
    
- It’s best to write functions whose signature contains a closure generic so that its user can pass both a function pointer and the appropriate closure to it.
- However, one case where you would want to accept just function pointers and not closures is when interfacing with external code that does not have closures (such as C)

### Returning Closures

- Closures are represented by traits, which means they cannot be returned directly from functions
- In most cases, you might want to return a concrete type that implements the trait rather than the trait itself
- However, this cannot be done for closures because they don’t have a concrete type that is returnable
- For example, the following code does not compile:
    
    ```rust
    fn returns_closure() -> dyn Fn(i32) -> i32 {
    	|x| x + 1
    }
    ```
    
    We get the following error message:
    
    ```rust
    --> src/lib.rs:1:25
      |
    1 | fn returns_closure() -> dyn Fn(i32) -> i32 {
      |                         ^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
      |
      = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
    help: use `impl Fn(i32) -> i32` as the return type, as all return paths are of type `[closure@src/lib.rs:2:5: 2:8]`, which implements `Fn(i32) -> i32`
      |
    1 | fn returns_closure() -> impl Fn(i32) -> i32 {
      |                         ~~~~~~~~~~~~~~~~~~~
    
    For more information about this error, try `rustc --explain E0746`.
    error: could not compile `functions-example` due to previous error
    ```
    
- As the error message says, Rust does not know the size of the closure at compile time.
- The solution as we saw before, is to use a pointer:
    
    ```rust
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    	Box::new(|x| x + 1)
    }
    ```
    
### Quiz

1. Consider implementing a `register` function that takes a callback in two ways:
    
    ```rust
    fn register1(cb: fn(Event) -> ());
    fn register2<F>(cb: F) where F: Fn(Event) -> ();
    ```
    
    Which type signature allows `register` to take the widest variety of arguments?
    
    - Ans

        `register2` (accepts both closure `Fn` and function pointers)

## Macros

### Introduction

- The term `macro` refers to a family of features in Rust: *declarative* macros with `macro_rules!` and three kinds of *procedural* macros:
    - Custom `#[derive]` macros that specify code added with the `derive` attribute used on structs and enums
    - Attribute-like macros that define custom attributes usable on any item
    - Function-like macros that look like function calls but operate on the tokens specified as their argument

### The difference between Macros and Functions

- Fundamentally, macros is a means of *metaprogramming* — writing code that writes other code
- All of the macros we have seen so far, *expand* to produce more code
- Macros reduce the amount of code that you have to write and maintain — which is also what functions do
- However, a function signature must declare the. number and type of parameters the function has
- Macros, on the other hand, can:
    - take a variable number of parameters:
        - `println!("hello");`, `prinltn("Hello, {}", name);`
    - implement a trait at runtime because it expands before the compiler interprets the meaning of the code (functions cannot do this because they are called at runtime but a trait must be implemented at compile-time)
- The downside of a macro is that macro definitions are more complex, and difficult to read, understand and maintain than function definitions because you’re writing Rust code that writes Rust code
- Another important difference is that you must first define macros or bring them into scope *before* you call them in a file, as opposed to functions that you can define and call anywhere

### Declarative Macros with `macro_rules!` for General Metaprogramming

- Declarative macros are the most widely used macros
- They are sometimes referred to as: `macros by example`, `macro_rules!` macros or just plain `macros`
- At their core, they enable us to write something similar to the `match` expression in that macros also compare a value to patterns that are associated with particular code
- In this situation:
    - the value is the literal Rust source code passed to the macro;
    - the patterns are compared with the structure of that source code;
    - and the code associated with each pattern, when matched, replaces the code passed to the macro;
    
    all of which happens at compile time.
    
- To define a macro, you use the `macro_rules!` construct
- To understand how it works, this is roughly how the `vec` macro is defined in the standard library:
    
    ```rust
    #[macro_export]
    macro_rules! vec {
    	( $( $x:expr ),* ) => {
    		{
    			let mut temp_vec = Vec::new();
    			$(
    					temp_vec.push($x);
    			)*
    			temp_vec
    		}
    	};
    }
    ```
    
    Here,
    
    - `#[macro_export]` annotation indicates that this macro should be made available whenever the crate in which the macro is defined is brought into scope
    - `macro_rules!` starts the macro definition which is followed by the `name` of our macro (without the exclamation sign), the `name` in this case is `vec`
    - the structure in the `vec` body is similar to that in a `match` expression. It contains a single arm with the pattern: `( $( $x:expr ), * )` followed by `=>` and the block of code associated with this pattern
- Valid pattern syntax in macro definitions is different from the pattern syntax covered before in `match` because these pattern refer to the Rust code structure itself rather than values
- For example, declarative macros can match against expressions (`expr`), types (`ty`) and even entire items (`item`). More on these in the [Rust Reference](https://doc.rust-lang.org/reference/macros-by-example.html).
- Now, let’s understand the pattern used in `vec`
- The Pattern
    - First, we use a set of parentheses to encompass the whole pattern.
    - We use a `$` to declare a variable in the macro system that will contain the Rust code matching the pattern. This makes it clear that it is a macro variable rather than a regular variable
    - The next set of parentheses captures the values that match the pattern to be used in the replacement code
    - The `$x:expr` declares a macro variable `x` of the type `expression` that matches any rust expression
    - The comma after the inner closing parenthesis indicates that a literal comma separator could optionally appear after the code that matches the code in `$()`.
    - The `*` specifies that the pattern matches zero or more of whatever precedes the `*`.
    - When we call this macro with `vec![1, 2, 3]`, the `$x` pattern matches three times with the three expressions `1`, `2`, and `3`.
- The Body
    - `temp_vec.push()` within `$()*` is generated for each part that matches `$()` in the pattern zero or more times depending upon on how many times the pattern matches
    - The `$x` is replaced with each expression matched
    - When we call this macro with `vec![1, 2, 3]`, the code that is generated) is:

        ```rust
        { 
        	let mut temp_vec = Vec::new();
        	
        	temp_vec.push(1);
        	temp_vec.push(2);
        	temp_vec.push(3);
        	
        	temp_vec
        }
        ```

- A great resource to learn about how to write macros is [The Little Book of Rust Macros](https://veykril.github.io/tlborm/) and

<https://www.youtube.com/watch?v=q6paRBbLgNw>

### Procedural Macros for Generating Code from Attributes

- A *procedural* macro acts more like a function (and is a type of procedure)
- These macros must accept some code as an input, operate on that code, and produce some code as an output rather than matching against patterns and replacing the code with other code as declarative macros do
- The three kinds of procedural macros are custom derive, attribute-like, and function-like, and all work in a similar fashion
- When creating procedural macros, the definitions must reside in their own crate with a special crate type (for reasons that the Rust team hopes to eliminate in the future)
- Here is an example of a procedural macro definition:
    
    ```rust
    use proc_macro;
    
    #[some_other_attribute]
    pub fn some_name(input: TokenStream) -> TokenStream {
    	...
    }
    ```
    
- Any function that defines a procedural macro takes a `TokenStream` as an input and produces a `TokenStream` as an output.
- The `TokenStream` type is defined by the `proc_macro` crate that is included with Rust and represents a sequence of tokens
- This is the core of the macro: the source of the code that the macro is operating on makes up the input `TokenStream`
- The function also has an attribute attached to it that specifies which kind of procedural macro we’re creating
- A crate can have multiple types of procedural macros

#### Writing a Custom `derive` Macro

- Let’s create a custom `derive` macro called `HelloMacro` under a new crate called `procedural` that has the following functionality:
    
    ```rust
    use procedural::HelloMacro;
    use procedural_derive::HelloMacro;
    
    #[derive(HelloMacro)]
    struct Pancakes;
    
    fn main() {
        Pancakes::hello_macro();
    }
    ```
    
- The default implementation of this macro ought to print out:
    
    ```rust
    Hello, Macro! My name is <TypeName>!
    ```
    
    where, `<TypeName>` is the name of the type for which the macro has been derived.
    
- A trivial way to accomplish this is to create a `trait` with function called `hello_macro` and implement it manually for the `Pancakes` struct:
    
    ```rust
    trait HelloMacro {
    	fn hello_macro() {
    		...
    	}
    }
    ```
    
- However, users will have to do this for every type that they need to derive this macro for
- We don’t want users to have to implement this manually. Additionally, we want the type information to be generated at runtime (which is not possible as is as Rust does not have runtime reflection capabilities)
- So, we need to use a macro — a procedural macro
- The first step is to create a procedural macro.
- As of this writing, procedural macros need to be in their own crate — the convention is that for a library crate `foo`, the procedural macro crate be named `foo_derive`.  So, we create a crate named `procedural_derive` inside our `procedural` project:
    
    ```rust
    cargo new procedural_derive --lib
    ```
    
- Our two crates are tightly coupled — if we change the trait definition for `HelloMacro`, we’ll need to change the implementation of the `procedural_derive` as well.
- This is why we create the `procedural_macro` crate inside the directory for our `procedural` crate
- However, these two crates will have to be published separately and users who want to use both must bring both into their scope separately
- This allows users to just use the `HelloMacro` trait without the `derive` functionality
- We need to declare the `procedural_derive` crate as a procedural macro crate, and we’ll also need functionalities from the `syn` and `quote` crates as we’ll soon see. So, let’s add these to our `Cargo.toml` file:
    
    ```toml
    [lib]
    proc-macro = true
    
    [dependencies]
    quote = "1.0.33"
    syn = "2.0.38"
    ```
    
- To start off, place the following code in the `src/lib.rs` for `procedural_derive`:
    
    ```rust
    use proc_macro::TokenStream;
    use quote::quote;
    
    #[proc_macro_derive(HelloMacro)]
    pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
        // Construct a representation of Rust code as a syntax tree
        // that we can manipulate
        let ast = syn::parse(input).unwrap();
    
        // Build the trait implementation
        impl_hello_macro(&ast)
    }
    ```
    
- We’ve split the implementation into two parts:
    - `hello_macro_derive` that parses the `TokenStream` into an abstract syntax tree, and
    - `impl_hello_macro` that transforms the syntax tree
- The code in the `hello_macro_derive` will be the same in almost every procedural macro definition
- The code in the `impl_hello_macro` will depend on the specifics of the macro being implemented
- We’ve also introduced three new crates:
    - `proc-macro`
        - comes with Rust, so we didn’t need to add that to the dependencies — this crate is the compiler’s API that allows us to read and manipulate Rust code from our code
    - `syn`
        - parses Rust code from a string into a data structure that we can perform operations on
    - `quote`
        - turns `syn` data structures back into Rust code
- These crates in conjunction make it much simpler to parse any sort of rust code we might want to handle: writing a full parser for Rust code is no simple task.
- The `hello_macro_derive` method will be called any time a user of our library specifies a `#[derive(HelloMacro)]` on a type — this is made possible because we have annotated our function with `proc_macro_derive` and specified the name `HelloMacro` which matches the name of the trait that we want to derive (this naming is by convention)
- Within this function, we feed the `TokenStream` (our Rust code) into `syn` which converts it into a `DeriveInput` struct representing the parsed Rust code
- For our `Pancakes` struct, this is (part of) what we get from `syn`:
    
    ```rust
    DeriveInput {
        // --snip--
    
        ident: Ident {
            ident: "Pancakes",
            span: #0 bytes(95..103)
        },
        data: Struct(
            DataStruct {
                struct_token: Struct,
                fields: Unit,
                semi_token: Some(
                    Semi
                )
            }
        )
    }
    ```
    
- The fields of this struct show that the Rust code we have parsed is a unit struct with the `ident` (i.e., identifier) of `Pancakes`
- Also note that our macro function returns a `TokenStream` instead of a `Result` because to confirm with procedural macro API, it must return `TokenStream`
- This is why we `panic` with `unwrap` if `syn` fails to parse the `TokenStream` instead
- A production code would use `panic` or `expect` to provide a more contextual message in case of failure, to the users
- Now, let’s implement the `impl_hello_macro` function that generates the output `TokenStream` from the syntax tree:
    
    ```rust
    fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
        let name = &ast.ident;
        let gen = quote! {
            impl HelloMacro for #name {
                fn hello_macro() {
                    println!("Hello, Macro! My name is {}", stringify!(#name));
                }
            }
        };
    
        gen.into()
    }
    ```
    
- Here, we get the name of the macro from the `ident` field in the `DeriveInput` as we saw earlier
- The `quote!` macro lets us define the Rust code that we want to return
- The compiler expects something different than the thing returned by this macro, so we must called `into` on the result to get the expected output
- `quote!` also provides some cool templating mechanisms as with the `#name` above that effectively replaces `#name` with the value of the `name` variable in the outer scope. More on the available techniques and templates on the `[crate docs](https://docs.rs/quote)`.
- Another new macro is the `stringify!` macro that is built into Rust. It takes any expression and at compile time converts into a string, such as `1 + 2` into `"1 + 2"`
- This is different from `format!` or `println!` macros which evaluate the expression and then turn the result into a `String`.
- An added benefit is that `stringify` also saves us an allocation by converting `name` into a string literal at compile time.
- Moreover, `ident` can also be an expression so we need to use `stringify` just to be sure.
- Finally, in our `procedural` we need to hook up the path to our `proc_macro` crate in the dependencies:
    
    ```toml
    [dependencies]
    procedural_derive = { path = "./procedural_derive" }
    ```
    
- This should allow us to run the following code:
    
    ```rust
    use procedural_derive::HelloMacro;
    
    pub trait HelloMacro {
        fn hello_macro() {
            println!("hello, Macro!");
        }
    }
    
    #[derive(HelloMacro)]
    struct Pancakes;
    
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn hello_macro_works() {
            Pancakes::hello_macro();
    				// prints "Hello Macro! My name is Pancakes"
        }
    }
    ```
    
#### Attribute-like Macros

- They are similar to custom derive macros but instead of generating code for the `derive` attribute, they allow you to create new attributes
- They are also more flexible in that `derive` only works for structs and enums but attributes can be applied to other items as well, such as functions
- For example,
    
    ```rust
    #[route(GET, "/")]
    fn index() {
    ```
    
    Here, `#[route}` attribute would be defined by the framework as a procedural framework whose signature would look like this:
    
    ```rust
    #[proc_macro_attribute]
    pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    ```
    
- Here, we have two parameters of type `TokenStream`:
    - `attr` ⇒ for the contents of the attribute: the `GET, "/"` part
    - `item` ⇒  for the body of the item the attribute is attached to: `fn index() {}`

#### Function-like Macros

- These macros look like function calls
- Similar to `macro_rules!`, they are more flexible than functions — they can take an unknown number of arguments.
- The difference is that while `macro_rules` can be define only by using match-like syntax, function-like macros take a `TokenStream` parameter and their definition manipulates the `TokenStream` using Rust code as the other two type of procedural macros do
- An example is the `sql!` macro:
    
    ```rust
    let sql = sql!(SELECT * FROM posts WHERE id=1);
    ```
    
- This macro would parse the SQL statement inside it and check that it’s syntactically correct, which is much more complex processing than a mere `macro_rules!` can proviide
- Its definition would look like this:
    
    ```rust
    #[proc_macro]
    pub fn sql(input: TokenStream) -> TokenStream {
    ```
    
### Quiz

1. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    macro_rules! manylet {
        ( $( $i:ident ),* = $e:expr ) => {
            $(
                let mut $i = $e;
            )*
        }
    }
    fn main() {
        let mut s = String::from("A");
        manylet!(x, y = s);
        x.push_str("B");
        println!("{x}{y}");
    }
    ```
    
    - Ans

        This program does not compile as it expands to:

        ```rust
        let mut x = s;
        let mut y = s;
        x.push_str("B");
        println!("{x}{y}");
        ```

        This results in multiple mutable references being active while one of them is mutated in between (the printing of `y` becomes invalid)

2. Which of the following are valid reasons for implementing a macro as a procedural macro instead of a declarative macro?
    - Ans
        - You want to integrate with Rust's derive system
        - Your macro requires nontrivial analysis of the macro user's syntax
3. Which of the following best describes the input to a procedural macro?
    - Ans

        The input is a sequence of tokens
