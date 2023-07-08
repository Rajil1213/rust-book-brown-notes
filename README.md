# Managing Growing Projects with Packages, Crates and Modules
## Overview

- As the codebase size grows, organizing it will become increasingly important.
- By separating code into separate “packages” with distinct features, it will be clear where to find code that implementes a particular feature
- As a project grows, you should organize code by splitting it into multiple modules and then, multiple files
- A package can contain multiple binary crates and optionally one library crate
- As a package grows, you can extract parts into separate crates that become external dependencies.
- For very large projects comprising of a set of inter-related packages that evolve together, Cargo provides *workspaces* (disccussed later).
- This chapter covers:
    - Packages and crates as described above
    - Encapsulating implementation details (abstraction)
    - Scoping
    - Module system:
        - `Packages`: A Cargo feature that lets you build, test and share crates
        - `Crates`: A tree of modules that produces a library or executable
        - `Modules` and `use`: Let you control the organization, scope, and privacy of paths
        - `Paths`: A way of naming an item, such as a struct, function or module.
    - The interaction between the above

## Packages and Crates

- `Crate`
    - The smallest amount of code that the Rust compiler considers at a time.
    - Even when compiling a single file with `rustc`, it considers the file as a `crate`.
    - A crate can contain modules which may be defined in other files that get compiled with the crate
    - A crate can either be:
        - a binary crate:
            - programs that can compile to an executable that you can run, such as a command-line program or server
            - must have a `main` function that defines what happens when the executable is run
        - a library crate:
            - define functionality intended to be shared with multiple projects
            - don’t have a `main` function
    - In Rust lingo, crate *usually* means **library** crate and used interchangeably with `library`
    - The `crate root` is a source file that the Rust compiler starts from and makes up the root module of your crate
- `Package`
    - A bundle of one or more crates that provides a set of [related] functionality
    - A package contains a `Cargo.toml` file that describes how to build those crates
    - A package can be created with the `cargo new <package_name> [--lib]` command which creates a directory named `<package_name` with a file called `Cargo.toml`, and `src/main.rs` file.

### More on Cargo

- `Cargo` itself is a package that contains the binary crate for the command-line tool
- `Cargo` also contains a library crate that the binary crate depends on
- Other projects can depend on the Cargo library crate to use the same logic the Cargo command-line tool uses
- Cargo follows the following conventions:
    - `src/main.rs` is the crate root of a binary crate with the same name as the package
    - `src/lib.rs` is the crate root of the library crate with the same name as the package
- Cargo passes the crate root files to `rustc` to build the library or binary
- A package can have multiple binary crates by placing files in the `src/bin` directory: each file will be a separate binary crate.

### Quiz

1. Which is the correct order, where "A > B" means "A contains B"?
    - Ans
        
        package > crate > module
        
