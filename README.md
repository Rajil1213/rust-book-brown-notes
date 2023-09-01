# Object-Oriented Programming in Rust

## Introduction

- Object-Oriented Programming (OOP) is a way of modelling programs
- By some definitions, Rust is object-oriented and by some definitions, it is not
- This chapter explores features of Rust that are considered object-oriented, and how those characteristics translate to idiomatic Rust
- We’ll also explore how an object-oriented pattern can be implemented in Rust and its tradeoffs compared to using Rust’s core strengths

## Characteristics of Object-Oriented Languages

- Rust is influenced by many programming paradigms including OOP
- OOP languages share common characteristics, namely objects, encapsulation and inheritance

### Objects Contain Data and Behavior

> Object oriented programs are made up of objects. An *object* packages both data and procedures that operate on that data. The *procedures* are typically called *methods* or *method operations

- Design Patterns: Elements of Reusable Object-Oriented Software*by*The Gang of Four*
>
- By this definition, Rust is object-oriented because:
    - Structs and enums in Rust have data
    - and `impl` blocks provide methods on structs and enums
- This despite the fact that structs and enums with methods aren’t *called* objects

### Encapsulation that Hides Implementation Details

- *Encapsulation* means that the implementation details of an object aren’t accessible to code using the object
- So, the only way to interact with an object is via its public API — code using the object shouldn’t be able to reach into the object’s internals and change data/behavior directly
- This allows programmers to change the internals of an object (while keeping the API consistent) without needing to change the code that uses the object
- In Rust, encapsulation can be achieved by using the `pub` keyword that exposes the Public API along with the modules, types and functions we wish to expose, and by default, everything is private
- If encapsulation is a required aspect for a language to be considered object-oriented, then Rust meets that requirement

### Inheritance as a Type System and as Code Sharing

- Inheritance is a mechanism whereby an object can inherit elements from another object’s definition, thus gaining the parent object’s data and behavior without your having to define them again
- If a language must inheritance to be an object-oriented language, then Rust is not one
- There is no way to direclty get inheritance feature in Rust but Rust does provide other solutions, depending upon your reason for reaching for inheritance in the first place
- You would choose inheritance for two main reasons:
    - Reuse of code:
        - You can implement a behavior for a particular type and inheritance enables you to reuse that implementation for a different type
        - This can be done in a limited way using default trait method implementations.
        - For example, when we needed a default implementation of the `summarize` method on the `Summary` trait
        - Any type implementing the `Summary` trait would have the `summarize` method available on it without any further code
        - The default implementation can be overriden by other structs as well
        - This is similar to how a parent class may implement a method which may be overriden by a child class
    - Enable a child type to be used in place of the parent type
        - This is also called *polymorphism*
        - This means that you can substitute multiple objects for each other at runtime if they share certain characteristics
- Polymorphism
    - To many, this is synonymous with inheritance but it’s actually a more general concept that refers to code that can work with data of multiple types
    - For inheritance, those types are generally subclasses
    - Rust instead uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide — sometimes called `bounded parametric polymorphism`

> Inheritance has recently fallen out of favor as a programming design solution in many programming languages because it’s often at risk of sharing more code than necessary
Subclasses shouldn’t always share all characteristics of their parent class but will do so with inheritance
>
- Rust takes the different approach of using trait objects instead of inheritance

### Quiz

1. Which of the following aspects of object-oriented programming does Rust implement?
    - Ans
        - Encapsulation of private data
        - Objects with methods

## Polymorphism with Trait Objects

