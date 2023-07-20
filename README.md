# Generic Types, Traits & Lifetimes
## Introduction

- Generics is one of the tools in Rust to handle duplication of concepts
- They are abstract stand-ins for concrete types or other properties
- They provide us a means of expressing behavior of abstract types and their interactions with each other without actually knowing what these extract types are
- This chapter covers:
    - How to extract a function to reduce code duplication
    - How to use generic types in struct and enum definitions
    - How to use traits to define behavior in a generic way
    - How to constrain what types are supported by your generic function based on their traits
    - A variety of generics called *lifetimes* that give the compiler information about how references relate to each other and how long their references should last

## Motivation

- Let’s start with the following program:
    
    ```rust
    fn main() {
        let num_list = vec![17, 31, 23, 13];
    
        let mut largest = &num_list[0];
    
        for number in &num_list {
            if number > largest {
                largest = number;
            }
        }
    
        println!("The largest number is: {largest}");
    }
    ```
    
    This program gets the largest number from a list of numbers
    
- If we’re asked to do this for two different lists, we’d factor out the logic to a separate function that accepts a list of numbers:
    
    ```rust
    fn main() {
        let num_list = vec![17, 31, 23, 13];
    
        let largest = get_largest(&num_list);
        println!("The largest number in {num_list:?} is {largest}");
    }
    
    fn get_largest(num_list: &Vec<i32>) -> &i32 {
        if num_list.len() == 0 {
            panic!("list of numbers cannot be empty");
        }
    
        let mut largest = &num_list[0];
    
        for number in num_list {
            if number > largest {
                largest = number;
            }
        }
    
        largest
    }
    ```
    
- In the same way that we can increase the reusability of a piece of code while reducing code duplication, we can use generics to reduce code duplication by allowing multiple concrete types
- For example, we can create a generic function that can get the largest value in a list of `i32` as well a list of `char` values.

## Generic Data Types

### In Functions

- The way to handle `char` types with the above function logic is to just add another function that accepts a `Vec<char>`
- This leads to the duplication of the code logic and the only thing different would be the type of the input parameters.
- To parameterize the types in a new single function, we need to the name the type parameter. This parameter is usually named to be `T` by convention.
- The convention of custom types in general is `PascalCase`
- When we use a parameter in the body of the function, we have to declare the parameter name in the function signature itself so that the compiler knows what that name means
- We also need to use a type parameter in the function name before we use it
- To convert the `get_largest` function into a generic function, the signature would be:
    
    ```rust
    fn get_largest<T>(list: &[T]) -> &T {
    	...
    }
    ```
    
    We read this definition as:
    
    > the function `get_largest` is generic over some type `T` and has one more parameter named `list` which is a slice of values of type `T`. This function returns a reference to value of the same type `T`.
    > 
- Then, the function definition would change accordingly to:
    
    ```rust
    fn get_largest<T>(num_list: &Vec<T>) -> &T {
        if num_list.len() == 0 {
            panic!("list of numbers cannot be empty");
        }
    
        let mut largest = &num_list[0];
    
        for number in num_list {
            if number > largest {
                largest = number;
            }
        }
    
        largest
    }
    ```
    
    This, however, does not compile and throws the following error message:
    
    ```rust
    error[E0369]: binary operation `>` cannot be applied to type `&T`
      --> src/main.rs:16:19
       |
    16 |         if number > largest {
       |            ------ ^ ------- &T
       |            |
       |            &T
       |
    help: consider restricting type parameter `T`
       |
    8  | fn get_largest<T: std::cmp::PartialOrd>(num_list: &Vec<T>) -> &T {
       |                 ++++++++++++++++++++++
    ```
    
- This is because the parameter type `T` is unrestricted meaning that it can represent *any* type. And not all types are comparable with the `>` operator.
- Some other programming languages like `C++` that has `templates`, the compiler does not complain when defining the function but instead complains when trying to pass a value to the function that is not comparable.
- Rust instead expects you to state the capabilities of your generic types up front
- Another feature of Rust is that, unlike a language like `Java` where all objects have a set of methods like `Object.toString()`, it doesn’t have any core methods. This means that without restrictions, a generic type `T` has not capabilities: it cannot be printed, cloned, or mutated (although it can be dropped)

