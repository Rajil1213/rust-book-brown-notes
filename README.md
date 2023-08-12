# Functional Language Features: Iterators and Closures

## Introduction

- This chapter covers the following functional language features:
    - *Closures* — a function-like construct you can store in a variable
    - *Iterators* — a way of processing a series of elements
    - How to use closures and iterators to improve the I/O project (from previous chapter)
    - The performance of closures and iterators

## Closures

- Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions
- You can create a closure in one place and call it in some other context
- **Unlike functions, closures can capture values from the scope in which they’re defined**

### Capturing the Environment with Closures

- We’ll look at the following scenario to demostrate Rust’s Closure functionality:
    - Every so often, our t-shirt company gives away an exclusive, limited-edition shirt to someone on our mailing list as a promotion
    - People in the mailing list can optionally add their favorite color to their profile
    - If the person chosen for a free t-shirt has their favorite color set, they get that color shirt
    - If this person hasn’t specified a favorite color, they get whatever color the company currently has the most of
- First, let’s create the relevant structures:
    - `ShirtColor` — an enum with `Red` and `Blue` variants; limiting the colors for simplicity
    - `Inventory` — representing the company’s inventory that contains:
        - a `Vec<ShirtColor>` representing the shirt colors currently in stock;
        - a `giveaway` method that gets the optinal shirt color preference of the free shirt winner, and returns the shirt color the person wil get
- Here is the library implementation:
    
    ```rust
    // src/lib.rs
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum ShirtColor {
        Red,
        Blue,
    }
    
    pub struct Inventory {
        shirts: Vec<ShirtColor>,
    }
    
    impl Inventory {
        pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
            user_preference.unwrap_or_else(|| self.most_stocked())
        }
    
        pub fn most_stocked(&self) -> ShirtColor {
            let mut num_red = 0;
            let mut num_blue = 0;
    
            for color in &self.shirts {
                match color {
                    ShirtColor::Red => num_red += 1,
                    ShirtColor::Blue => num_blue += 1,
                }
            }
    
            if num_red > num_blue {
                return ShirtColor::Red;
            }
    
            ShirtColor::Blue
        }
    }
    ```
    
- And here is how we would use it:
    
    ```rust
    // src/main.rs
    use demo::{Inventory, ShirtColor};
    
    fn main() {
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };
    
        let user_pref1 = Some(ShirtColor::Red);
        let giveaway1 = store.giveaway(user_pref1);
        println!("The user with preference {user_pref1:?} gets {giveaway1:?}");
    
        let giveaway2 = store.giveaway(None);
        println!("The user with no preference gets {giveaway2:?}");
    }
    ```
    
- In the above library implementation, specifically in the `giveaway` method, we are using a `closure` in the argument to `unwrap_or_else`
- This method is defined in the standard library and takes one argument — a closure without any arguments that returns a value `T` (the same type stored i the `Some` variant of the `Option<T>` on which the `unwrap_or_else` method is called, in this case `ShirtColor`)
- If the `Option<T>` is the `Some` variant, `unwrap_or_else` unwraps the value stored within it and returns it
- If the `Option<T>` is the `None` variant, the closure passed as argument to the `unwrap_or_else` method is called and the value returned by it is returned
- Now let’s look at the closure itself:
    
    ```rust
    || self.most_stocked()
    ```
    
    - This closure takes no arguments (if it did, the argument would appear within the `| |` vertical bars)
    - The body of the closure calls the `most_stocked` method on `self`
    - The closure is beind defined here and but it is called later if the `Option<T>` returns a `None` value
- The above code produces the following output:
    
    ```rust
    The user with preference Some(Red) gets Red
    The user with no preference gets Blue
    ```
    
- The closure is called in the second case, where `giveaway` method is passed the `None` variant for `user_preference`. In this case, the closure is invoked which in turn returns the return value of the `most_stocked` method

<aside>
💡 An interesting aspect here is that we’ve passed a closure that calls the `self.most_stocked` on the current inventory instance. The closure captures an immutable reference to the `self Inventory` instance and passes it with the code we specify  to the `unwrap_or_else` method. Functions are not able to capture their environment in this way

</aside>

### Closure Type Inference and Annotation