- In Chapter 8, we mentioned that one limitation of vectors is that they can store elements of only one type
- A workaround would be to use an enum with variants that can hold integers, floats and text
- The vec of this enum can now store multiple types (sum types)
- This is perfectly acceptable when our interchangeable items are fixed set of types that we know when our code is compiled
- However, sometimes we want our library user to be able to extend the set of types that are valid in a particular situation
- To demonstrate, we’ll implement a GUI library tool that iterates through a list of items, calling a `draw` method on each one to draw it to the screen
- The library will be called `gui` (and won’t be a fully-fledged GUI library)
- The library will include some defaults such as `Button` or `TextField` but users will want to create their own types as well that can be drawn — such as an `Image` or `SelectBox`
- At the time of writing, we cannot be sure of all the types other programmers might want to create
- But we do know that `gui` needs to keep track of many values of different types, and it needs to call the `draw` method on each of these differently typed values
- It doesn’t need to know what will exactly happen when we call the `draw` method — just that the `draw` method will be available on the object
- In a programming language that has inheritance, this can be achieved by:
    - defining a class called `Component` that has a method named `draw` on it
    - other classes like `Button`, `Image`, `SelectBox` would inherit from `Component` and thus inherit the `draw` method
    - each child could override the `draw` method to define their own custom behavior
    - the framework can treat each of these types as if they were `Component` instances and call `draw` on them
- In Rust, in the absence of inheritance, we’ll need a different approach for creating the `gui` library

### Defining a Trait for Common Behavior

- To implement the behavior, we’ll create a trait called `Draw` that will have one method named `draw`
- Then, we can define a vector that takes a `trait` object — that points to both an instance of a type implementing our specified trait. and a table used to look up trait methods on that type at runtime
- We can create a trait object by:
    - specifying some sort of pointer, such as `&` or `Box<T>` smart pointer
    - then, the `dyn` keyword
    - then, specifying the relevant trait
- Trait objects can be used in the place of generic or concrete type
- We we use a trait object, Rust’s type system ensures that at compile time, any value used in that context will implement the trait objects’s trait
- This means that we do not need to know all the possible types at compile time
- While `struct` and `enum` are not called “objects” since the data and behavior defined on them are not coupled together like in objects, a `trait object` is more like an object in the conventional sense
- But trait objects differ from traditional objects in that we can’t add data to a trait object — their only purpose is to add a layer of abstraction over common behavior
- Let’s define a trait object for our `gui` library:
    
    ```rust
    pub trait Draw {
        fn draw(&self) -> String;
    }
    ```
    
    This is similar to how traits are defined as shown in Chapter 10
    
- Now, we need to define the `vec` of types that implement the above trait:
    
    ```rust
    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }
    ```
    
    Here,
    
    - we define a struct named `Screen` with a single member `components`
    - `components` is a vector of `Box<dyn Draw>` which is a trait object
    - `dyn Draw` is a stand-in for any type that implements the `Draw` trait
- On this `Screen` struct, we’ll define a method `run` that will call the `draw` method on each of the components:
    
    ```rust
    impl Screen {
        pub fn run(&self) -> String {
    				let mut output = vec![];
            for component in self.components.iter() {
                output.push(component.draw());
            }
    
    				output
        }
    }
    ```
    
- While the syntax may be similar to how struct that uses a generic type parameter with trait bounds work, they are actually different
- A generic type parameter can only be substituted with one concrete type at a time, whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime
- A generic implementation would have looked like the following:
    
    ```rust
    pub struct GenericScreen<T> {
        pub components: Vec<T>,
    }
    
    impl<T> GenericScreen<T>
    where
        T: Draw,
    {
        pub fn run(&self) -> String {
    				let mut output = vec![];
            for component in self.components.iter() {
                output.push(component.draw());
            }
    
    				output
        }
    }
    ```
    
    In this case, since a generic can only accept one concrete type at a time, our `components` vec is restricted to contain only components of a single concrete type i.e, we cannot have a components vector that contains both a `Button` and `TextField` since they are not of the same type `T` although they both implement `Draw`!
    
### Implementing the Trait

- Now, let’s create some types that implement `Draw`:
    
    ```rust
    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }
    
    impl Draw for Button {
        fn draw(&self) {
            println!("drawing a button");
        }
    }
    ```
    