### In Structs

- We can use the `<>` syntax in structs as well to define the type parameters for one or more members.
- For example:
    
    ```rust
    struct Point<T> {
    	x: T,
    	y: T,
    }
    
    fn main() {
    	let p1 = Point { x: 1, y: 1};
    	let p2 = Point { x: 2.0, y: 2.0 };
    }
    ```
    
- Due to our struct definition, the two members `x` and `y` must have the same type. For example, `let p3 = Point { x: 1, y: 2.0 }` will not work as the types for `x` and `y` namely, `i32` and `f64` are not the same

### In Enums

- Similar to structs, we can define generic structs as well
- In fact, the `Option<T>` and `Result<T, E>` are both generic enums
- For example:
    
    ```rust
    enum Optional<T> {
      Exists(T),
      None
    }
    
    fn main() {
    	let exists = Optional::Exists(2);
    	let does_not_exist: Optional<i32> = Optional::None;
    }
    ```
    
    Note that we need to provide the concrete type when declaring the `does_not_exist` variable with the `None` variant because the concrete type cannot be inferred from usage in this case.
    

### In Method Definitions

- When creating a generic method, we need to add `<>` to both the type as well as the `impl` keyword itself.
- Example:
    
    ```rust
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    
        fn y(&self) -> &T {
            &self.y
        }
    }
    ```
    
    The above creates generic methods on the `Point<T>` type
    
- We can also define methods that apply to a specific type:
    
    ```rust
    impl Point<f64> {
    	fn distance_from_origin(&self) -> f64 {
    		(self.x.powi(2) + self.y.powi(2)).sqrt()
    	}
    }
    ```
    
    This method `distance_from_origin` only applies to the `Point` struct whose concrete type is `f64`
    
- One caveat to this is that Rust does not allow us to define methods of the same name that have both a generic and a concrete variant
- Also note that generic type parameters in the struct definition need not be the same ones defined in the method signature. For example, we can redefine our `Point` struct so that the members can have different types and implement method that accepts two different points on it:
    
    ```rust
    pub struct Point<X1, Y1> {
        x: X1,
        y: Y1,
    }
    
    impl<X1, Y1> Point<X1, Y1> {
        pub fn x(&self) -> &X1 {
            &self.x
        }
    
        pub fn y(&self) -> &Y1 {
            &self.y
        }
    		
    		// here, we use <X2, Y2> instead of <X1, Y1> since those are the ones that need defining
    		// <X1, Y1> have already been defined in the `Point` impl
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            Point {
                x: *self.x(),
                y: *other.y(),
            }
        }
    }
    ```
    

### Performance of Code Using Generics

- The Rust compiler, at compile time, converts all the generic types into concrete types — this process is called monomorphization
- It is the opposite of what we are doing with generics.
- It effectively duplicates our code to fit the various concrete types actually passed into a generic function, struct, enum or method
- For example, take the following code:
    
    ```rust
    let integer = Some(1);
    let float = Some(2.0);
    ```
    
    The compile transforms the above code to something like this:
    
    ```rust
    enum Option_i32 {
        Some(i32),
        None,
    }
    
    enum Option_f64 {
        Some(f64),
        None,
    }
    
    fn main() {
        let integer = Option_i32::Some(1);
        let float = Option_f64::Some(2.0);
    }
    ```
    
- So, there is no real run-time cost to using generics — it would be the same as if we had written duplicate definitions instead of using generics.

### Quiz

1. Imagine using a third-party function whose implementation you don't know, but whose type signature is this:
    
    ```rust
    fn mystery<T>(x: T) -> T {
      // ????
    }
    ```
    
    Then you call `mystery` like this:
    
    ```rust
    let y = mystery(3);
    ```
    
    Assuming `mystery` uses no unsafe code, then the value of `y` must be :
    
    - Ans
        
        3
        
        The only possible return value (if the function does not panic) is the input value itself because since there are no constraints on the generic type parameter, we (and the compiler) cannot assume that it possesses any functionality. The only thing that can be done is to return the input itself.
        
2. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    fn print_slice<T>(v: &[T]) {
      for x in v {
        println!("{x}");
      }
    }
    fn main() {
      print_slice(&[1, 2, 3]);
    }
    ```
    
    - Ans
        
        This does NOT compile
        
        Since we are using the generic `T`, we cannot assume that it contains any functionality — even the functionality to print it
        
3. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    struct Point<T> { x: T, y: T }
    impl Point<i32> {
      fn f(&self) -> &i32 { &self.y }
    }
    impl<T> Point<T> {
      fn f(&self) -> &T { &self.x }
    }
    fn main() {
      let p: Point<i32> = Point { x: 1, y: 2 };
      println!("{}", p.f());
    }
    ```
    
    - Ans
        
        This program does NOT compile
        
        There are two methods with the same name `f` one of which uses a concrete type and one of which uses the generic type — which is not allowed.
        

## Traits: Defining Shared Behavior

- A *trait* defines functionality a particular type has and can share with other types
- We can use traits to define shared behavior in an abstract way
- We can use *trait bounds* to specify that a generic type can be any type that has a certain behavior
- Traits are similar to `interfaces` in other languages, but with some differences

### Defining a Trait

- A type’s behavior consists of the methods that we can call on that type
- Different types have the same behavior if we can call the same methods on all of those types
- Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose
- As a running example, take a case where we have multiple structs that hold various kinds and amounts of text: a `NewsArticle` struct that holds a news story filed in a particular location, and a `Tweet` struct that can have at most 280 characters along with metadata that indicates whether it was a new tweet, a retweet, or a reply to another tweet.
- We want to make a media aggregator library called `aggregator` that can display summaries of data that might be stored in a `NewsArticle` or `Tweet` instance. To do this, we need a summary from each type, and we’ll request by calling a `summarize` method on an instance.
- So, we define a `Summary` trait that expresses this behavior:
    
    ```rust
    pub trait Summary {
    	fn summarize(&self) -> String;
    }
    ```
    
    Here, we define a trait using the `trait` keyword that defines a behavior through a function called `summarize` that takes a type that implements this trait and returns a `String`. Instead of providing an implementation within curly braces, we use a semicolon. Each type implementing this trait must provide thier own custom behavior for the body of the method.
    
- A trait can have multiple methods defined on it and each one is defined on a separate line that ends with a semicolon.

### Implementing a Trait on a Type

- Let’s implement the `Summary` trait on the `NewsArticle` and `Tweet` structs:
    
    ```rust
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} {}", self.headline, self.author, self.location)
        }
    }
    
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    ```
    
    Here, we implement a trait for a type using the `for` keyword along with the `impl` keyword that is used to define methods on a type
    
- The above implemenation is similar to normal methods defined on a struct except that now, users have to bring the `trait` into scope when using the methods defined in the trait that these types implmement:
    
    ```rust
    use aggregator::{Summary, Tweet};
    
    fn main() {
    		let tweet = Tweet {
            username: String::from("tweeter"),
            content: String::from("this is my first tweet"),
            reply: false,
            retweet: true,
        };
    
        println!("1 new tweet: {}", tweet.summarize());
    }
    ```
    
    The above outputs:
    
    ```rust
    1 new tweet: tweeter: this is my first tweet
    ```
    
- There is an important restriction to what traits can be implemented by what types called the `orphan rule` (a part of a property called `coherence`) according to which:
    
    > you can’t implement external traits on external types
    > 
- For example:
    
    
    | Trait | Type | Okay? |
    | --- | --- | --- |
    | Summary (local) | Vec<T> (external) | YES |
    | Display (external) | Tweet (local) | YES |
    | Display (external) | Vec<T> (external) | NO |
- This rule ensures that other people’s code can’t break your code and vice versa. Without this trait, two crates could implement the same trait for the same type and Rust wouldn’t know which one to use.

### Default Implementations

- We can define default implemenations for some or all of the methods in a trait
- This eliminates the need for implementing all the methods in a particular trait for a given type while allowing us to override these default methods on our type
- We can define default implementations by writing out the implementation on the trait definition itself:
    
    ```rust
    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }
    ```
    