- Closures don’t usually require type annotation (for both parameters and return types)
- Type annotation are required in functions because they are part of an explicit interface exposed to users
- Closures aren’t used in an exposed interface — they’re stored in variables and used without naming them and exposing them to users of our library
- Closures are typically short and relevant only within a narrow context rather than in any arbitrary scenario — within this narrow context, the compiler can infer the types of most variables (in rare cases, this might not be sufficient)
- We can, of course, add type annotation to closures in the spirit of explicitness and clarity at the expense of verbosity that isn’t strictly necessary
- For example:
    
    ```rust
     let expensive_closure = |num: u32| -> u32 {
    		println!("calculating slowly...");
    		thread::sleep(Duration::from_secs(2));
    		num
    }
    ```
    
    With these type annotaitons, the closure syntax becomes similar to function syntax:
    
    ```rust
    // going from function syntax to closure syntax:
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;
    ```
    
- The compiler infers on concrete type for each of their parameters and for their return value. For example, let’s take a simple closure that returns its parameter:
    
    ```rust
    let example_closure = |x| x;
    
    let s = example_closure(String::from("hello"));
    let n = example_closure(1);
    ```
    
    The above program fails because the compiler infers that the type of `x` is `String` based on its first invocation but we are now trying to call the same closure on an `i32` which is not allowed!
    
### Quiz

1. Which of the following best describes the rationale for why Rust will infer the types of arguments/returns for closures, but not top-level functions?
    - Ans

        Top-level functions can be part of a library's external interface, while closures cannot be directly exposed

        **Context**: Rust could, in theory, provide type inference for top-level functions. Functional languages like Haskell and OCaml have this feature. However, Rust designers made the conscious decision to require type annotations on top-level functions to promote clarity at the interface level: a function will always have exactly the type it says it has.

2. Rust permits pattern matching within closure arguments, including the use of the underscore. For example, you could write the following:
    
    ```rust
    let f = |_| (); // sometimes called the "toilet closure"
    let s = String::from("Hello");
    f(s);
    ```
    
    Which of the following best describes the relationship between `f` and `s` in this program?
    
    - Ans

        `f` causes `s` to be immediately dropped

        **Context**: The toilet closure is similar to `[std::mem::drop](https://doc.rust-lang.org/std/mem/fn.drop.html)`, i.e. a function that moves an argument and causes it to be dropped.

### Capturing References or Moving Ownership

- A closure can capture values from their environment in three way (which directly map to thre three ways a function can take a parameter):
    - borrowing immutably
    - borrowing mutably
    - owning
- The closure will decide which of these to use based on what the body of the closure does with the captured values
- Take the following code:
    
    ```rust
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    
    let only_borrows = || println!("From closure: {list:?}");
    
    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
    ```
    
    When we run the above, we get:
    
    ```rust
    Before defining closure: [1, 2, 3]
    Before calling closure: [1, 2, 3]
    from closure: [1, 2, 3]
    After calling closure: [1, 2, 3]
    ```
    
    This code demonstrates that:
    
    - A closure can be bound to a variable which can then be called like a function
    - Because we have multiple immutable references to the `list`, we can call it:
        - before the closure is defined
        - after the closure is defined but before it is called
        - inside the closure
        - after the closure is called
- Now, if we change the closure definition like so:
    
    ```rust
    let mut borrows_mutably = || list.push(4);
    println!("Before calling mutable closure: {list:?}");
    borrows_mutably();
    println!("After calling mutable closure: {list:?}");
    ```
    
    This fails to compile and shows the following error message:
    
    ```rust
    error[E0502]: cannot borrow `list` as immutable because it is also borrowed as mutable
      --> src/ownership.rs:12:47
       |
    11 |     let borrows_mutably = || list.push(4);
       |                           -- ---- first borrow occurs due to use of `list` in closure
       |                           |
       |                           mutable borrow occurs here
    12 |     println!("Before calling mutable closure: {list:?}");
       |                                               ^^^^^^^^ immutable borrow occurs here
    13 |     borrows_mutably();
       |     --------------- mutable borrow later used here
       |
       = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
    ```
    
    If we are to remove the line where we print out the value of the list before calling the closure, the code compiles and outputs (because the scope of the mutable reference ends after we invoke the closure):
    
    ```rust
    After calling mutable closure: [1, 2, 3, 4]
    ```
    