- Here, the data associated with `Button` is specific to a `Button` in the GUI and it might have other behaviors associated with it as well
- But each type we create will need to implement `Draw` so that it can work with `Screen`
- For example, we or the user of our library could have just as easily created a `SelectBox`:
    
    ```rust
    #[test]
    fn draws_a_custom_component() {
        struct SelectBox {
            width: u32,
            height: u32,
            options: Vec<String>,
        }
    
        impl Draw for SelectBox {
            fn draw(&self) -> String {
                format!(
                    "drawing a select box with height: {}, width: {}, options: {:?}",
                    self.height, self.width, self.options
                )
            }
        }
    
        let options = vec![String::from("option A"), String::from("option B")];
        let width = 10;
        let height = 12;
        let select_box = SelectBox {
            width,
            height,
            options: options.clone(),
        };
    
        let screen = Screen {
            components: vec![Box::new(select_box)],
        };
    
        assert_eq!(
            vec![format!(
                "drawing a select box with height: {}, width: {}, options: {:?}",
                height, width, options,
            )],
            screen.run()
        );
    }
    ```
    
### Using the Trait

- With our trait object, we can also create a `Screen` instance which holds components of different concrete types that implement the `Draw` trait object:
    
    ```rust
    #[test]
    fn draws_two_different_components() {
        const WIDTH: u32 = 10;
        const HEIGHT: u32 = 12;
        let placeholder: String = String::from("placeholder text");
    
        struct TextField {
            width: u32,
            height: u32,
            placeholder: String,
        }
    
        impl Draw for TextField {
            fn draw(&self) -> String {
                format!(
                    "drawing a text field with width: {}, height: {}, placeholder: {}",
                    self.width, self.height, self.placeholder
                )
            }
        }
    
        let screen = Screen {
            components: vec![
                Box::new(Button {
                    width: WIDTH,
                    height: HEIGHT,
                    label: String::from("test button label"),
                }),
                Box::new(TextField {
                    width: WIDTH,
                    height: HEIGHT,
                    placeholder: placeholder.clone(),
                }),
            ],
        };
    
        assert_eq!(vec![
            String::from("drawing a button"),
            format!("drawing a text field with width: {WIDTH}, height: {HEIGHT}, placeholder: {placeholder}"),
        ], screen.run());
    }
    ```
    
- When we created the library, we did not know that someone might add the `TextField` type but our `Screen` implementation was able to operate on the new type and draw it because `TextField` implements the `Draw` trait
- This concept of being concerned only with the messages a value responds to rather the value’s concrete type is similar to the concept of *duck typing* in dynamically typed languages — if it walks like a duck and quacks like a duck, then it must be a duck!
- In the implementation of `run` on the `Screen`, `run` doesn’t need to know what the concrete type of each component is
- By specifying `Box<dyn Draw>` as the type of the values in the `components` vector, we’ve defined `Screen` to need values that we can call the `draw` method on
- Rust’s type system along with trait objects ensure that the compiler rejects code where a type does not implement a trait that a trait object needs — without us having to check whether a value implemenets a trait explicitly

### Trait Objects and Type Inference

- One downside of using trait objects is how they interact with type inference
- For example, consider the following example:
    
    ```rust
    let v = vec![]; // type annotation needed
    
    let v = vec!["Hello world"]; // all is well; type can be inferred from single type
    ```
    
- In case of trait objects, the above strategy does not work. For example:
    
    ```rust
    let components = vec![
    	Box::new(SelectBox { /* ... */ }),
    	Box::new(Button { /* ... */ }),
    ];
    
    let screen = Screen { components };
    screen.run();
    ```
    
    The above code does not compile because Rust’s type inference expects `vec` to hold elements of a single type `T` and `SelectBox` and `Button` are not of the same type
    
    So, we need to explicitly type `components` with `Box<dyn Draw>`
    
    ```rust
    // solution 1:
    let components: Vec<Box<dyn Draw>> = vec![
    	Box::new(SelectBox { /* ... */ }),
    	Box::new(Button { /* ... */ }),
    ];
    
    // solution 2:
    let components = vec![
    	Box::new(SelectBox { /* ... */ }) as Box<dyn Draw>,
    	Box::new(Button { /* ... */ }),
    ];
    ```
    
    The above solutions work so long as the types that we include in the vec implement the specified trait object
    
### Trait Objects Perform Dynamic Dispatch