- To use the default implementation, we declare an empty `impl` block:
    
    ```rust
    impl Summary for NewsArticle {}
    ```
    
- Default implementations can also call other methods in the same trait, even if those methods do not have a default implementation.
- However, the method that the default method depends on should be implemented either by the trait itself or the type that is trying to invoke the default method.
- Example:
    
    ```rust
    pub trait Summary {
        fn summarize_author(&self) -> String;
    
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }
    ```
    
    Then, we cannot simply declare an empty `impl` block for `NewsArticle` that implements the `Summary` trait because the `summarize_author` method is now required to use the default `summarize` method.
    
    ```rust
    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
    }
    ```
    
    The default implementation of `summarize` now works just fine!logpoi
    
- We can of course, override the implementation as well:
    
    ```rust
    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    ```
    
    Note here that the `summarize_author` method is still required since it does not have a default implementation
    

### Quiz

1. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    trait MakeNoise {
      fn make_noise(&self) {
        println!("(silence)");
      }
    }
    struct Dog {}
    struct Cat {}
    impl MakeNoise for Dog {
      fn make_noise(&self) {
        println!("bark");
      }
    }
    impl MakeNoise for Cat {}
    fn main() {
      let dog = Dog {};
      let cat = Cat {};
      dog.make_noise();
      cat.make_noise();
    }
    ```
    
    - Ans
        
        This program DOES compile with the output:
        
        ```rust
        bark
        (silence)
        ```
        
        The `dog` struct has its own implementation of the `make_noise` method while the `Cat` struct does not and so uses the default implementation
        
2. The following are statements about what kinds of trait implementations are permitted by Rust. Select each statement which is true.
    - Ans
        - You can implement a local trait for a local type
        - You can implement a local trait for an external type
        - You can implement an external trait for a local type
        
        **Context**: The "orphan rule" requires that you cannot implement an external trait for an external type, to ensure code doesn't break if two crates provide conflicting implementations.
        

### Traits as Parameters

- We can use traits as parameters to a function that can now accept many different types (provided that each of these types implements the specified trait)
- For example, we can create a `notify` function that accepts the `Summary` trait:
    
    ```rust
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    ```
    
    The above can be invoked on a `tweet` that is an instance of `Tweet` with:
    
    ```rust
    notify(&tweet);
    ```
    
- Here, instead of a concrete type for the `item` parameter, we specify `impl` keyword and the trait name.
- This parameter accepts any type that implements the specified trait.

#### Trait Bound Syntax

- The `impl Trait`  syntax works for straightforward cases but is actually syntactic sugar for a longer form known as `trait bound`, that looks like:
    
    ```rust
    pub fn notify<T: Summary>(item: &T) {
    	println!("Breaking news! {}", item.summarize());
    }
    ```
    
- For simple cases, this is a bit verbose and the `impl Trait` approach is better in these cases. But more complex cases, the *trait bound* syntax is more clear:
    
    ```rust
    pub fn notify<T: Summary>(item1: &T, item2: &T) { ... }
    
    // instead of:
    
    pub fn notify(item1: &impl Summary, item2: &impl Summary) { ... }
    ```
    

#### Multiple Trait Bounds

- We can also specify more than on trait bound
- For example, we might want to specify that the `item` should also display formatting in addition to the summary.
- Then, the `item` parameter must implement both the `Display` and `Summary` traits.
- We can specify this constraint using the `+` operator:
    
    ```rust
    pub fn notify<T: Display + Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }
    ```
    
    With this specification, we need to implement the `Display` trait for the `Tweet` struct to be able to use the `notify` function
    

#### Clearer Trait Bounds with `where` Clauses

- When using too many trait bounds, it becomes unclear what the function signature actually is
- For such cases, Rust provides an alternate syntax using the `where` clause:
    
    ```rust
    fn some_function<T, U>(t: &T, u: &U) -> i32
    where
    	T: Display + Clone,
    	U: Clone + Debug,
    {
    ```
    
    instead of:
    
    ```rust
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    ```
    

### Returning Types that Implement Traits

- We can also use the `impl Trait` syntax in the function return to specify that the function returns a type that implements the specified trait:
    
    ```rust
    fn returns_summarizable() -> impl Summary {
    	Tweet {
    		username: String::from("tweeter"),
    		content: String::from("my first tweet"),
    		reply: false,
    		retweet: false,
    	}
    }
    ```
    
- This syntax allows us to specify that a function returns some type that implements a particular trait without needing to write out a long type
- However, you can only use `impl Trait` if you’re returning a single type. You cannot, for example, create a function that either returns a `Tweet` or a `NewsArticle` from a function that returns `impl Summary` even though both of them implement the trait!
- This is due to restrictions around how the `impl Trait` syntax is implemented in the compiler (more on this later)

### Using Trait Bounds to Conditionally Implement Methods

- By using a trait bound with an `impl` block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits
- For example, we can specify a method on the `Pair<T>` struct if its inner type `T` implements the `PartialOrd` trait (that enables comparison) *and* the `Display` trait that enables printing.
    
    ```rust
    use std::fmt::Display;
    
    struct Pair<T> {
        x: T,
        y: T,
    }
    
    // here, T is unrestricted
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }
    
    // here, T is restricted by the trait bounds specified
    // so only those types that confirm to these restrictions can use the `cmp_display` method
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
    ```
    
- We can also conditionally implement a trait for any type that implements another trait. For example, implementing a trait `A` on a type that implements trait `B`
- Implementations of a trait on any type that satisfies the trait bounds are called *blanket implementations* and are extensively used in the *Rust Standard Library*:
    
    ```rust
    impl<T: Display> ToString for T {
    	// implement the ToString trait on a type T that implements the Display Trait
    	...
    }
    ```
    

### Quiz

1. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    use std::fmt::Display;
    fn displayable<T: Display>(t: T) -> impl Display { t }
    fn main() {
      let s = String::from("hello");
      let mut s2 = displayable(s);
      s2.push_str(" world");
      println!("{s2}");
    }
    ```
    
    - Ans
        
        This does NOT compile
        
        **Context**: Because `displayable` returns `impl Display`, then we only know that `s2` is *some* type that implements `Display`, not that it is a `String` which has a `push_str` method. Therefore we cannot call `s2.push_str(..)`. If the return type of `displayable` was `-> T`, then this program would compile.
        