2. Imagine you see a Rust package `foobar` with the following files:
    
    ```rust
    foobar
    ├── Cargo.toml
    ├── build.rs
    └── src/
        ├── main.rs
        ├── util.rs
        ├── lib.rs
        └── bin/
            └── alt.rs
    ```
    
    How many crates does this package contain? Write your answer as a digit, e.g. 0, 1, and so on.
    
    - Ans
        
        3 namely, main.rs (binary crate with name `foobar`, lib.rs (library crate), and alt.rs (binary crate with name `alt`)
        

## Modules

### How do Modules Work?

1. **Start from the Crate Root**
    - When compiling a crate, the compiler first looks in the **crate root** file (*usually* `src/lib.rs` for a library crate and `src/main.rs` for a binary crate) for the code to compile
2. **Declaring Modules**
    - In the crate root file, you can declare multiple modules with the `mod <module_name>;` syntax.
    - he crate will look for the module’s code in the following places:
        - Inline, within curly brackets that replace a semicolon following `mod <module_name>` as:
            
            ```rust
            mod some_module {
            	... // contents
            }
            ```
            
        - In the file, `src/<module_name>.rs`
        - In the file `src/<module_name>/mod.rs`
3. ****************************************Declaring Submodules****************************************
    - In any file other than the create root file, you can declare submodules.
    - If you declare `mod vegetables;` in the file `src/garden.rs`, the compiler will look for the submodule’s code within the directory names for the parent module (in this case, `garden`) in the following places:
        - Inline, directly following `mod vegetables` wihtin curly braces instead of the semicolon
        - In the file, `src/garden/vegetable.rs`
        - In the file, `src/garden/vegetable/mod.rs`
4. Paths to code in modules
    - Once a module is a part of your crate, you can refer to code in that module from anywhere in the same crate, as long as the privacy rules allow, using the path to the code
    - For example, an `Asparagus` type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`
5. Private vs public
    - Code wihtin a module is private from its parent modules by default
    - To make a module public, declare it with `pub mod` instead of `mod`.
    - To make items within a public module public as well, use `pub` before their declarations
6. The `use` keyword
    - Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths.
    - For example, `use crate::garden::vegetables::Asparagus` allows us to use `Asparagus` directly within the scope

### Quiz

1. Which of the following is NOT a good reason to use a module?
    - Ans
        
        Modules boost the runtime performance of interdependent code within the same module
        

### Paths for Referring to an Item in the Module Tree

- To show Rust where to find an item in a module tree, we use a path in the same way that we use a path when navigating a filesystem.
- A path can take two forms:
    - An *absolute* path that starts from the root crate which is the crate’s name for an external crate and the literal `crate` for a local one
    - A *relative* path starts from the current module and uses `self`, `super` or an identifier in the current module
- Both types of paths are followed by one or more identifier separated by double colons (`::`)
- Example:
    
    ```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    
        mod serving {
            fn take_order() {}
        }
    }
    
    fn eat_at_restaurant() {
        // Absolute
        crate::front_of_house::hosting::add_to_waitlist();
    
        // Relative
        front_of_house::hosting::add_to_waitlist();
    }
    ```
    
    Here, in the first approach, we use the absolute path by starting with the `crate` keyword and then, moving thorugh the filesystem to the location of the function we want to call. In the second approach, we start with `front_of_house` which is defined at the same level at the current module.
    
    Also, notice that we are using the `pub` keyword before the module and the function that we are trying to use.
    
- The decision to use absolute or relative paths depends on how the modules are shipped.
    - If the function is in the same package, either one works
    - If the function is in a separate package or module, the absolute version works just fine but the relative one will need to change
- The `pub` keyword is required because the Rust module hides implementation details of a module by default.
    - `front_of_house` does not need to be public because it is defined in the same file (and so the same scope) as `eat_at_restaurant` i.e, they are siblings
    - `hosting` needs to be public because it is in a different scope. Using the `pub` keyword makes the function accessible through its parent module
    - `add_to_waitlist` also needs to be public because of the same reason
- Whether a function or module needs to be public depends on the kind of API that is being designed. More on this in the [The Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).

#### Best Practices for Packages with a Binary and a Library

- A package can contain both a binary crate (defined in `src/main.rs`) and a library crate (defined in `src/lib.rs`)
- Packages with this structure will have enough code in the binary crate to start an executable that calls code with the library crate
- This pattern allows other projects to take advantage of the functionality by the binary because the library carate can be shared
- The binary crate essentiallly becomes the user of the library crate (just like any other external crate)
- The library crate’s API should then be designed in a way that it can be used both by the binary crate and other external users.

#### Quiz

1. What is the keyword you use at the start of an absolute path to an item in the current crate?
    - Ans
        
        `crate`
        
2. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    pub mod foo {
      fn a() { println!("a"); }
      mod bar {
        pub fn b() { println!("b"); }
      }
    }
    fn main() {
      foo::bar::b();
    }
    ```
    
    - Ans
        
        This program **does not** compile (because `bar` is not public)
        
3. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    pub mod foo {
      pub mod bar {
        pub fn b() { println!("b");  }
      }
      pub fn a() { bar::b(); }
    }
    fn main() {
      foo::a();
    }
    ```
    
    The program does compile with output `b` because `a` is publicly accessible from `main` and `bar::b()` is publicly accessible from `a` with the relative path as they are in the same scope
    

#### Starting Relative Paths with `super`

- We can construct relative paths that begin in the parent module, rather than the current module or the crate root, by using the `super` at the start of the path
- This is like using the `..` syntax in the UNIX filesystem
- Example:
    
    ```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
    
        }
    
        pub mod serving {
            pub fn serve_order() {}
        }
    }
    
    mod back_of_house {
        fn fix_incorrect_order() {
            cook_order();
            super::front_of_house::serving::serve_order();
    				// equivalent to:
            crate::front_of_house::serving::serve_order();
        }
    
        fn cook_order() {}
    }
    ```
    
    Here, the `fix_incorrect_order` function access the `serve_order` function defined in an outside module.
    

### Making Stucts and Enums public

- Just like in the case of functions, we can make structs and enums defined within the scope of a module public
- However, each field in the struct will have to be made public separately and for enums, all the variants need to be accessible so they become public if the enum itself is public:
    
    ```rust
    mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }
    
        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }
    }
    
    fn eat_at_restaurant() {
    		let mut meal = back_of_house::Breakfast::summer("Rye");
        meal.toast = String::from("wheat");
    		// this is not allowed:
    		// meal.seasonal_fruit = String::from("blueberies");
        println!("I'd like {} toast please", meal.toast);
    }
    ```
    
    Here, the users are only allowed to choose the `toast` but not the `seasonal_fruit` (which is decided by the `back_of_house` module).
    

#### Quiz

1. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    pub mod a {
      pub mod b {
        pub fn f() { println!("b1"); }
        pub mod c {
          pub fn f() { println!("c1"); }
        }
      }
      pub fn entry() { super::b::c::f(); }
    }
    pub mod b {
      pub fn f() { println!("b2"); }
      pub mod c {
        pub fn f() { println!("c2"); }
      }
    }
    fn main() {
      crate::a::entry();
    }
    ```
    
    - Ans
        
        The program compiles with `c2` as the output
        
        **Context**: `entry` uses the path `super::b::c::f`. `entry` is within the module `a`, so `super` refers to the parent module of `a`, which is the root crate. Then the child `b` of the root is the outermost module `b`, whose child `c` contains a function `f` that prints `"c2"`.
        
2. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    pub mod point {
      #[derive(Debug)]
      pub struct Point(i32, i32);
      impl Point {
        pub fn origin() -> Self { Point(0, 0) }
      }
    }
    fn main() {
      let mut p = point::Point::origin();
      p.0 += 1;
      println!("{p:?}");
    }
    ```
    
    - Ans
        
        This program **does not** compile.
        
        **Context**: While the `Point` structure and the `origin` static method are both public, the field `i32` is not marked as pub. Therefore directly accessing `p.0` outside of the `point` module is not allowed. This program would compile if line 3 were changed to `pub struct Point(pub i32, pub i32)`.
        

## Bringing Paths into Scope with `use`

- It might be inconvenient to write out the full paths every time we want to access a function or struct or enum declared within a module
- For the sake of convenience, Rust provides the `use` keyword that brings these into scope so that we don’t have to write out the full path
- Example:
    
    ```rust
    use crate::front_of_house::hosting; // brings hosting into scope
    
    fn eat_at_restaurant() {
    	hosting::add_to_waitlist();
    }
    ```
    
- This is akin to using a symbolic link to another path
- Just like for absolute and relative paths, the thing being brought into scope must be publicly accessible.
- Also note that the `use` statement only brings the thing into the scope that it is defined in.

### Creating Idiomatic `use` Paths

1. When bringing in a function into the scope, it is more idiomatic to **not** bring the function itself into scope but its parent module
    
    ```rust
    // non-idiomatic
    use crate::front_of_house::hosting::add_to_waitlist();
    ...
    add_to_waitlist();
    
    // idiomatic
    use crate::front_of_house::hosting;
    ...
    hosting::add_to_waitlist();
    ```
    
    The advantage with this approach is that we know exactly where a particular function is coming from without having to write out the full path
    
2. When bringing a Struct or an Enum or other items into the scope, it is more idiomatic to bring it directly into the scope:
    
    ```rust
    // non-idiomatic
    use std::collection;
    ...
    let mut map = collection::HashMap::new();
    
    // idiomatic
    use std::collections::HashMap;
    ...
    let mut map = HashMap::new();
    ```
    
    There is no strong reason to this idiom except that it has become a convention among Rustaceans
    
3. When bringing in two items with the same name into the scope, which Rust does not allow, it is more idiomatic/necessary to bring the parent module into the scope:
    
    ```rust
    use std::fmt;
    use std::io;
    
    fn function1() -> fmt::Result {
    	...
    }
    
    fn function2() -> io::Result {
    	...
    }
    ```
    
    Another solution in this case would be to use the `as` keyword that allows us to create an *alias*
    
    ```rust
    use std::fmt::Result;
    use st::io::Result as IoResult;
    ```
    

### Re-exporting Names with `pub use`

- When we bring a path into the current scope with `use`, whatever is brought into the scope is still private to the outside users of our API
- To enable other users to access the item as if it was defined in the scope (from the perspective of the outside user), we can prepend the `pub` keyword.
    
    ```rust
    pub use crate::front_of_house::hosting;
    ```
    
- Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain. For example, customers visiting a restaurant do not think about the “front” and “back” of the restaurant so exporting the `hosting` module makes sense

### Using External Packages

- Pulling in external packages into the scope of our program has the same syntatic structure as with our own local packages, via the `use` keyword
- This, of course, requires that we first download the package from `[crates.io](http://crates.io)` with `cargo` by defining the dependency in the `Cargo.toml` file
- For example:
    
    ```rust
    // Cargo.toml
    [dependencies]
    rand = "0.8.5"
    ```
    
    ```rust
    use rand::Rng;
    
    fn main() {
    	let secret_number = rand::thread_rng().gen_range(1..=100);
    }
    ```
    

### Using Nested Paths to Clean Up Large `use` Lists

- We might be importing multiple items from the same package
- Defining each one on a separate line can cause an unnecessary amount of clutter
- We can instead nest the items:
    
    ```rust
    // no nesting
    use std::cmp::Ordering;
    use std::io;
    use std::io;
    use std::io::Write;
    
    // nesting
    use std::{cmp::Ordering, io};
    use std::{self, Write};
    ```
    

### Using the Glob Operator

- We can also bring everything inside a package or module into scope using the glob operator: `*`:
    
    ```rust
    use std::collections::*;
    ```
    
    The above brings all public items defined in the `std::collections` module into the current scope.
    
- This syntax makes it difficult to determine where an item is coming from so must be used sparingly.
- A common use-case for this syntax is in tests when we need to test a module without worrying about the specific path.

### Quiz

1. Which of the following statements best describes the function of the `use` keyword?
    - Ans
        
        `use` reduces the verbosity of referring to items in the used path
        
2. Consider this module and `use` statement:
    
    ```rust
    pub mod parent {
      pub fn a() {}
      fn b() {}
      pub mod child {
        pub fn c() {}
      }
    }
    fn main() {
      use parent::{*, child as alias};
      // ...
    }
    ```
    
    Inside `main`, what is the total number of paths that can refer to `a`, `b`, or `c` (not including those that use `self`, `super`, or `crate`)? Write your answer as a digit such as 0 or 1. For example, if the only two valid paths were `a` and `parent::b`, then the answer would be 2.
    
    - Ans
        
        5
        
        **Context**: There are two paths to `a`: `parent::a` and `a`. There are no paths to `b`, because it is private. There are three paths to `c`: `parent::child::c`, `child::c`, `alias::c`.
        

## Separting Modules into Separate Files

- So far, each of the modules have been defined in the same file as where the module is being used
- A better alternative is to separate modules into separate files
- For example, the `front_of_house` module can be defined in a separate file called `src/front_of_house.rs` which contains all the contents of the `front_of_house` module that was defined inline and only leave the declaration in the original file:
    
    ```rust
    // src/front_of_house.rs
    pub mod hosting {
        pub fn add_to_waitlist() {}
    
        fn seat_at_table() {}
    }
    
    pub mod serving {
        fn take_order() {}
    
        pub fn serve_order() {}
    
        fn take_payment() {}
    }
    ```
    
    ```rust
    // src/lib.rs
    mod front_of_house;
    ```
    
- A `mod` declaration only need exist in one part of your module tree. Once the compiler knows where the code associated with a module exists, other parts of your code can reference the module through any paths as mentioned above.
- In short, `mod` is not an `import` operation akin to other programming languages.
- Extracting the `hosting` module from the above is a bit tricky as it belongs to the `front_of_house` module
    - We remove the contents of the `hosting` definition to its own file `src/front_of_house/hosting.rs` leaving behind only the `mod` declaration in `src/front_of_house.rs`
    - Alternatively, we can move `front_of_house` module in `src/front_of_house/[mod.rs](http://mod.rs)` file as well. This is an older style that is not as idiomatic and can be confusing because with this approach, there can be too many files in the package with the `mod.rs` name.

### Quiz

1. Imagine a Rust package with the following directory structure:
    
    ```rust
    foobar
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        ├── engine.rs
        └── engine/
            └── analysis.rs
    ```
    
    The contents of each file are:
    
    ```rust
    // engine/analysis.rs
    pub fn run() {}
    // engine.rs
    mod analysis;
    pub use analysis::*;
    // lib.rs
    pub mod engine;
    ```
    
    Say that another Rust developer is using the `foobar` library crate in a separate package, and they want to call the `run` function. What is the path they would write?
    
    - Ans
        
        `foobar::engine::run`
        
        **Context**: The module tree generated by this directory structure is as follows:
        
        ```
        foobar
        └── engine
            └── run
        ```
        
        Therefore the path to `run` is `foobar::engine::run`.