- In the chapter on Generics, we learnt that Rust performs monomorphization when we use trait bounds on generics — the compiler generates non-generic implementations of functions and methods for each concrete type that we use in place of a generic type parameter
- The code that results from monomorphization is doing *static dispatch —* the compiler knows what method you’re calling at comile time
- This is opposed to *dynamic dispatch* — the compiler can’t tell at compile time which method you’re callling
- In this approach, the compiler emits code that at runtime will figure out which method to call
- When we use trait objects, Rust must use dynamic dispatch
- Rust uses pointers inside the trait object to know which method to call
- This lookup incurs a runtime cost that doesn’t occur with static dispatch
- This also prevents Rust from choosing to inline a method’s code — which in turn, prevents some optimizations
- The tradeoff is between these optimizations and the extra flexibility we get

### Quiz

1. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    use std::fmt::Debug;
    fn main() {
        let n = 1;
        let s = String::from("Hello");
        let v: Vec<&dyn Debug> = vec![&n, &s];
        let n_ref = v[0] as &i32;
        println!("{}", n_ref + 1);
    }
    ```
    
    - Ans

        This program **DOES NOT** compile

        **Context**: Unlike some OOP languages, a trait object cannot be "downcasted" to a more concrete type (except in the case of the `[Any](https://doc.rust-lang.org/std/any/index.html)` trait).

2. Consider implementing a generic function in two ways:
    
    ```rust
    fn f_opt1<T: SomeTrait>(t: &T) { /* ... */ }
    fn f_opt2(t: &dyn SomeTrait) { /* ... */ }
    ```
    
    Which of the following are valid reasons to prefer the trait object version (`f_opt2`) over the type-parametric version (`f_opt1`)? Select each that applies.
    
    - Ans
        - Faster compile times
        - Smaller binary size

        **Context**: By avoiding monomorphization, trait-object functions will be both faster to compile and produce smaller binaries. However, this usually comes at the cost of runtime performance. Trait objects are rarely easier for developers to work with than trait-bounded generics.

## Implementing an Object-Oriented Design Pattern `State`

- The `state` pattern is an object-oriented design pattern where we define a set of states a value can have internally
- The states are represented by a set of `state` objects, and the value’s behavior changes based on its state
- We’re going to work through an example of a blog post struct that has a field to hold its state, which will be a state object from the set `draft`, `review` or `published`
- The state objects share functionality — each object is responsible for its own behavior and for governing when it should change into another state
- The value that holds the state knows nothing about the behavior of individual states and when they should transition
- The advantage of using the state pattern is that, when the business requirements of the program change, we don’t need to change the code of the value holding the state or the code that uses the value — we’ll only need to update the code inside of thte state objects to change its rules or perhaps add more state objects
- First, we’ll implement the functionality using a traditional object-oriented way, and then, we’ll look at how we can implement it in a way more natural to Rust
- The end result will look like this:
    - A blog post starts as an empty draft
    - When the draft is done, a review of the post is requested
    - When the post is approved, it gets published
    - Only published blog posts return content to print, so unapproved posts can’t accidentally be published
- Any other changes attempted on a post should have no effect. For example, if we try to approve a draft blog post before we’ve requested a review, the post should remain an unpublished draft
- Here is a sample usage of the API:
    
    ```rust
    // src/lib.rs
    #[cfg(test)]
    mod blog {
        use super::Post;
    
        #[test]
        fn only_allows_approved_content_to_be_posted() {
            let mut post = Post::new();
            let content = "I ate a salad for lunch today";
    
            post.add_text(content);
            assert_eq!("", post.content());
    
            post.request_review();
            assert_eq!("", post.content());
    
            post.approve();
            assert_eq!(content, post.content());
        }
    }
    ```
    
    The above test does not even compile but it provides a skeleton for how our API should work
    
    Here,
    
    - We want users to be able to add text to their blog post
    - But if they try to get the content directly, before approval, they shouldn’t get any text because the post is still a draft
    - Next, we want to be able to request for a review of the post, and we want `content` to still return an empty string while waiting for a review
    - When the post receives approval, it should get published, meaning the text of the post will be returned when `content` is called
- We’re only using the `Post` type from the `Blog` crate
- This type wil use the state pattern and will hold a value that will be one of three state objects representing the various states a post can be — draft, waiting for review, or published
- Changing from one state to another will be handled internally within the `Post` type
- These changes will be effected in response to the methods called on the `Post` instance but they don’t have to manage the state changes directly
- This also prevents users from messing up the state changes — for example, publishing a post before it has been reviewed

### Defining `Post` and Creating a New Instance in the Draft State

- Let’s create an initial implementation for `Post`:
    
    ```rust
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }
    
    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }
    }
    
    trait State {}
    struct Draft {}
    
    impl State for Draft {}
    ```
    
- This implementation is largely empty but note that we have created a `Post` struct that holds the `state` and the `content` of the `Post`
- The `state` is an `Option` that holds the `Box` of any type that implements the `State` trait
- When a new `Post` is created, the `state` will be a `Some` variant in `Draft` state
- Also note that `state` is private which means that it cannot be accessed directly from the outside (via an instance of `Post`)
- The content for a new post is set to an empty string
- We will implement the actual trait object (`State`) and the `Draft` state later (that implements the trait)

### Storing the Text of the Post Content

- In our test, we declare that we need an `add_text` function that adds text to the post
- We add this as a method instead of exposing the `content` field directly
    
    ```rust
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    ```
    
- This `add_text` function takes a mutable (exclusive) reference to `self` because we need to be able to modify the `Post` instance that we’re calling this method on
- This behavior does not depend on the state that the post is on, so it’s not part of the state system. We can of course, change the state [back] to `Draft` whenever any text is added

### Ensuring the Content of a Draft Post is Empty

- When the post is in Draft state, we want an empty string to be returned by `content` even though there may be text in it
- For now, let’s create a dummy implementation:
    
    ```rust
    pub fn content() -> &str {
        ""
    }
    ```
    
- With this, the first of our assertions works!

### Requesting a Review of the Post Changes its State

- Next, we need to add the `request_review` functionality
- This call should change the internal state from `Draft` to `PendingReview`
    
    ```rust
    		
    impl Post {
    		// --snip---
    
    		pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }
    }
    
    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
    }
    struct Draft {}
    
    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }
    }
    
    struct PendingReview {}
    
    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }
    ```
    
- We first implement the `request_review` method on the `Post`.
- This method will be responsible for setting the `State` of the `Post` to `PendingReview`.
- We use `take` so that the `Some` value moves out, leaving `None` in its place, effectively clearing the state — ensuring that the old state value is not usable anymore
- Then, we set the state to whatever the next state should be by calling the `request_review` method on the `state` value
- Now, we need to implement this `request_review` method on the `Draft` state
- When a `Post` is in the `Draft` state, calling `request_review` on it should move the state to the `PendingReview` state — so that is what we return (wrapped in a `Box` to match the method signature on the trait object)
- This also means that we need to create a `PendingReview` struct which should implement the `State` trait i.e, we need a `request_review` method on it
- When a `Post` is in the `PendingReview` state, calling `request_review` should not change the state, so the state will still be `PendingReview` i.e, `self`
- With this implementation, we ensure that the `request_review` method on the `Post` is the same no matter what the underlying implementation of `request_review` is on the individual `State` value

### Adding `approve` to Change the Behavior of `content`

- The `approve` method should change the state from `PendingReview` to `Published`
- As a side effect, the `content()` method should return the actual text and not an empty string
- Let’s start by adding the logic for state transition:
    
    ```rust
    pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }
    }
    
    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
    }
    struct Draft {}
    
    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }
    
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }
    
    struct PendingReview {}
    
    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
    
        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }
    }
    
    struct Published {}
    
    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
    
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }
    ```
    
- As before, we start by implementing the `approve` method on the `Post` and then, take a top-down approach to implement it first on the `State` trait object and then, the actual State structs that implement it
- Now, we need to implement the `content()` method. We want to delegate this task to the actual state so that a value is returned depending upon the state and our `Post` struct does not have to know about it
    
    ```rust
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    ```
    
    This implementation gets a reference to `state` and `unwraps` it to get the `Some` value. We don’t want the state to be accessed when it is in the `None` state. For that reason, the code will panic if the `state` is `None` (although there are more graceful ways to handle this. Then, we call the `content` method (yet to be defined) that takes the `Post` instance from which to get the actual content
    
- Now, let’s define the `content` method on our trait object `State`
- Since we only need the actual conent in one of the states, we’ll add a default implementation for the `content()` method so that an empty string is returned:
    
    ```rust
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    ```
    
    Also note that this method needs lifetime annotations because it cannot be elided in this case. The content is tied to the lifetime of the `Post` and not the `State` because we’re returning a part of the `Post`. So, we use the lifetime generic for the method, the post and the return value, and leave Rust to use a random value for `self`
    
- This method will be overriden by the `Published` state:
    
    ```rust
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    ```
    
- With this all our assertions pass:
    
    ```rust
    ╰─λ cargo t
        Finished test [unoptimized + debuginfo] target(s) in 0.00s
         Running unittests src/lib.rs (target/debug/deps/blog-dd2b2053157c0228)
    
    running 1 test
    test blog::only_allows_approved_content_to_be_posted ... ok
    
    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    ```
    
### Trade-offs of the State Pattern

- We’ve seen that Rust is capable of implementing the object-oriented state pattern to encapsulate the different kinds of behavior a post should have in each state
- The methods on `Post` know nothing about the specific behaviors of each state
- With this type of implementation, we have to look in only one place to know the different ways a published post can behave — the implementation of the `state` trait on the `Published` struct
- It is possible to use an `enum` to store the state variants instead of using the `state` pattern, we will need to use `match` expressions in the methods on the `Post` or even in the `main` code that check the state of the post and change behavior in those places
- This would also mean we would have to look in several places to understand all the implications of a post being in the published state — this complexity would only increase when we start adding more states
- With the `state` pattern, all we need to do to add a new state is to create a struct and implement the `State` trait
- However, there are a few downsides to this approach as well:
    - Because states implement the transition, they are coupled together — we’ll have to change quite a lot of things if we add a state between any two states
    - We’ve also duplicated some logic, especially the `request_review` and `approve` methods. We could use a default implementation that returns `Self` but this would violate object safety because the trait doesn’t know what the concrete `self` will be exactly
    - The implementation of `request_review` and `approve` on the `Post` are also duplicated since both of these just delegate to the corresponding method on the `state` to update the state. If we had a lot of methods with this same pattern, we would be better off using a macro (more on this in later chapters)
    - By adhering strictly to this OOP pattern, we are not taking advantage of Rust’s strengths as well as we could have
- To see the advantage of this pattern, you can implement the following additional functionalities:
    - Add a `reject` method that changes the post’s state from `PendingReview` back to `Draft`
    - Require two calls to `approve` before the state can be changed to `Published`.
    - Allow users to add text content only when a post is in the `Draft` state. Hint: have the state object responsible for what might change about the content but not responsible for modifying the `Post`.
    
    All of the above have been implemented in the repo for reference.
    
### Encoding States and Behavior as Types

- We’ll see how to rethink the state pattern to get a different set of trade-offs
- Rather than encapsulating the states and transitions completely so outside code has no knowledge of them, we’ll encode the states into differen types
- This will allow Rust’s type checking system to throw a compiler error if we violate any of the principles set about in the type system — for example, using draft posts where only published posts are allowed
- The overall API (and tests) will remain *mostly* the same
- Let’s start with the `content`. Rather than have the `content()` method return an empty string for `Draft` posts, we’ll create a `DraftPost` struct that does not even have a `content` method
- This way, if we try to use the `content()` method, we’ll get a compiler error telling us that the method does not exist on a `DraftPost`
    
    ```rust
    struct Post {
        content: String,
    }
    
    struct DraftPost {
        content: String,
    }
    
    impl Default for Post {
        fn default() -> Self {
            Self::new()
        }
    }
    
    impl Post {
        pub fn new() -> Post {
            Post {
                content: String::new(),
            }
        }
    
        pub fn content(&self) -> &str {
            &self.content
        }
    }
    
    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }
    }
    ```
    
- Both the `Post` and `DraftPost` have private `content` fields that store the blog post text
- The structs no longer have the `state` field because we’re moving the encoding of the state to the types of the structs
- The `Post` struct will represent a published post that has the content method defined on it
- Calling the `new` function on the `Post` returns a `DraftPost` instead of a `Post`
- At this point, there is no way to create a new `Post`
- This `DraftPost` has an `add_text` method defined on it that allows us to add text to the content of the `DraftPost`
- So, the compiler enforces that we cannot call the `content()` method on the `DraftPost`

### Implementing Transitions as Transformations into Different Types

- To get a published post, the draft post has to first change into a pending review post and it can then change to a published post upon approval
- So, we define the respective methods on each state-struct that return the next state
- For example, on the `DraftPost`, we define a `request_review` method that returns a `PendingReviewPost` struct, and a `review` method on the `PendingReviewPost` that returns a `Post` that has a `content` method!
    
    ```rust
    struct PendingReviewPost {
        content: String,
    }
    
    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }
    
        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }
    
    impl PendingReviewPost {
        pub fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }
    }
    ```
    
- With this approach, the `content` becomes straight up inaccessible from a `DraftPost` or a `PendingReviewPost`ensuring that the `content` within these structs cannot be accessed:
    
    ```rust
    #[test]
    fn only_allows_approved_content_to_be_posted() {
        let mut post: DraftPost = Post::new();
        let content = "I ate a salad for lunch today";
    
        post.add_text(content);
        // assert_eq!("", post.content()); // no method called `content`
    
        let post: PendingReviewPost = post.request_review();
        // assert_eq!("", post.content()); // no method called `content`
    
        let post: Post = pending_review_post.approve();
        assert_eq!(content, post.content());
    }
    ```
    
- The changes we needed to make to reassign the `post` mean that this implementation doesn’t quite follow the object-oriented state pattern anymore: the transformations between states are no longer encapsulated entirely within the `Post` implementation.
- Our gain is that invalid states are now impossible because of the type system and the type checking that happens at compile time!
- This ensures that certain bugs like publishing of an unreviewed post is downright impossible — and checked at compile time
- We can also implement the extra features (as implemented in the state pattern) in the types approach. Note that the third feature namely, being able to add text to only draft posts is already implemented. For the the second feature, we need to modify the `approve()` method so that it may return either a `PendingReviewPost` or `Post` depending upon the number of approvals.

### Quiz

In the running example from this section, a blog post could have one of three states: `Draft`, `PendingReview`, and `Published`. In the example, each state was represented as a struct, and state transitions was represented as trait methods returning trait objects.

An alternative implementation could represent the states as an enum, like this:

```rust
enum BlogState {
    Draft,
    PendingReview,
    Published
}
impl Post {
    fn request_review(&mut self) {
        use BlogState::*;
        self.state = match self.state {
            Draft => PendingReview,
            PendingReview => PendingReview,
            Published => Published
        }
    }
}
```

Which of the following are valid reasons to prefer the struct/trait representation over the enum representation?

- Ans
    - Adding a new state does not require modifying methods on other states
    - An API client could add a new state to the system
    
    **Context**: The struct/trait approach is extensible in the sense that an API client could potentially create a new state (such as `Retracted`) without changing the core API functionality. When adding this state, the methods for other states do not need to be changed. Whereas with enums, a client cannot add a new branch to the enum. Moreover, all `match` expressions must be updated when a state is added.
    
    A `match` is not likely to be slower than dynamic dispatch. A match is a simple branch based on an enum's tag, while dynamic dispatch requires a layer of indirection through a trait object's virtual table with non-inlined function calls.
    
    An API client cannot add a new *method* for existing states in the struct/trait approach, they can only add new states. The methods are fixed by the API author's trait definition. Note that you could add a new method which only builds on existing methods via extension traits, such as:
    
    ```rust
    trait StateExt {
        fn request_review_twice(self: Box<Self>) -> Box<dyn State>;
    }
    
    impl<S: State> StateExt for S {
        fn request_review_twice(self: Box<Self>) -> Box<dyn State> {
            self.request_review().request_review()
        }
    }
    ```
    
    But these extensions cannot read the private data of the states