2. What is the smallest set of trait bounds on `T` needed to make this function type-check?
    
    ```rust
    fn f<T: /* ??? */>(t: &T) {
      let t2 = t.clone();
      println!("{t2}");
    }
    ```
    
    - Ans
        
        `Clone + Display`
        
        **Context**: Because `clone` is called and the `{}` display brackets are used in a formatter, then `T` must be `Clone` and `Display`.
        

## Validating References with Lifetimes

- A kind of generic that ensures a type has the behavior we want — ensuring that references are valid as long as we need them to be.
- Every reference in Rust has a *lifetime*, which is the scope for which that reference is valid
- Most of the times, lifetime of a reference is inferred just like the type — we only need to annotate types when multiple types are possible.
- Similarly, we need to specify the lifetime of a reference when there are multiple ways in which the lifetimes of references could be releated.
- Rust provides generic lifetime prarameters to ensure this.
- This is a concept that many programming languages do *not* have.

### Preventing Dangling References

- A *dangling reference* causes a program to reference data other than the data it’s intended to reference.
- For example, take the following program:
    
    ```rust
    let r;
    {
    	let x = 5;
    	r = &x;
    }
    println!("r: {}", r);
    ```
    
    Here, `r` holds the reference to `x`, but `x` goes out of scope by the time we display the value of `r` which leads to undefined behavior.
    
- Indeed, when we try to run this program, we get the following error:
    
    ```rust
    error[E0597]: `x` does not live long enough
     --> src/lifetimes.rs:6:13
      |
    5 |         let x = 5;
      |             - binding `x` declared here
    6 |         r = &x;
      |             ^^ borrowed value does not live long enough
    7 |     }
      |     - `x` dropped here while still borrowed
    8 |
    9 |     println!("The value of r = {r}");
      |                                --- borrow later used here
    ```
    
- Here, we say that `r` lives longer than `x`

### The Borrow Checker Ensures Data Outlives its References