- If we require that our closure moves the capture value even though our function body does not need it, we can use the `move` keyword in the closure definition like so:
    
    ```rust
    use std::thread;
    
    fn main() {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);
    
        thread::spawn(move || println!("From thread: {:?}", list))
            .join()
            .unwrap();
    }
    ```
    
    Here, we spawn a new thread that takes a closure as an argument. We use the `move` keyword in this closure so that the closure now takes ownership of the closed values, in this case `list`
    
    Here, even though all `println!` needs is an immutable reference to `list` (and Rust infers this), we use `move` to tell Rust to own the `list` instead, moving the `list` into the closure. The rationale behing this is that:
    
    - The new thread might finish before the rest of the main thread finishes, or the main thread might finish first.
    - If the main thread maintained ownership of `list` but ended before the new thread did and dropped `list`, the immutable reference in the thread would be invalid
    - Therefore, the compiler requires that the `list` be moved into the closure given to the new thread so that the reference will be valid
    - Removing the `move` keyword would throw a compilation error:

        ```rust
        error[E0373]: closure may outlive the current function, but it borrows `list`, which is owned by the current function
          --> src/ownership.rs:19:19
           |
        19 |     thread::spawn(|| println!("From thread: {:?}", list))
           |                   ^^                               ---- `list` is borrowed here
           |                   |
           |                   may outlive borrowed value `list`
           |
        note: function requires argument type to outlive `'static`
          --> src/ownership.rs:19:5
           |
        19 |     thread::spawn(|| println!("From thread: {:?}", list))
           |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        help: to force the closure to take ownership of `list` (and any other referenced variables), use the `move` keyword
           |
        19 |     thread::spawn(move || println!("From thread: {:?}", list))
           |                   ++++
        ```

### Moving Captured Values Out of Closures and the `Fn` Traits
    
- Once a closure has captured a reference or captured ownership of a value from the environment where the closure is defined, the body of the closure defines what happens to the references or values when the closure is evaluated later
- The closure body can do any of the following:
    - move a captured value out of the closure
    - mutate the captured value
    - neither move nor mutate the value
    - capture nothing from the environment to begin with
- The behavior of the closure body dictates what trait(s) the closure implements
- These traits, in turn, specify what kinds of closures funcitons and structs can accept/use
- Closures will automatically implement one or more of the following `Fn` traits in additive fashion:

    | Behavior/Trait | FnOnce | FnMut | Fn |
    | --- | --- | --- | --- |
    | ## Calls | 1 | ≥ 1 | ≥ 1 |
    | Behavior | Move captured values
    All closures implement this trait | Do not move but can mutate captured values | Either do not capture anything or do not mutate the captured values |
- With this information, let’s look at the implementation of the `unwrap_or_else` method:

    ```rust
    impl<T> Option<T> {
        pub fn unwrap_or_else<F>(self, f: F) -> T
        where
            F: FnOnce() -> T
        {
            match self {
                Some(x) => x,
                None => f(),
            }
        }
    }
    ```

    Here, `T` is a generic representing the value in the `Some` variant of an `Option` which is also the return type of the method itself

    There is also an additional generic parameter `F` which is the type of the parameter named `f` — the closure we provide when calling the `unwrap_or_else` method

    The trait bound specified on the generic type `F` is `FnOnce() -> T`, which means `F` must be:

    - able to be called once,
    - take no arguments, and
    - return a `T`

    This trait bound expresses the constraint that the closure is called at most once in the method body

    Since all closures implement the `FnOnce` trait, this method is accepting of most types of closures

- Next, let’s look at another standard library method called `sort_by_key` defined no slices:
    
    ```rust
    pub fn sort_by_key<K, F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        // <snip>
    }
    ```
    
    Here, the clsoure `f` is of generic type `F` that has a trait bound of `FnMut` that accepts an immutable reference to the type `T` stored in the slice and returns another genric `K` that can be ordered
    
    This function is useful when you sort a slice by an attribute of each element of a slice
    
    The reason this closure is bound to `FnMut` is that the body of this method calls the closure multiple times.
    
    With this trait bounds, if we have struct defined as:
    
    ```rust
    struct Rectangle {
    	width: u32,
    	height: u32,
    }
    ```
    
    And a slice of these structs, then we can pass the following closure to the `sort_by_key` method:
    
    ```rust
    | r | r.width
    ```
    
    This closure does not capture, mutate or move out anything from its environment so it meets the trait bounds for `sort_by_key`
    
    On the other hand, we cannot pass the following closure:
    
    ```rust
    | r | {
    	some_captured_vec.push(some_captured_string_value);
    	r.width
    }
    ```
    
    This closure captures a vec and a string from its environment and mutates the vec. As such, it cannot be called multiple times because the string value will no longer be available for pushing a second time
    
    Alternatively, the following closure can be used:
    
    ```rust
    | r | {
    	captured_count += 1
    	r.width
    }
    	
    ```
    
    Here, the captured value `captured_count` is mutated but is safe to call it multiple times.
    
<aside>
💡 `FnOnce` is the least restrictive of trait bounds because we can pass almost any closure to it. However, this means that when we write method that accepts or returns a closure that implements this trait, there are major restrictions to what we can do in that function given that it can only be called once
`Fn` is the most restrictive of trait bounds because we can pass only those closures that either do not capture their environment or those that do not mutate it. Inversely, when we write a function that accepts or returns a closure that implements this trait bound, we can safely use it do anything with it i.e., there are the least restrictions

</aside>

### Closures Must Name Captured Lifetimes

- When you start defining functions that accept or return closures, you will need to think about the lifetimes of data captured by the closure
- For example:
    
    ```rust
    fn make_a_cloner(s_ref: &str) -> impl Fn() -> String {
    	move || s_ref.to_string()
    }
    ```
    
    The above function returns a closure that clones the input string
    
    However, this function is rejected by the compiler
    
    ```rust
    error[E0700]: hidden type for `impl Fn() -> String` captures lifetime that does not appear in bounds
     --> test.rs:2:5
      |
    1 | fn make_a_cloner(s_ref: &str) -> impl Fn() -> String {
      |                         ---- hidden type `[closure@test.rs:2:5: 2:12]` captures the anonymous lifetime defined here
    2 |     move || s_ref.to_string()
      |     ^^^^^^^^^^^^^^^^^^^^^^^^^
    ```
    
- To understand the above error message, let’s see what would happen if Rust allowed the above code to compile
- Here is an unsafe program that we could write
    
    ```rust
    fn make_a_cloner(s_ref: &str) -> impl Fn() -> String {
    	move || {
    		s_ref.to_string() // L3
    	}
    }
    
    fn main() {
    	let s_own = String::from("Hello world");
    	let cloner = make_a_cloner(&s_own); // L1
    	drop(s_own); // L2
    	cloner();
    }
    
    ```
    
    In the above code:
    
    - `make_a_cloner` is called at `L1` where we get back a closure `cloner`
    - within the closure, is its environment that has captured `s_ref`
    - but at `L2`, we are allowed to drop the `s_own` String that the closure references
    - this invalidates `cloner` because its environment contains a pointer to deallocated memory
    - calling `cloner()` would then cause a use-after-free undefined behavior
- In the above error message, Rust is telling us that **we need to tell Rust that the closure returned from `make_a_cloner` must not live longer than `s_ref`**:
    
    ```rust
    fn make_a_cloner<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {
    	...
    }
    ```
    
    Here, we add the generic lifetime parameter to the function signatue and the input parameter
    
    Then, we also add a `+ 'a` to the trait bounds for the closure’s return, which indicates that the closure must live no longer than `'a`
    
    Rust now deduces the function as safe and our program fails to compile when we try to `drop` `s_own` before calling the `cloner` closure:
    
    ```rust
    error[E0505]: cannot move out of `s_own` because it is borrowed
      --> test.rs:9:6
       |
    8  | let cloner = make_a_cloner(&s_own);
       |                            ------ borrow of `s_own` occurs here
    9  | drop(s_own);
       |      ^^^^^ move out of `s_own` occurs here
    10 | cloner();
       | ------ borrow later used here
    ```
    
- We can also use Rust’s lifetime elision rules to simply our function signature to:
    
    ```rust
    fn make_a_cloner(s_ref: &str) -> impl Fn() -> String + '_ {
    	...
    }
    ```
    
    Here, we are keeping an indicator `'_` to say that the returned closure depends on *some* lifetime
    
