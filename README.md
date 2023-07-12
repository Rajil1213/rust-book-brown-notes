# Common Collections

## Introduction

- Rust provides some very useful data structures called `collections`
- These can contain multiple values but unlike `array` and `tuple` types, the data these collections point to are stored in the *heap*.
- This means that the amount of data does not need to be known at compile time and can grow or shrink as the program runs.
- This chapter covers:
    - `vector` that allows sotring a variable number of values next to each other
    - `string` that is a collection of characters
    - `hash map` that allows associating a vlaue with a particular key; a particular implemenation of a general data structure called a `map`.
- More on these is available in [the documentation](https://doc.rust-lang.org/std/collections/index.html).

## Storing Lists of Values with Vectors

- Vectors, represented by `Vec<T>` in Rust allow you to store more than one value in a single data structure that puts all the values next to each other in memory
- They can only store values of the same type

### Creating a New Vector

- We use the `Vec::new` function:
    
    ```rust
    let v: Vec<i32> = Vec::new();
    ```
    
    Here, we are also using type annotation because we are not adding any values and as such, Rust cannot infer the type without the type annotation. A vector in Rust is defined via *generics* that can hold *any* type. The type within the angular brackets represents the concrete type to use.
    
- Another way to create a vector is to initialize it with values:
    
    ```rust
    let v = vec![1, 2, 3];
    ```
    
    Here, the type is inferred to be `i32`, the default. We are using the `vec!` macro to create the vector.
    

### Updating a Vector

- To add elements to a vector, we use the `push` method:
    
    ```rust
    let mut v = Vec::new();
    
    v.push(5);
    v.push(7);
    ```
    
    Here, since we are trying to mutate the vector by pushing to it, we need to add the `mut` keyword in the declaration. We do not need to annotate this vector since we’re pushing `i32` values in the subsequent lines so Rust is smart enough to infer the type of the vector as `i32`.
    

### Reading a Vector

- There are two ways to reference a value stored in a vector: indexing or using the `get` method
- Examples:
    
    ```rust
    let v = vec![1, 2, 3];
    
    let third: &i32 = &v[2]; // indexing
    println!("The third element is {third}");
    
    let third: Option<&i32> = v.get(2);
    match third {
    	Some(third) => println!("The third value is {third}"),
    	None => println!("There is no third element"),
    }
    ```
    
- Here, we use the `&` and `[]` operators to reference a value at a particular index. Alternatively, we use the `get` method to pass in the index as an argument.
- The main difference between these two methods is how we want to handle out-of-bounds indices.
- When we use a direct index with `&` and `[]`, and we try to access a reference that is not in the vector, we get a panic. For example, if we try to index the 4th element in the above vector, we get:
    
    ```rust
    Compiling demo v0.1.0 (/Users/rajil/courses/rust/rust-book-brown-notes/demo)
        Finished dev [unoptimized + debuginfo] target(s) in 0.68s
         Running `target/debug/demo`
    thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 3', src/vectors.rs:4:18
    ```
    
    The `get` operation is, however, safe because we are handling the case where there is no 4th element via the `None` variant.
    
- The direct index is best used when you want your program to crash if there’s an attempt to acces an element past the end of the vector.
- The `get` method is useful when we want to handle the error gracefully as for example, if the user has a typo while entering an index, it is better to prompt the user again or provide some feedback rather than to just crash the program altogether.
- When a program has a valid reference, the borrow checker enforces the ownership and borrowing rules to ensure this reference and any other references to the contents of the vector remain valid.
- This program fails to compile:
    
    ```rust
    let mut v = vec![1, 2, 3];
    
    let borrowing_index_one = &v[0];
    v.push(7); // immutable borrow
    
    println!("The first element is {borrowing_index_one}");
    ```
    
    The compile throws the following error message:
    
    ```rust
    error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
      --> src/vectors.rs:16:5
       |
    15 |     let borrowing_index_one = &v[0];
       |                                - immutable borrow occurs here
    16 |     v.push(7); // immutable borrow
       |     ^^^^^^^^^ mutable borrow occurs here
    17 |
    18 |     println!("The first element is {borrowing_index_one}");
       |                                    --------------------- immutable borrow later used here
    ```
    
- At first glance, it seems that the above code is safe, why would appending to the end of an array affect the reference to a particular index in the array?
- The error has to do with how vectors are implemented underneath:
    - since elements inside a vector are stored contiguously the entire might have to be “moved” to a location in the heap so that all the elements in it can fit
    - if the elements do have to be moved, all references to elements of the vector become *pointless* and so Rust conservatively decides to invalidate these references.

### Iterating over the Values in a Vector

- To access each element in the vector, we iterate through it instad of using indices to access one element in it at a time.
- For this, we use a `for` loop to obtain immutable references to each element in the vector:
    
    ```rust
    for v_ref in &v {
        println!("v_ref = {v_ref}");
    }
    ```
    
- We can also obtain a mutable reference:
    
    ```rust
    // iterating through a vector, mutably
    for v_ref in &mut v {
        *v_ref += 10;
        println!("v_ref = {v_ref}");
    }
    ```
    
- As in other cases, we need to derefence (`*`) the reference (`&`) to the vector before being able to access the value or mutate it.

### Quiz

1. Which call to this `find_until` function will cause a runtime panic?
    
    ```rust
    fn find_until(v: &Vec<i32>, n: i32, til: usize) -> Option<usize> {
      for i in 0 .. til {
        if v[i] == n {
          return Some(i);
        }
      }
      return None;
    }
    ```
    
    - Ans
        
        `find_until(&vec![1, 2, 3], 4, 4)`
        
        This call will try to reference the vector at index 4 that does not exist (out-of-bounds)
        
2. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    fn main() {
      let mut v = Vec::new();
      let s = String::from("Hello ");
      v.push(s);
      v[0].push_str("world");
      println!("original: {}", s);
      println!("new: {}", v[0]);
    }
    ```
    
    - Ans
        
        This DOES NOT compile as `v.push(s)` moves its argument, namely `s` so that `println!("original: {}", s);` cannot use `s`.
        
3. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    fn main() {
      let v = vec![String::from("Hello ")];
      let mut s = v[0];
      s.push_str("world");
      println!("{s}");
    }
    ```
    
    - Ans
        
        This DOES NOT compile. The operationi `s = v[0]` requires moving the `v[0]` to `s` but since `s` (of type `String`) is not copyable, this operation fails. More on this behavior later.
        

### Safely Using Iterators

- An important detail for the present, is that iterators contain a pointer to the data within the vector
- We can see how iterators work by desugaring the `for...in` syntax with the `iter` and `next` method calls on the `Vec` and `Iterator` types respectively.
    
    ```rust
    // unsugarring for...in
    let mut v = vec![1, 2, 3, 4];
    let mut iter: std::slice::Iter<'_, i32> = v.iter();
    let n1: &i32 = iter.next().unwrap(); // get the next value and unwrap the option to get the Some value
    let n2: &i32 = iter.next().unwrap();
    let n3: &i32 = iter.next().unwrap();
    let n4: &i32 = iter.next().unwrap();
    println!("{},\n{},\n{},\n{},", n1, n2, n3, n4);
    
    let end: Option<&i32> = iter.next(); // this is an option type
    
    match end {
        Some(val) => println!("End value = {val}"),
        None => println!("Reached the end of the vector"),
    }
    ```
    
    The above produces the following output:
    
    ```rust
    1,
    2,
    3,
    4,
    Reached the end of the vector
    ```
    
- In the above code, `iter` holds the pointer to the first element in the vector
- Each calll to `next()` updates the pointer so that it points to the next element in the vector if it exists (hence, the `Option<&i32>` return type)
- Also, note that the `v.iter()` method removes the `W` permission from the `vector` that is it borrows the vector `v` and renders it immutable. The following would fail:
    
    ```rust
    let mut iter: std::slice::Iter<'_, i32> = v.iter();
    v.push(5); // this fails
    let n1: &i32 = iter.next().unwrap(); // get the next value and unwrap the option to get the Some value
    ```
    
    with the error message:
    
    ```rust
    error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
      --> src/vectors.rs:31:5
       |
    30 |     let mut iter: std::slice::Iter<'_, i32> = v.iter();
       |                                               -------- immutable borrow occurs here
    31 |     v.push(5);
       |     ^^^^^^^^^ mutable borrow occurs here
    32 |     let n1: &i32 = iter.next().unwrap(); // get the next value and unwrap the option to get the Some value
       |                    ----------- immutable borrow later used here
    ```
    
- Another way to iterate is to iterate over the indices rather than the values:
    
    ```rust
    let mut v: Vec<i32> = vec![1, 2];
    let mut iter: Range<usize> = 0..v.len();
    
    let i1: usizs = iter.next().unwrap();
    let n1: &i32 = &v[i1];
    ```
    

### Using an `enum` to Store Multiple Types

- Vectors can only store values that are of the same type
- However, there are cases when we need to store values of multiple types in the same vector.
- Since the variants of an enum are defined under the same [ `enum` ] type, we can use enums to represent multiple values:
    
    ```rust
    enum SpreadsheetCell {
    	Int(i32),
    	Float(f64),
    	Text(String),
    }
    
    let row = vec![
    	SpreadsheetCell::Int(3),
    	SpreadsheetCell::Text(String::from("blue")),
    	SpreadsheetCell::Float(10.12),
    ];
    ```
    
    The above “trick” helps us get around the type constraint imposed by a vector.
    
- Rust needs to know at compile time, how much meory on the heap wil be needed to store each element hence the need for Rust to know exactly what types exist in a vector.
- Using an `enum` plus a `match` expression means that Rust will ensure at compile time that every possible case is handled.
- If you do not know the exhaustive set of types a program will get at runtime to store in a vector, the `enum` technique won’t work and you’ll have to use a trait object (to be discussed later).

### Dropping a Vector

- Dropping a vector (as in when it goes out of scope) causes all of its contents to also be dropped, meaning the values it stores will be cleaned up.
- Rust ensures that any references to the values in a vector are only valid when the vector itself is also valid.

### Quiz

1. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    fn main() {
      let mut v = vec![1, 2, 3];
      for i in &mut v {
        v.push(*i);
      }
      println!("{} {} {}", v[3], v[4], v[5]);
    }
    ```
    
    - Ans
        
        This DOES NOT compile because the `push` operation needs to take ownership that is not allowed since it has already been mutably borrowed by the for loop
        
2. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    fn main() {
      let mut v: Vec<i32> = vec![1, 2, 3];
      let mut v2: Vec<&mut i32> = Vec::new();
      for i in &mut v {
        v2.push(i);
      }
      *v2[0] = 5;
      let a = *v2[0];
      let b = v[0];
      println!("{a} {b}");
    ```
    
    - Ans
        
        This program DOES compile with the output: `5 5`
        
        This is because `i` in the loop is a mutable borrow of the elements in `v`. Pushing `i` into `v2` causes `v2` to take ownership of all elements in `v`. So mutating `v2` causes elements of `v` to also be mutated accordingly.
        

## Strings

- Strings can be particularly difficult to handle for new programmers because of
    - Rust’s propensity to expose possible errors
    - Strings being a more complicated data structure than many programmers give them credit for, and
    - UTF-8
- Strings are implemented as a collection of bytes, plus some methods to provide useful functionality when those bytes are interpreted as text
- This section covers:
    - operations on `String` that every collection type has such as creating, updating and reading
    - how `String` is different from other collection types namely with regards to indexing

### What is a String?

- Rust has only one string type in the core language, which is the string slice `str` that is usually seen in its borrowed from `&str`
- String slices are references to some UTF-8 encoded string  data stored elsewhere
- String literals are stored in the program’s binary and are therefore, string slices
- The `String` type provided by Rust’s standard library rather than coded into the core language, is a **growable, mutable, owned, UTF-8 encoded** string type
- When referring to strings, Rustaceans may refer to both the string literal `&str` or the `String` type

### Creating

- Many of the operations available for a `Vec<T>` are also available with `String` as well because `String` is implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions and capabilities
- To create a new string:
    
    ```rust
    let mut a = String::new();
    ```
    
    This creates a new empty string `a` which we can then load data into.
    
- We might also have some initial string literal that we want to create our `String` type from. In this case, we can use the `to_string()` method:
    
    ```rust
    let data = "initial contents";
    let s = data.to_string();
    // or
    let s = "initial contents".to_string();
    ```
    
- We can also create the `String::from` function to create a `String` from a string literal:
    
    ```rust
    let s = String::from("initial contents");
    ```
    
- Which API to choose is a matter of style and readability.
- Since this type can store any UTF-8 encoded value, these are perfectly valid:
    
    ```rust
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    ```
    

### Updating

- A `String` can grow in size and its contents can change (just like `Vec<T>`) if you push more data into it
- We can also use the `+` operator or the `format!` macro
- Using `push_str`:
    
    ```rust
    let mut s1 = String::from("hello ");
    let s2 = "world";
    s1.push_str(s2);
    
    println!("After appending {s2}, we get {s1}");
    ```
    
- The `push_str` accepts a string slice because we do not necessarily want to take ownership of the parameter when calling this method on a string. If this did not happen, we would not be able to print `s2` after pushing it onto `s1`.
- We can also add a single caharacter to a string with the `push` method:
    
    ```rust
    let exclaim = '!';
    hello.push(exclaim);
    
    println!("hello + {world} + {exclaim} = {hello}");
    ```
    
- We can also use the `+` operator to concatenate two strings:
    
    ```rust
    let hello: String = String::from("hello, ");
    let world: String = String::from("world");
    let hello_world: String = hello + &world;
    println!("hello + {world} = {hello_world}");
    ```
    
    The caveat to this approach is that this operation calls the add function defined as:
    
    ```rust
    fn add(self, &str) -> String { ... }
    ```
    
    This function does not take ownership of the parameter passed to it (the right operand to `+`) but it does take ownership of `self` (the left operand). This means that after the `hello + &world` operation, `hello` becomes unusable!
    
    Taking ownership is necessary because the String may need to be reallocated.
    
    Also note that we can use `&world` i.e, `&String` when the expected type is `&str` because Rust can perform *deref coercion* to infer `&str` from `&String`. In actuality, `&world` is converted to `&world[..]` (string slice).
    
    So, all in all, the `s1 + &s2` operation:
    
    - takes ownership of `s1`
    - appends a copy of the contents of `s2` to `s1`,
    - and then, returns back ownership of `s1`.
- Sometimes, we might need to add multiple strings and using the above methods will make reading the code very difficult. In this case, the `format!` macro can come in handy:
    
    ```rust
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let tic_tac_toe = format!("{tic}-{tac}-{toe}");
    
    println!("{tic}-{tac}-{toe} = {tic_tac_toe}");
    ```
    
    The `format!` macro does not take ownership of the string parameters and instead uses references.
    

### Quiz

1. What is the difference between using `a + b` and `a.push_str(b)` to concatenate two strings?
    - Ans
        
        `+` consumes ownership of `a`, while `push_str` does not
        
2. What is the maximum number of times a heap allocation could occur in this program? Write your answer in digits, e.g. 0 or 1.
    
    ```rust
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    ```
    
    - Ans
        
        7 (each `String::from()` call plus each `+` operation)
        

### Indexing

- In many other programming languages, accessing individual characters in a String by referencing them by their index is valid
- In Rust, this results in an error:
    
    ```rust
    error[E0277]: the type `String` cannot be indexed by `{integer}`
      --> src/strings.rs:26:24
       |
    26 |     let first_letter = s[0];
       |                        ^^^^ `String` cannot be indexed by `{integer}`
       |
       = help: the trait `Index<{integer}>` is not implemented for `String`
       = help: the following other types implement trait `Index<Idx>`:
                 <String as Index<RangeFrom<usize>>>
                 <String as Index<RangeFull>>
                 <String as Index<RangeInclusive<usize>>>
                 <String as Index<RangeTo<usize>>>
                 <String as Index<RangeToInclusive<usize>>>
                 <String as Index<std::ops::Range<usize>>>
    ```
    
- But why?

#### Internal Representation

- A String is a wrapper around a `Vec<T>`
- Take the following string:
    
    ```rust
    let hello = String::from("hello");
    ```
    
    When indexing to the first character in this string, you would expect it to return `h`.  This is perfectly okay since `hello`, is a 5-byte string with each character occupying 1 byte when encoded in UTF-8.
    
- Now, take this string:
    
    ```rust
    let world = String::from("Здравствуйте");
    ```
    
    The fist character `З` is not the arabic numeral `3` but instead the capital Cyrillic letter `Ze`. When asked the total size of the string, we would expect 12 but Rust instead, says 24 — the number of bytes it takes to encode `Здравствуйте` in UTF-8 because each scalar value in that string takes 2 bytes of storage. Therefore, indexing into strings’ bytes will not awlays correlate to a valid Unicode scalar value.
    
- Rust itself stores a String as a vector of bytes. When indexing into a String, a user does not usually want the byte value and instead wants the character
- The approach that Rust takes to avoid returning an unexpected value and causing bugs that might not be discovered immediately is to not allow indexing and prevent misunderstanding early in the development process.

#### Bytes, Scalar Values and Graphene Clusters

- From Rust’s perspective, there are three ways to look at strings encoded with UTF-8:
    - as bytes
    - as scalar values
    - as grapheme clusters
- If you look at the Devanagari word: `नमस्ते`, it is stored as vector of `u8` values that looks like:
    
    ```rust
    [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
    224, 165, 135]
    ```
    
    These 18 bytes is how computers ultimately store the data.
    
    In terms of Unicode scalar values, i.e, Rust’s `char` type, we see what these bytes look like:
    
    ```rust
    ['न', 'म', 'स', '्', 'त', 'े']
    ```
    
    There are just 6 `char` values here but the fourth and sixth are not letters: they’re diacritics that don’t make sense on their own
    
    Finally, if we look at the Grapheme clusters, we’d get what a person would call the four letters that make up the word:
    
    ```rust
    ["न", "म", "स्", "ते"]
    ```
    
    Another reason for indexing is not allowed in Rust is that indexing is expected to be an `O(1)` operation but it isn’t possible to guarantee performance with a `String` because Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were!
    

#### Slicing Strings

- We can also slice a UTF-8 encoded string
- However, this slice should not lie in an UTF-8 encoding boundary (a partial `char`)
- For example:
    
    ```rust
    let namaste = "नमस्ते";
    let s = &namaste[1..3]
    ```
    
    The above when run panics with:
    
    ```rust
    thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'न' (bytes 0..3) of `नमस्ते`', src/strings.rs:38:14
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    ```
    
- You should be very careful when creating such ranges as it can cause your program to crash

#### Interating over Strings

- The best way to iterate over the indices of a `String` is to be explicit about whether we want `bytes` or `chars`
- Example:
    
    ```rust
    let namaste = "नमस्ते";
    let namaste_bytes = namaste.as_bytes();
    println!("Bytes in {namaste} = {namaste_bytes:?}");
    
    let namaste_as_chars = namaste.chars();
    let mut namaste_scalars: Vec<char> = Vec::new();
    for scalar in namaste_as_chars {
        namaste_scalars.push(scalar);
    }
    println!("Scalars (chars) in {namaste} = {:?}", namaste_scalars);
    ```
    
- Getting grapheme clusters, however, is a bit complex and is not provided by the standard library. Crates like [unicode_segementation](https://docs.rs/crate/unicode-segmentation/latest) can help achiveve this task.

### Quiz

1. Which statement is the best explanation for why Rust does not allow string indexing?
    - Ans
        
        Indexing strings is ambiguous because strings represent several granularities of sequenced data
        
2. Which statement best describes the difference between the types of a string slice `&str` and a byte slice `&[u8]`?
    - Ans
        
        `&str` points to bytes that can always be interpreted as UTF-8, whereas `&[u8]` can be any byte sequence
        

## Hash Maps

- A hash map `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V` using a *hashing function* which determines who it places these keys and values into memory
- Similar to `dict` in Python, `Hash` in Java, `map` in Golang
- These are useful when you want to lok up data not by using an index but by using a key that can be of any type
- They also store data on the heap

### Creating a Hash Map

- We can use the `new` and `insert` functions
    
    ```rust
    use std::collections::HashMap;
    
    let mut scores = HashMap::new();
    
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 20);
    
    println!("scores: {scores:?}");
    ```
    
    The above outputs:
    
    ```rust
    scores: {"blue": 10, "red": 20}
    ```
    
- Note that we first need to `use` the `HashMap` from the collections portion of the standard library
- Of the three collections discussed so far, this is the one used the least and is therefore not brought into scope automatically in the prelude.
- They also have less support from the standard libary — there is no macro to construct them.
- All keys of a Hash Map must be of the same type

### Accessing Values in a Hash Map

- We can get a value from a hash map by providing its key to the `get` method:
    
    ```rust
    let team_name = String::from("red");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Team {team_name} scored {score}");
    ```
    
    The above produces the following output:
    
    ```rust
    Team red scored 20
    ```
    
- The `get` method returns an `Option<&V>` which evaluates to `None` if there is no `value` associated with the specified `key`
- The above program handles the `Option` by calling `copied` to get an `Option<i32>` rather than an `Option<&i32>`
- The `unwrap_or` method then, unwraps the `Some` value if it exists or returns the default specified as the parameter if the `Option` evaluates to `None`
- We can also iterate over the key-value pairs in a hash map in a similar manner with a `for` loop:
    
    ```rust
    println!("Full score: ");
    for (key, value) in scores {
        println!("Team {key} scored {value}");
    }
    ```
    

### Hash Maps and Ownership

- When adding a key-value pair to a hash map, values that implement the `Copy` trait are copied over and those values that don’t have their ownership transferred to the `Hash Map`
- For example in the example below:
    
    ```rust
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    
    println!("map: {map:?}");
    println!("{field_name} has the value {field_value}");
    ```
    
    The above produces the following compiler error:
    
    ```rust
    error[E0382]: borrow of moved value: `field_name`
      --> src/hash_maps.rs:28:15
       |
    20 |     let field_name = String::from("favorite color");
       |         ---------- move occurs because `field_name` has type `String`, which does not implement the `Copy` trait
    ...
    24 |     map.insert(field_name, field_value);
       |                ---------- value moved here
    ...
    28 |     println!("{field_name} has the value {field_value}");
       |               ^^^^^^^^^^^^ value borrowed here after move
       |
       = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
    help: consider cloning the value if the performance cost is acceptable
       |
    24 |     map.insert(field_name.clone(), field_value);
       |                          ++++++++
    
    error[E0382]: borrow of moved value: `field_value`
      --> src/hash_maps.rs:27:42
       |
    21 |     let field_value = String::from("blue");
       |         ----------- move occurs because `field_value` has type `String`, which does not implement the `Copy` trait
    ...
    24 |     map.insert(field_name, field_value);
       |                            ----------- value moved here
    ...
    27 |     println!("{field_name} has the value {field_value}");
       |                                          ^^^^^^^^^^^^^ value borrowed here after move
       |
       = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
    help: consider cloning the value if the performance cost is acceptable
       |
    24 |     map.insert(field_name, field_value.clone());
       |                                       ++++++++
    ```
    
    Here, references to `field_name` and `field_value` are no longer valid since they have both been moved to the hash map.
    
- One way to get around this is to use references, but these references have to be valid so long as the map is valid (this requires discussion of lifetimes).

### Updating a Hash Map

- Each key in a hash map should be unique
- When we update a value for a key, any existing value will be overwritten!
- Updating can be achieved by simply inserting a key value pair with `insert` method as before
- It might make sense to not update a key if it is already present, thus preventing overwriting of existing values.
- In this case, we can use the special API called `entry`
- For example:
    
    ```rust
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    
    println!("Before adding new key Yellow to {scores:?}");
    scores.entry(String::from("Yellow")).or_insert(50);
    println!("After adding new key key Yellow: {scores:?}");
    
    println!("Trying to re-add existing key Blue to {scores:?}");
    scores.entry(String::from("Blue")).or_insert(20);
    println!("After trying to re-add existing key Blue: {scores:?}");
    ```
    
    The output of the above program is:
    
    ```rust
    Before adding new key Yellow to {"Blue": 10}
    After adding new key key Yellow: {"Blue": 10, "Yellow": 50}
    Trying to re-add existing key Blue to {"Blue": 10, "Yellow": 50}
    After trying to re-add existing key Blue: {"Blue": 10, "Yellow": 50}
    ```
    
    As we can see that when the key does not exist (in this case, `Yellow`), the key is inserted. And if the key does exist (in this case, `Blue`), the value for this key is not overwritten.
    
- The `entry` method returns the mutable reference to the value for the corresponding `Entry` key if that key exists. Otherwise, it inserts the parameter as the new value for this key and returns the mutable reference to the new value.
- The `or_insert` inserts the default value passed as parameter if the `entry` does not exist and does nothing otherwise
- This API works better with the borrow checker and can be clearer than writing out the logic.
- A common use case is to update a value based on the previous value such as when counting the unique letters in a word/sentence:
    
    ```rust
    fn char_counter(s: &str) -> HashMap<char, i32> {
        let mut char_count = HashMap::new();
        for letter in s.chars() {
            let count = char_count.entry(letter).or_insert(0);
            *count += 1;
        }
    
        return char_count;
    }
    ```
    

### Hashing Function

- By default, `HashMap` uses a hashing function called `SipHash` tha can provide resistance to [Denial-of-Service(DoS) attacks involving hash tables](https://en.wikipedia.org/wiki/SipHash).
- While this is not the fastest algorithm out there, it does provide a better trade-off in terms of security
- Rust also allows us to use a different `hasher`. A hasher is a type in Rust that implements the `BuildHasher` trait (more on traits later).

### Quiz

1. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    use std::collections::HashMap;
    fn main() {
      let mut h = HashMap::new();
      h.insert("k1", 0);
      let v1 = &h["k1"];
      h.insert("k2", 1);
      let v2 = &h["k2"];
      println!("{} {}", v1, v2);
    }
    ```
    
    - Ans
        
        This program DOES NOT compile
        
        **Context**: `h` cannot be mutated (`h.insert("k2", 1)`) while an immutable reference (`v1`) to it is live.
        
2. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    use std::collections::HashMap;
    fn main() {
      let mut h: HashMap<char, Vec<usize>> = HashMap::new();
      for (i, c) in "hello!".chars().enumerate() {
        h.entry(c).or_insert(Vec::new()).push(i);
      }
      let mut sum = 0;
      for i in h.get(&'l').unwrap() {
        sum += *i;
      }
      println!("{}", sum);
    }
    ```
    
    - Ans
        
        This program DOES compile with the output `5`.
        
        The program maps a character in the string `hello` with a vector storing the indices at which the character occurs.
        
        We are then summing up the indices of the occurrences of the letter `l` (i.e, index `2` and `3`). Hence, the output is `5`.