- The Rust compiler’s borrow checker will compare scopes to determine whether all borrows are valid
- Let’s add lifetime annotations to the above code (similar to what Rust compiler does):
    
    ```rust
    		// lifetime of r begins, say 'b
        let r;
    
        {
            // lifetime of x begins, say 'a
            let x = 5;
            r = &x;
            // lifetime of x ends
        }
    
        println!("The value of r = {r}");
        // lifetime of r ends
    ```
    
- Here, `r` has the lifetime `'b` that is larger than the lifetime of `x` i.e, `'a`

### Generic Liftetimes in Functions

- Take a sample function that returns the longer of two string slices:
    
    ```rust
    fn longer(x: &str, y: &str) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    ```
    
    The above code does not compile and throws the following error message:
    
    ```rust
    error[E0106]: missing lifetime specifier
     --> src/lifetimes.rs:1:32
      |
    1 | fn longer(x: &str, y: &str) -> &str {
      |              ----     ----     ^ expected named lifetime parameter
      |
      = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
    help: consider introducing a named lifetime parameter
      |
    1 | fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
      |          ++++     ++          ++          ++
    ```
    
- The error message tells us that the function needs a generic lifetime parameter on it because Rust can’t tell whether the reference being returned refers to `x` or `y` (in this case, neither do we, because the function can return either `x` or `y`)
- The borrow checker can’t determine whether the reference returned by the function will always be valid because it doesn’t know the concrete lifetimes of the references that will be passed in.
- In short, the borrow checker does not know how the lifetime of the return value relates to the lifetime of the input parameters
- To fix this error, we add generic lifetime parameters that encode the relationship between the references so that the borrow checker can perform its analysis.

### Lifetime Annotation Syntax

- These annotations do not change how long references live, but rather they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes
- Just like in the case of generic type parameters, generic lifetime paramters allow functions to accept references with any lifetime
- Lifetime parameters must start with an apostrophe (`’`) and are usually all lowercase and very short (most use `'a`)
- We place lifetime parameter annotations after the `&` of a reference, using a space to separate the annotation from the reference’s type
- Examples:
    
    ```rust
    &i32        // a reference
    &'a i32     // a reference with an explicit lifetime
    &'a mut i32 // a mutable reference with an explicit lifetime
    ```
    
- To use lifetime annotations in function signatures, we need to declare the generic *lifetime* parameters inside angle brackets between the function name and the parameter list
- In the above case, we want to express the constraint that the returned reference should be valid as long as both the input references are valid. This is the relationship that we denote with `'a` and then, add it to each reference:
    
    ```rust
    fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    ```
    
- This function signature tells Rust that:
    - For some lifetime `'a`, the function takes two parameters, both of which are string slices that live at least as long as lifetime `'a`
    - The string slice returned from the function will live at least long as lifetime `'a`
- In practice, it means that the lifetime of the reference returned by the function is the same as the smaller of the lifetimes of the values referred to by the function arguments.
- These annotations tell Rust to reject code that does not adhere to the constraints specified by these annotations
- For example, the following code will fail to compile:
    
    ```rust
    let longer_string;
    {
        let string1 = String::from("hello,");
        let string2 = "world";
    
        longer_string = longer(&string1, string2);
    }
    println!("The longer string is {longer_string}");
    ```
    
    This fails to compile with the error message:
    
    ```rust
    error[E0597]: `string1` does not live long enough
      --> src/lifetimes.rs:15:32
       |
    12 |         let string1 = String::from("hello,");
       |             ------- binding `string1` declared here
    ...
    15 |         longer_string = longer(&string1, string2);
       |                                ^^^^^^^^ borrowed value does not live long enough
    16 |     }
       |     - `string1` dropped here while still borrowed
    17 |     println!("The longer string is {longer_string}");
       |                                    --------------- borrow later used here
    ```
    
    The Rust compiler is smart enough to know that in this particular case, `longer_string` would hold a reference to `string1` which does not live long enough (i.e, the reference is dropped by the time we try to display the `longer_string`)
    
    If we were to declare `string1` outside the block, the code would compile just fine
    