### Quiz

1. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    fn main() {
        let mut s = String::from("Hello");
        let mut add_suffix = || s.push_str(" world");
        println!("{s}");
        add_suffix();  
    }
    ```
    
    - Ans

        This program **does not** compile.

        Here, the closure captures a mutable reference to `s` and within the scope of the closure (between when it’s defined and called), we are reading from the reference in `println!`. This is not allowed

2. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    fn main() {
        let mut s = String::from("Hello");
        let add_suffix = |s: &mut String| s.push_str(" world");
        println!("{s}");
        add_suffix(&mut s);  
    }
    ```
    
    - Ans

        The program **DOES** compile, with output `Hello`

        Here, `add_suffix` does not capture the environment, instead it accepts an input argument

3. Consider the following API:
    
    ```rust
    /// Executes a function with a mutable reference to each element of a vector
    fn for_each_mut<T, F: ___(&mut T)>(v: &mut Vec<T>, mut f: F) {
        for x in v.iter_mut() {
            f(x);
        }
    }
    ```
    
    - Ans

        `FnMut`

        The function closure `f` is called multiple times within the function body, so `FnOnce` is not appropriate. Both `FnMut` and `Fn` are acceptable but we use `FnMut` as it is less restrictive

4. Consider the following API:
    
    ```rust
    pub struct Analyzer<F> {
        postprocess: F
    }
    impl<F: ___(i32) -> i32> Analyzer<F> {
        fn process(&self, n: i32) -> i32 { /* .. */ }
        pub fn pipeline(&self, n: i32) -> i32 {
            let n = self.process(n);
            (self.postprocess)(n)
        }
    }
    ```
    
    Which of the following function traits is most appropriate to fill in the blank?
    
    - Ans

        `Fn`

        Here, the closure `postprocess` does not capture its environment rather accepts an input. So this closure can be called any number of times without restriction. This makes `FnOnce` inappropriate. `FnMut` is also not appropriate because `self` is an immutable refererence in the `pipeline` method which needs to be mutable for `FnMut`. So, `Fn` is the most appropriate