- Another case is when we declare the `string2` dynamically as well. In this case, even this code fails to compile:
    
    ```rust
    let longer_string;
    let string1 = String::from("hello,");
    {
        let string2 = String::from("world");
    
        longer_string = longer(&string1, &string2);
    }
    println!("The longer string is {longer_string}");
    ```
    
    This throws:
    
    ```rust
    error[E0597]: `string2` does not live long enough
      --> src/lifetimes.rs:15:42
       |
    13 |         let string2 = String::from("world");
       |             ------- binding `string2` declared here
    14 |
    15 |         longer_string = longer(&string1, &string2);
       |                                          ^^^^^^^^ borrowed value does not live long enough
    16 |     }
       |     - `string2` dropped here while still borrowed
    17 |     println!("The longer string is {longer_string}");
       |                                    --------------- borrow later used here
    ```
    

### Quiz

1. Which kind of programming error is a lifetime supposed to prevent?
    - Ans
        
        Using a reference to an object after its memory has been freed
        
2. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    fn shortest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
      if x.len() < y.len() {
        x
      } else {
        y
      }
    }
    fn main() {
      println!("{}", shortest("hello", "rust"));
    }
    ```
    
    - Ans
        
        This does NOT compile
        
        Because the return value only lives as much as the first parameter but we are returning a parameter that has the lifetime of the second parameter. The actual error message is:
        
        ```rust
        error: lifetime may not live long enough
          --> src/lifetimes.rs:13:9
           |
        9  | fn shorter<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
           |            --  -- lifetime `'b` defined here
           |            |
           |            lifetime `'a` defined here
        ...
        13 |         y
           |         ^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
           |
           = help: consider adding the following bound: `'b: 'a`
        ```
        

### Thinking in Terms of Lifetimes

- The lifetime that needs to be specified depends on what your function is doing
- If our `longest` function were to return just the first argument, then only annotating the lifetime for the first parameter would satisfy the compiler:
    
    ```rust
    fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    	x
    }
    ```
    
- The lifetime annotation of the return parameter must match one of the lifetimes of the input parameters
- It can reference the lifetime one of the variables created within the function itself but that variable would go out of scope as soon as the function ends! So, the following code fail to compile:
    
    ```rust
    fn longest<'a>(x: &str, y: &str) -> &'a str {
    	let result = String::from("Hello, world!");
    	result.as_str()
    }
    ```
    
    The best course here (if this indeed a required use case) is to just return the ownership instead of a reference
    

### Lifetime Annotations in Struct Definitions

- So far, we have only seen structs that hold owned types
- We can define structs that hold references but in this case, we need to add lifetime annotations:
    
    ```rust
    struct ImportantExcerpt<'a> {
    	part: &'a str,
    }
    ```
    
    we can use it thus:
    
    ```rust
    let novel = String::from("it was the best of time, it was the worst of ...");
    let first_part = novel.split(',').next().expect("could not find a ','");
    let i = ImportantExcerpt { part: first_part };
    
    println!("First part = {}", i.part);
    ```
    
- The reference means that an instance of the struct `ImportantExcerpt` cannot outlive the reference held by the `part` member.

### Lifetime Elision

- In the pre-1.0 Rust, every function that works on references and returns a reference would need lifetime annotations.
- Even the following code would not pass the borrow checker:
    
    ```rust
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
    
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
    
        &s[..]
    }
    ```
    
    unless we add the annotation:
    
    ```rust
    fn first_word<'a>(s: &'a str) -> &'a str {
    ```
    
- Over time, as this led to a lot of code repetition with deterministic patterns, the Rust team encoded certain deterministic patterns into the compiler itself so that lifetimes can be inferred implicitly. These patterns are called `Lifetime Elision Rules`
- These rules do not provide full inference. If upon applying the rules, there is stil ambiguity as to what lifetimes the references have, the compiler won’t guess what the lifetime of the remaining references should be. So, instead of guessing, the compile throws an error
- Lifetimes on function or method parameters are called `input lifetimes` and those on the return values are called `output lifetimes`

#### Rules

| Rule ## | Inferred | Annotated |
| --- | --- | --- |
| The compiler assigns a different lifetime parameter to each lifetime in each input type | fn foo(x: &i32)
fn foo(x: &i32, y: &i32)
fn foo(x: &ImportantExcerpt)  | fn foo<'a>(x: &'a i32)
fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
fn foo<'a, 'b>(x: &'a ImportantExcerpt<'b>) |
| If there is exactly one input lifetime parameter, that lifetime is assigned to all output liftetime parameters | fn foo(x: &i32) -> i32 | fn foo<'a>(x: &'a i32) -> &'a i32 |
| If there are multiple lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters | fn foo(&self, &x) | fn foo<'a>(&'a self, &'a x) |
- We’ll apply these rules to see what the compiler infers under the hood, for the `first_word` function defined above
- According to the first rule, each parameter gets a lifetime:
    
    ```rust
    fn first_word<'a>(s: &'a str) -> &str {
    ```
    
- According to the second rule, since there is only one input parameter, the same lifetime is applied to the output parameter:
    
    ```rust
    fn first_word<'a>(s: &'a str) -> &'a str {
    ```
    
- The third rule requires there to be `&self` or `&mut self` which does not apply in this case.
- Now, let’s try to annotate the `longer` function to see why we got an error when we did not add lifetime annotations.
- Applying the first rule:
    
    ```rust
    fn longer<'a, 'b>(x: &'a str, y: &'b str) -> &str {
    ```
    
- The second and third rule does not apply here
- Now, we are left with a missing lifteime annotation for the return type which is not acceptable!

### Lifetime Annotations in Method Definitions

- To demonstrate this, let’s implement a method on the `ImportantExcerpt` struct:
    
    ```rust
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }
    ```
    
- Here, the type annotation after `impl` and the struct are required but not for the `&self` parameter due to the third elision rule
- Let’s create another method that takes an additional parameter:
    
    ```rust
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please, {announcement}");
        self.part
    }
    ```
    
    Applying the first rule:
    
    ```rust
    fn announce_and_return_part<'a, 'b>(self: &'a self, announcement: &'b str)
    ```
    
    As the second rule does not apply, applying the third rule:
    
    ```rust
    fn announce_and_return_part<'a, 'b>(self: &a self, announcement: &b str) -> &a str
    ```
    
    This satsifies the lifetime requirement for the input and output parameters!
    

### The Static Lifetime

- References that are annotated with `'static` are special in that they live over the entire duration of the program
- All string literals have this annotation implicitly as they are baked into the program’s binary.
- The compiler might suggest adding the `static` lifetime when an error occurs, but we must decide whether or not this is actually necessary for our program.