## Iterators

- Iterator pattern allows us to perform some task on a sequence of items in turn
- An iterator is responsible for the logic of iterating over each item and determining whether the sequence has finished — logic that we will not have to implement ourselves
- Iterators in Rust are `lazy` — they have no effect until you call methods that consume the iterator to use it up
- For example, consider the following code snippet:
    
    ```rust
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // create and store iterator
    
    for val in v1_iter { // loop through the values in the iterator
        println!("Got: {val}");
    }
    ```
    
    Here, we are obtaining the iterator out of Vec. This iterator can then be used in a `for..loop`. We don’t have to specify when or how the loop ends because that logic is handled by the iterator itself.
    
### The `Iterator` Trait and the `next` Method

- All iterators implement a trait called `Iterator` that is defined in the standard library:
    
    ```rust
    pub trait Iterator {
        type Item;
    
        fn next(&mut self) -> Option<Self::Item>;
    
        // methods with default implementations elided
    }
    ```
    
    Here, `type Item` defines the `associated type` with this trait (covered later)
    
    All this means is that implementing the `Iterator` trait requires that you also define an `Item` type, and this `Item` type is used in the return value of the `next` method.
    
    The above trait definition also means that we only need implement the `next` method in order to create an iterator, which returns on item of the iterator at a time wrapped in `Some` and, when iterator is over, returns `None`
    
- We can also call the `next` method directly:
    
    ```rust
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();
    
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
    ```
    
    Here, we need to define `v1_iter` as `mut` because the `next()` method changs the internal state of the iterator that it uses to keep track of where it is in the sequence. In other words, this code *consume* or *uses up* the iterator — each call to `next` eats up an item from the iterator
    
    <aside>
    💡 We did not need to make `v1_iter` mutable to use it in the `for` loop because the loop took ownership of `v1_iter` and made it mutable behind the scenes
    
    </aside>
    
    Also note that the values we get from the `next` method are immutable references to the values in the vector. If we want to create an iterator that takes ownership of `v1` and returns owned values, we can call `into_iter` instead. If we want to iterate over mutable references, we can call `iter_mut`
    
### Methods that Consume the Iterator

- The iterator trait has a number of different methods with default implementations provided by the standard library
- Some of these methods rely on the `next` method defined on the iterator which is why it needs to be implemented
- Methods that call the `next` method are called the `consuming adaptors`
- One example is the `sum` method that adds each item returned by the `next` method to a running total and returns the total when the iteration is complete
- For example:
    
    ```rust
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
    
        let sum: i32 = v1_iter.sum();
        assert_eq!(sum, 6);
    }
    ```
    
    Here, we are not allowed to use `v1_iter` after the call to `sum` because it takes ownership of the iterator.
    