### Mixing it all up

- Generic types, trait bounds and lifetime annotations can all go together as well.
- Consider, for example, this function:
    
    ```rust
    fn announce_the_longer<'a, T>(x: &'a str, y: &'a str, announcement: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement: {}", announcement);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    ```
    
- This function takes the lifetime parameter `'a` that is at least as long as shorter of the lifetimes of `a` and `b`
- It also takes another parameter `announcement` of the generic type `T` that is bound by the trait `Display`
- Both the lifetime and the generic type go inside the angular brackets as they are both a type of generic

### Quiz

1. If a reference has a lifetime `'static`, then this means:
    - Ans
        
        The data under the reference is never deallocated
        
2. Consider the following un-annotated function signature.
    
    ```rust
    struct Foo<'a> {
      bar: &'a i32
    }
    fn baz(f: Foo) -> &i32 { /* ... */ }
    ```
    
    Will Rust accept this function signature? If so, what lifetimes will it infer?
    
    - Ans
        
        `fn baz<'a>(f: Foo<'a>) -> &'a i32`
        
        (from the first and second rules)
        
3. Consider the following un-annotated function signature.
    
    ```rust
    struct Foo<'a> {
      bar: &'a i32
    }
    // Foo changed to &Foo
    fn baz(f: &Foo) -> &i32 { /* ... */ }
    ```
    
    Will Rust accept this function signature? If so, what lifetimes will it infer?
    
    - Ans
        
        Rust will reject this function signature
        
        **Context**: Rust will not compile this program, because it is ambiguous whether the lifetime of the output is tied to the lifetime of `&Foo` or the reference `Foo.bar`.