### Methods that Produce Other Iterators

- `Iterator Adaptors` are methods defined on the iterator that do not consume the iterator
- Instead, they produce different iterators by changing some aspect of the original iterator
- For example:
    
    ```rust
    let v2_iter = v1.iter().map(|x| x + 1);
    for val in v2_iter {
        println!("In new iterator, got: {val}");
    }
    ```
    
    The above prints:
    
    ```rust
    In new iterator, got: 2
    In new iterator, got: 3
    In new iterator, got: 4
    ```
    
    We can also write a test:
    
    ```rust
    #[test]
    fn adatper_iterator() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter().map(|x| x + 1);
        let v2_values: Vec<i32> = v1_iter.collect();
        assert_eq!(v2_values, vec![2, 3, 4]);
    }
    ```
    
- Here, the `map` method takes a closure and returns an iterator an iterator — in this case, one that contains the elements of the original iterator incremented by 1
- Because it takes a closure, we can define any operation we want to perform on each item while reusing the iteration behavior
- We can also chain multiple iterators in a readable way but since they are lazy, we have to call one of the consuming adaptor methods to get results from calls to iterator adaptors (for example, `collect()` to consume the result of `map`)

### Using Closures That Capture Their Environments

- Closures passed into iterator adatpors usually capture their environment
- For example, there is a filter method that takes a closure — that takes an item from the iterator and returns a `bool` which if true for a given item will cause the item to be included in the iterator produced by the filter
- Example:
    
    ```rust
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }
    
    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes
            .into_iter()
            .filter(|shoe| shoe.size == shoe_size)
            .collect()
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn filters_by_size() {
            let shoes = vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 12,
                    style: String::from("sandal"),
                },
                Shoe {
                    size: 12,
                    style: String::from("boot"),
                },
            ];
    
            let in_my_size = shoes_in_size(shoes, 12);
    
            assert_eq!(
                in_my_size,
                vec![
                    Shoe {
                        size: 12,
                        style: String::from("sandal"),
                    },
                    Shoe {
                        size: 12,
                        style: String::from("boot")
                    }
                ]
            )
        }
    }
    ```
    
- Here, the `shoes_in_size` function takes ownership of a vector of shoes and a shoe size as parameter and returns shoes of the specified size
- Inside the closure, the `shoe_size` value is captured and when we use the `collect` adaptor the closure is called repeatedly on the iterator to produce the final vec that is returned

### Quiz

1. Which of the following best describes why iterators are described as "lazy"?
    - Ans

        An iterator has no effect until you call methods that extract elements from the iterator

2. True/false: these two code snippets are semantically equivalent.
    
    ```rust
    // Snippet 1:
    
    while let Some(x) = iter.next() {
        f(x);
    }
    
    // Snippet 2:
    
    for x in iter {
        f(x);
    }
    ```
    
    - Ans

        True

        **Context**: The for-loop is a syntactic sugar for the `while let`, which is itself a sugar for `loop` and `break`.

3. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    fn main() {
        let v = vec![1, 2, 3, 4];
        let a: Vec<_> = v.iter().filter(|x: &&i32| *x % 2 == 0).map(|x: &i32| x * 2).collect();
        let b: Vec<_> = v.iter().map(|x: &i32| x * 2).filter(|x: &i32| x % 2 == 0).collect();
        println!("{} {}", a[0], b[0]);
    }
    ```
    
    - Ans

        The program DOES compile with the output: `4 2`

        **Context**: The order of iterators matters --- a filter and a map is not the same as a map and a filter!

        You might wonder why the first filter uses `*x` and the second filter does not. `v.iter()` produces an `Iterator<Item = &i32>`. The `.filter()` call takes an `Iterator<Item = T>` as input, and passes `&T` to its predicate. Therefore `x: &&i32` on line 3. The Rust standard library implements the remainder operator `%` for `&i32` on the left-hand side ([see the docs](https://doc.rust-lang.org/std/ops/trait.Rem.html#impl-Rem%3Ci32%3E-for-%26'a+i32)), but not for `&&i32`. So we have to dereference `x` once to use it in the expression `*x % 2`.

        By contrast on line 4, when `.map()` takes an `Iterator<Item = T>` as input, it passes `T` to its closure. Therefore the closure in `map` takes `&i32` as input. The multiplication operator `*` is implemented for `&i32`, so `x` does not need to be dereferenced in `x * 2`. The operation `x * 2` produces a value of type `i32`, so the result of the map is an `Iterator<Item = i32>`. The `filter` then takes `x : &i32`, which also does not need a dereference to do `x % 2`. Now you know!

## Improving the I/O Project

### Using Iterator to Parse Args

- We can use our new learnt iterators concept to improve the `build` function in our I/O Project
- Instead of inefficiently cloning the args, we can simply take ownership of the args and return the Config object
- The `std::env::args()` function conveniently returns an iterator for this purpose
- We can change our function signature for `build` to accept a parameter than implements and Iterator for the type `String`.
- With this approach our function is a bit more general
- Then, in the function body, we can call the `next` method to get the required values from the command-line:
    
    ```rust
    impl Config {
        pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
            // first => program_name, second, third => arguments
            const IGNORE_CASE_ENV_KEY: &str = "IGNORE_CASE";
            let ignore_case = env::var(IGNORE_CASE_ENV_KEY).is_ok();
    
            // ignore the program name
            args.next();
    
            // get the search string
            let searchstring = match args.next() {
                Some(searchstring) => searchstring,
                None => return Err("didn't get a search string"),
            };
    
            let filepath = match args.next() {
                Some(filepath) => filepath,
                None => return Err("didn't get a filepath"),
            };
    
            Ok(Self {
                searchstring,
                filepath,
                ignore_case,
            })
        }
    ```
    
    ```rust
    // src/main.rs
    fn main() {
    		let config = minigrep::Config::build(env::args()).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        });
    ```
    
### Making Code Clearer with Iterator Adaptors

- Instead of using a `for...loop`, we can make our implementation more precise by simply using an iterator

```rust
// before
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

```rust
// after
fn search<'a>(searchstring: &'a str, contents: &'a str) -> Vec<&'a str> { 
		contents
      .lines()
      .filter(|line| line.contains(searchstring))
      .collect()
}
```

## Performance: Loops vs Iterators

- To determine when to use a loop vs an iterator, you must first k onw which implementation is faster
- The authors of the book ran a benchmark for the search function by loading the entire *Adventures of Sherlock Holmes* into a `String` and then searching for the word `the` and obtained the following results:
    
    ```rust
    test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
    test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
    ```
    
- From this experiment, we can see that both versions of the implementation perform similarly with a slight edge for the one using the iterator instead of the loop
- While the benchmarking data is far from conclusive, it provides a general sense regarding their performances.
- One compiled, iterators basically “become” loops
- So, iterators are part of the Rust’s *zero-cost abstractions —* the abstraction imposes no runtime overhead
- This aligns with Bjarne Stroustrup’s definition of zero-overhead:
    
    > What you don’t use, you don’t pay for. What you do use, you couldn’t hand code any better
    >
- To look at a more concrete real-world example, here is a snippet from a [high-performance] audio decoder program:
    
    ```rust
    let buffer: &mut [i32];
    let coefficients: [i64; 12];
    let qlp_shift: i16;
    
    for i in 12..buffer.len() {
        let prediction = coefficients.iter()
                                     .zip(&buffer[i - 12..i])
                                     .map(|(&c, &s)| c * s as i64)
                                     .sum::<i64>() >> qlp_shift;
        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
    }
    ```
    
    Te above piece of code uses iterators to:
    
    - pair values in the `coefficients` array to another array holding the previous 12 values in the same array using `zip`
    - then, multiply these two values together,
    - then, sum the values in the resulting array, and
    - shift the sum by `qlp_shift` bytes to get the final `prediction`
- When the above code is compiled, the resulting assembly code just contains a single loop — there is no loop corresponding to the iteration over the values in the `coefficients`. Rust knows that there will be 12 iterations corresponding to the iteration over the values in `coefficients` so it “unrolls” the loop
    - *Unrolling* is an optimization that removes the overhead of the loop controllling code and instead generates repetitive code for each iteration of the loop
    - All the coefficients are stored in registers which makes the value lookups extremely fast
    - There are no bounds checks on the array access at runtime
- All this is to say that the Rust compiler knows enough about our code to make optimizations that ultimately provides us with higher-level abstractions that do not have a performance penalty attached to them!
