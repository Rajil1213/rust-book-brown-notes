# Fearless Concurrency

## Introduction

- Concurrent Programming — different parts of a program execute independently
- Parallel Programming — differents parts of a program execute at the same time
- Historically, programming in these contexts has been difficult and error prone
- Over time, the Rust team discovered that the ownership and type systems are a powerful set of tools to help manage memory safety *and* concurrency problems!
- By leveraging ownership and type checking, many concurrency errors are compile-time errors in Rust and not runtime errors — this aspect of Rust has been named *Fearless Concurrency*
- For simplicity’s sake, *concurrency* in the book (and in these notes) are used in place of *concurrency and/or parallelism* for convenience
- Higher-level languages provide only a subset of possible solutions for handling concurrent operations (such as message-passing concurrency in Go, Erlang) which is justified because higher-level languages promise benefits from giving up some control to gain abstractions
- Low-level languages, on the other hand, are expected to provide the best performance in any given situation and have fewer abstractions over the hardware
- This chapter delves into the following concurrent features of Rust:
    - Creating threads to run multiple pieces of code at the same time
    - *Message-passing* concurrency, where channels send messages between threads
    - *Shared-state* concurrency, where multiple threads have access to some piece of data
    - The `Sync` and `Send` traits, which extend Rust’s concurrency guarantees to user-defined types as well as types provided by the standard library

## Using Threads to Run Code Simultaneously

- In most OSes, an executed program’s code is run in a *process*, and the OS will manage multiple processes at once
- Within a program, we can have multiple parts that run simultaneously — the features that run these independent parts are called *threads.* For example, a web server can have multiple theads to respond to more than 1 request at a time
- This can improve performance but also introduce complexity
- We also lose the inherent guarantee about the order in which parts of our code on different parts will run which can lead to:
    - Race conditions, where threads are accessing data or resources in an inconsistent order
    - Deadlocks, where two threads are waiting for each other, preventing both threads rom continuing
    - Bugs that happen only in certain situations and are hard to reproduce and fix reliably
- Rust attempts to mitigate the negative effects of using threads, but programming in a multithreaded context still takes careful thought and requires a code strucure that is different from that of porgrams running in a single thread
- Rust standard library uses a 1:1 model of thread implementation — a program usses one operating system thread per one language thread (there are other libraries with different models of threading that make different tradeoffs)

### Creating a New Thread with `spawn`

- To create a new thread, we call `thread::spawn` function and pass it a closure containing the code we want to run in the new thread
- Example:
    
    ```rust
    use std::{thread, time::Duration};
    
    fn main() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1));
            }
        });
    
        for i in 1..5 {
            println!("hi number {i} from main thread!");
            thread::sleep(Duration::from_millis(1));
        }
    }
    ```
    
- The output of the above program:
    
    ```rust
    hi number 1 from main thread!
    hi number 1 from the spawned thread!
    hi number 2 from main thread!
    hi number 2 from the spawned thread!
    hi number 3 from main thread!
    hi number 3 from the spawned thread!
    hi number 4 from main thread!
    hi number 4 from the spawned thread!
    hi number 5 from the spawned thread!
    ```
    
- As we can see the loop in the closure of the spawned thread does not complete. This is because all spawned threads are shut down (regardless of whether they have finished runnning) when the main thread of a Rust program terminates
- The threads will probably take turns but that isn’t guaranteed and depends on how the OS schedules these threads

### Waiting for All Threads to Finish Using `join` Handles

- There is not guarantee as to when a spawned thread is run or if it will complete its execution or if the thread will even run at all
- We can fix this problem by saving the return value of `thread::spawn` in a variable — an instance of `JoinHandle`
- A `JoinHandle` is an owned value that, when we call the `join` method on it, will wait for its thread to finish:
    
    ```rust
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 1..5 {
        println!("hi number {i} from main thread!");
        thread::sleep(Duration::from_millis(2));
    }
    
    handle.join().unwrap();
    ```
    
    The call to `join` at the end of the main thread ensures that the spawned thread does not end prematurely. The code inside the closure can still fail/panic, so we need to handle that case in some way. Here, `unwrap()` is being used to “handle” this case
    
- The output now shows all the numbers in the iteration inside the closure:
    
    ```rust
    hi number 1 from main thread!
    hi number 1 from the spawned thread!
    hi number 2 from the spawned thread!
    hi number 3 from the spawned thread!
    hi number 2 from main thread!
    hi number 4 from the spawned thread!
    hi number 3 from main thread!
    hi number 5 from the spawned thread!
    hi number 6 from the spawned thread!
    hi number 4 from main thread!
    hi number 7 from the spawned thread!
    hi number 8 from the spawned thread!
    hi number 9 from the spawned thread!
    ```
    
- If we move the call to `join()` to the beginning of the `for` loop in the main thread, the main thread blocks until the spawned thread completes its execution
- Details such as where `join` is called, can affect whether or not your threads run at the same time

### Using `move` Closures with Threads

- We often use the `move` keyword with closures passed to `thread::spawn` because the closure will take ownership of the values it uses from the environment, thus transferring ownership from one thread to another
- This section focusses on the interaction between `move` and `thread::spawn`
- Since we are not passing any data while creating the main thread, we need to the closure to capture its environment if we want to use values from the main thread but ths does not work as expected:
    
    ```rust
    let v = vec![1..5];
    
    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });
    ```
    
    This throws a compile error with:
    
    ```rust
    error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
     --> src/threads.rs:6:32
      |
    6 |     let handle = thread::spawn(|| {
      |                                ^^ may outlive borrowed value `v`
    7 |         println!("Here's a vector: {:?}", v);
      |                                           - `v` is borrowed here
      |
    note: function requires argument type to outlive `'static`
     --> src/threads.rs:6:18
      |
    6 |       let handle = thread::spawn(|| {
      |  __________________^
    7 | |         println!("Here's a vector: {:?}", v);
    8 | |     });
      | |______^
    help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
      |
    6 |     let handle = thread::spawn(move || {
      |                                ++++
    ```
    
- Here, Rust infers how to capture `v`, and because `println!` only needs a reference to `v`, the closure tries to borrow `v`
- However, Rust can’t tell how long the spawned thread will run, so it doesn’t know if the reference to `v` will always be valid
- This will be evident if we `drop(v)` right after spawning the thread in which case, the closure might hold on to a reference that no longer exists, while it’s waiting for execution
- The fix is to `move` the captured environment into the closure:
    
    ```rust
    thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    })
    ```
    
    If we were to call `drop(v)` after this line, we will get another compiler error:
    
    ```rust
    error[E0382]: use of moved value: `v`
      --> src/threads.rs:10:10
       |
    4  |     let v = vec![1..5];
       |         - move occurs because `v` has type `std::vec::Vec<std::ops::Range<i32>>`, which does not implement the `Copy` trait
    5  |
    6  |     let handle = thread::spawn(move || {
       |                                ------- value moved into closure here
    7  |         println!("Here's a vector: {:?}", v);
       |                                           - variable moved due to use in closure
    ...
    10 |     drop(v);
       |          ^ value used here after move
    ```
    
    This error message arises not from anything specific to theads but from the ownership rules inherent to Rust
    
    Rust is telling us that we are trying to `drop` a value that has already been moved elsewhere. This is similar to the case where instead of the thread, there was a function into which the value could have been moved (if the value wasn’t `Copy`)
    
### Quiz

1. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    use std::thread;
    fn main() {
        let mut n = 1;
        let t = thread::spawn(move || {
            n = n + 1;
            thread::spawn(move || {
                n = n + 1;
            })
        });
        n = n + 1;
        t.join().unwrap().join().unwrap();
        println!("{n}");
    }
    ```
    
    - Ans

        This program **DOES** compile and produces the output: `2`

        **Context**: The `move` keyword causes `n` to be copied into the closure, so the assignments `n = n + 1` within `thread::spawn` have no effect on the outer `n`.

2. Consider this example from the text where a vector is improperly captured by a thread:
    
    ```rust
    use std::thread;
    fn main() {
        let v = vec![1, 2, 3];
        let handle = thread::spawn(|| {
            println!("Here's a vector: {:?}", v);
        });
        handle.join().unwrap();
    }
    ```
    
    The Rust compiler includes this diagnostic:
    
    ```text
    note: function requires argument type to outlive `'static`
       --> src/main.rs:15:18
       |
    15 |       let handle = thread::spawn(|| {
       |  __________________^
    16 | |         println!("Here's a vector: {:?}", v);
    17 | |     });
       | |______^
    help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
       |
    15 |     let handle = thread::spawn(move || {
       |                                ++++
    ```
    
    Recall that `'static` is the lifetime of references that are valid for the entire program's duration.
    
    Which of the following best describes the note "function requires argument type to outlive `'static`"?
    
    - Ans

        Rust doesn't know how long a thread will run, so the thread's captures must live forever

        **Context**: If a closure captures a reference to a value that lived less than `'static`, it's possible that the thread would live longer than the value and violate memory safety. Note that `[thread::scope](https://doc.rust-lang.org/std/thread/fn.scope.html)` can be used to allow threads to capture non-static references by statically limiting the lifetime of the thread.

## Using Message Passing To Transfer Data Between Threads

- In this approach, threads or actors communicate by sending each other messages containing data

> Do no communicate by sharing memory; instead share memory by communicating

- [Go Language Documentation](https://golang.org/doc/effective_go.html#concurrency)
>
- To accomplish message-sending concurrency, Rust’s standard library provides an implemetation of `channels` — a general programming concept by which data is sent from one thread to another
- A channel has two halves:  a transmitter and a receiver
- Think of channles like a stream or river, and the data as rubber ducks:
    - Transmitter half is the upstream location where you put rubber ducks into the river
    - Receiver half is the downstream location where where the rubber duck ends up
- One part of your code calls methods on the transmitter with the data you want to send, and another part checks the receiving end for arriving messages
- A channel is considered *closed* if either the transmitter or the receiver half is dropped
- Let’s create a simple program that sends over values via a channel:
    
    ```rust
    use std::sync::mpsc;
    
    fn main() {
      let (tx, rx) = mpsc::channel();
      
    }
    ```
    
    The above code does not compile since Rust needs to know the type of the data being passed through the channel
    
- Here, a channel is created via `mpsc` which stands for `mutliple producers, single consumer` — which means this channel can have multiple sending ends but a single receiving end that consumes those values
- For starters, we’ll use a single producer
- The `mpsc::channel()` function returns a tuple of the `Sender<T>` and `Receiver<T>` that are being destructured into two variables `tx` and `rx` with the `let` expression
- Now, let’s transmit data:
    
    ```rust
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    ```
    
    Here
    
    - We’re using `thread::spawn` to create a new thread and then, using `move` to move `tx` into the closure so the spawned thread owns `tx`.
    - The spawned thread needs to own the transmitter to be able to send message through the channel.
    - The `send` method on the transmitter that takes the value we want to send.
    - The `send` method returns a `Result<T, E>` type, so if the receiver has already been dropped and there’s nowhere to send a value, the send operation will return an error.
    - `unwrap` is being used to handle the error case but in a real scenario, the error case should be handled properly
- Now, let’s receive the value:
    
    ```rust
    let received = rx.recv().unwrap();
    println!("Got: {received}");
    ```
    
    Here,
    
    - `Receiver<String>` (`rx`) has two methods: `recv` and `try_recv`.
    - `recv` blocks the main threads execution and wait until a value is sent down the channel
        - Once a value is sent, `recv` will return it in a `Result<T, E>`
        - When the transmitter closes, the error variant is returned
    - `try_recv` doesn’t block but will instead return a `Resut<T, E>` immediately
        - an `Ok` variant will be returned if a message is available, and an `Err` variant if it is not at the moment
        - this is useful when the receiver may be doing other things while also waiting for messages
- The above program outputs:
    
    ```rust
    Got: hi
    ```
    
### Channels and Ownership Transferance

- Ownership allows us to write safe, concurrent code
- To demonstrate ownership rules in the context of concurrent programs, let’s modify the closure in the previous program slightly to pring the string `val` *after* it’s sent via the channel:
    
    ```rust
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("Val is {val}"); // <- new line of code
    });
    ```
    
- This program throws the following compiler error:
    
    ```rust
    error[E0382]: borrow of moved value: `val`
     --> src/channels.rs:9:26
      |
    7 |         let val = String::from("hi");
      |             --- move occurs because `val` has type `std::string::String`, which does not implement the `Copy` trait
    8 |         tx.send(val).unwrap();
      |                 --- value moved here
    9 |         println!("Val is {val}");
      |                          ^^^^^ value borrowed here after move
      |
      = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
    ```
    
- Allowing this code is indeed a bad idea because once the value has been sent to another thread, that thread could modify or drop it before we try to use the value again
- The `send` function, therefore, takes ownership of the data being sent and any violation of the susbsequent ownership rules causes a compile-time error

### Sending Multiple Values and Seeing the Receiver Waiting

- Our previous code works fine but doesn’t demonstrate the communication between the two separate threads
- Let’s modify our code so that the transmitting thread sends multiple messages at an interval which are then received by the receiving thread:
    
    ```rust
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // can't do this now:
        // println!("Val is {val}");
    });
    
    for received in rx {
        println!("Got: {received}");
    }
    ```
    
    Here:
    
    - Instead of sending a single `val`, we are sending over a bunch of `val`s in the spawned thread
    - Between each sent message, the thread sleeps for 1 second
    - In the main thread, we are treating `rx` as an iterator — for each value received, we are printing it, the iteration ends when the channel closes (and the iterator returns `None`)
- When we run the above code, we get:
    
    ```rust
    Got: hi
    Got: from
    Got: the
    Got: thread
    ```
    
    with a 1 second delay between the printing of each of these lines
    
### Creating Multiple Producers by Cloning the Transmistter

- Let’s use `mpsc` to create multiple threads that all send values to the same receiver by cloning the same receiver
    
    ```rust
    let (tx, rx) = mpsc::channel();
    let tx_clone = tx.clone();
    
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // can't do this now:
        // println!("Val is {val}");
    });
    
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx_clone.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // can't do this now:
        // println!("Val is {val}");
    });
    
    for received in rx {
        println!("Got: {received}");
    }
    ```
    
    Here,
    
    - We create a clone of our original receiver using the `clone` method on the `tx`
    - Then, we spawn another thread that uses this clone to send some additional messages over the same channel
- The above code outputs all the messages over both the transmitter threads but the order in which the messages are received on the receiver is non-deterministic:
    
    ```rust
    Got: hi
    Got: more
    Got: messages
    Got: from
    Got: for
    Got: the
    Got: you
    Got: thread
    ```
    
- If we use a different delay for either of the transmitter threads, we’d receive the messages from the faster thread sooner

### Quiz

1. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    use std::{sync::mpsc, thread};
    enum ClientMessage { Incr, Get, Quit }
    enum ServerMessage { Get(usize) }
    fn main() {
        let (server_tx, client_rx) = mpsc::channel();
        let (client_tx, server_rx) = mpsc::channel();
        let server = thread::spawn(move || {
            let mut n = 0;
            loop {
                match server_rx.recv().unwrap() {
                    ClientMessage::Quit => break,
                    ClientMessage::Incr => n += 1,
                    ClientMessage::Get => server_tx.send(ServerMessage::Get(n)).unwrap()
                }
            }
        });
        for msg in [ClientMessage::Incr, ClientMessage::Get, ClientMessage::Quit] {
            client_tx.send(msg).unwrap();
        }
        if let ServerMessage::Get(n) = client_rx.recv().unwrap() {
            println!("{}", n)
        }
        server.join().unwrap();
    }
    ```
    
    - Ans

        The program **DOES** compile and produces the output: `1`

        Here, `server_tx` communicates with `client_rx` and `client_tx` communicates with `server_rx` (two-way communication).

        The client sends three variants of messages: increment, get and exit that the server receives. When the server receives the `increment` message, it increments an internal counter. When the serve receives the `quit` message, it stops transmitting (closing the channel) and when it receives `get`, it sends over the current value of the internal counter to the client which then prints it out.

        In the above example, the client is sending `incr`, `get` and `quit` in order. So, server increments the internal counter by 1, sends it to the client and closes. The client prints thr received value, i.e., `1`

2. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    use std::{sync::mpsc, thread};
    fn main() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let s = String::from("Hello world");
            tx.send(s.clone()).unwrap();
            tx.send(s.len()).unwrap();
        });
        let s = rx.recv().unwrap();
        let n = rx.recv().unwrap();
        println!("{s} {n}");
    }
    ```
    
    - Ans

        This program **DOES NOT** compile

        **Context**: Channels can only send values of a single type, so `tx.send(s.len())` is a type error. If you want to send values of multiple types, you can use either an enum or the `[Any](https://doc.rust-lang.org/std/any/trait.Any.html)` trait.

## Shared-State Concurrency

- Message passing is not the only way of handling concurrency — another method would be to access the same shared data
- Recall that the Go language documentation tell us to “not communicate by sharing memory”
- But what does communicating by sharing memory look like? And why would message-passing enthusiasts caustion against it?
- In a way, channels in any programming language are similar to single ownership — you should no longer use a value once you send it over a channel
- Shared-memory concurrency is like multiple ownership — which as we have seen is possible but tricky!

### Using Mutexes to Allow Access to Data from One Thread at a Time

- *Mutex* is an abbreviation for *mutual exclusion*, as in, a mutex allows only one thread to access some data at any given time
- To access the data in a mutex, the thread must first signal that it wants access by asking to acquire the mutex’s lock.
- The lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data — so a mutex *guards* the data it holds via the *locking* system
- Mutexes have a reputation for being difficult to use because you have to remember two rules:
    - You must attempt to acquire the lock before using the data
    - When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock
- Management of mutexes can be incredibly tricky to get right, which is why so many people are enthuisiastic about channels — however, Rust’s type and ownership system makes it incredibly difficult to get it wrong!

### The API fo `Mutex<T>`

- Let’s start by using mutex in a single-threaded context
    
    ```rust
    use std::sync::Mutex;
    
    pub fn test() {
        let m = Mutex::new(5);
    
        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }
    
        println!("m = {m:?}");
    }
    ```
    
    Here,
    
    - we create a new mutex with the `new` function (like in many other types)
    - to access the data, we use the `lock` method to acquire the lock — this is a blocking operation so the current thread can do nothing but wait for it’s turn to acquire the lock
    - this call to `lock` will fail if another thread which is currently holding the lock panics — in which case, no one would be able to acquire he lock. So, we `unwrap()` and panic if we encounter such a situation
    - after the lock has been acquired, we can treat the return value, named `num` in this case, as a mutable reference to the data inside
    - the type system ensures that we `lock` before accessing the value within
    
    The above program outputs:
    
    ```rust
    m = Mutex { data: 6, poisoned: false, .. }
    ```
    
- `Mutex<T>` is a smart pointer — more accurately, the call to `.lock` *returns* a smart pointer called `MutexGuard`, wrapped in a `LockResult` that we handled with the call to `unwrap`:
    
    ```rust
    pub fn lock(&self) -> LockResult<MutexGuard<'_, T>>
    ```
    
- `MutexGuard` implements the `Deref` trait to point at our inner data
- `MutexGuard` also implements the `Drop` trait to release the lock automatically when a `MutexGuard` goes out of scope (here, this happens at the end of the inner loop). This makes sure that we don’t accidentally forget releasing the lock thereby blocking other threads from acquiring it

### Sharing a `Mutex<T>` Between Multiple Threads

- As an example, let’s spin up 10 threads and have them each increment a counter value by 1, so that the counter goes from 0 to 10
- Here is the naive approach:
    
    ```rust
    let counter = Mutex::new(0);
    let mut handles = vec![];
    
    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
    
            *num += 1
        });
    
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Result: {}", *counter.lock().unwrap());
    ```
    
    Here,
    
    - we create `counter` variable that holds an `i32` inside a `Mutex<T>`
    - we then create 10 threads that each try to increment `counter` after acquiring the lock on the mutex
    - each of these theads are added to a vec
    - we then iterate over this vector and call `join` on each of them so that the our main thread does not terminate before each thread has finished execution
    - finally, our main thread acquires the lock to read the value of the `counter` *after* all threads have finished their execution
- However, the above code results in a compiler error:
    
    ```rust
    error[E0382]: use of moved value: `counter`
     --> src/mutexes.rs:8:36
      |
    4 |     let counter = Mutex::new(0);
      |         ------- move occurs because `counter` has type `std::sync::Mutex<i32>`, which does not implement the `Copy` trait
    ...
    8 |         let handle = thread::spawn(move || {
      |                                    ^^^^^^^ value moved into closure here, in previous iteration of loop
    9 |             let mut num = counter.lock().unwrap();
      |                           ------- use occurs due to use in closure
    ```
    
    This error message tells us that by using `move`, we have moved the `mutex` itself. In this case, the first created handle gets ownership of the mutex and susbequent loops cannot!
    
### Multiple Ownerships with Multiple Threads

- To allow multiple owners, we are going to wrap the mutex in an `Rc<T>`:
    
    ```rust
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
    
            *num += 1
        });
    
        handles.push(handle);
    }
    ```
    
    However, this does not work either and throws the following compiler error:
    
    ```rust
    error[E0277]: `std::rc::Rc<std::sync::Mutex<i32>>` cannot be sent between threads safely
       --> src/mutexes.rs:9:36
        |
    9   |           let handle = thread::spawn(move || {
        |                        ------------- ^------
        |                        |             |
        |  ______________________|_____________within this `[closure@src/mutexes.rs:9:36: 9:43]`
        | |                      |
        | |                      required by a bound introduced by this call
    10  | |             let mut num = counter.lock().unwrap();
    11  | |
    12  | |             *num += 1
    13  | |         });
        | |_________^ `std::rc::Rc<std::sync::Mutex<i32>>` cannot be sent between threads safely
        |
        = help: within `[closure@src/mutexes.rs:9:36: 9:43]`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<std::sync::Mutex<i32>>`
    note: required because it's used within this closure
       --> src/mutexes.rs:9:36
        |
    9   |         let handle = thread::spawn(move || {
        |                                    ^^^^^^^
    note: required by a bound in `std::thread::spawn`
       --> ~/lib/rustlib/src/rust/library/std/src/thread/mod.rs:683:8
        |
    680 | pub fn spawn<F, T>(f: F) -> JoinHandle<T>
        |        ----- required by a bound in this function
    ...
    683 |     F: Send + 'static,
        |        ^^^^ required by this bound in `spawn`
    ```
    
    Yikes!
    
    The important part of this error message is:
    
    > `std::rc::Rc<std::sync::Mutex<i32>>` cannot be sent between threads safely
    >
    
    The compiler also tells us why:
    
    > the trait `std::marker::Send` is not implemented for `std::rc::Rc<std::sync::Mutex<i32>>`
    >
    
    `Send` is one of the traits that ensures the types we use with threads are indeed meant for use in concurrent situations
    
- Unfortunately, `Rc<T>` is not safe to share across threads (as mentioned in the chapter on Smart Pointers)
- When `Rc<T>` manages the refference count, it adds to the count for each call to `clone` and subtracts from the count when each clone is dropped
- But it doesn’t use any concurrency primitives to make sure that changes to the count can’t be interrupted by another thread — this can lead to wrong counts and bugs that could in turn lead to memory leaks or a value being dropped before we’re done with it
- We need a type that allows multiple ownership like `Rc<T>` but can work in concurrent situations

### Atomic Reference Counting with `Arc<T>`

- This is where `Rc<T>` comes in!
- The `a` stands for `atomic`
- Atomics are an additional kind of concurrency primitive (which won’t be covered here — see `[std::sync::atomic](https://doc.rust-lang.org/std/sync/atomic/index.html)`)
- At this point, suffice it to say that atomics work like primitive types but are safe to share across threads
- You might now ask why not all types in the standard library are atomics and why standard library types aren’t implemented to use `Arc<T>` by default
- The answer is that thread-safety comes with a performance penalty that you only want to pay when you need to
- Let’s fix our code by using `Arc` instead of `Rc`:
    
    ```rust
    use std::{
        sync::{Arc, Mutex},
        thread,
    };
    
    pub fn test() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
    
        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
    
                *num += 1
            });
    
            handles.push(handle);
        }
    
        for handle in handles {
            handle.join().unwrap();
        }
    
        println!("Result: {}", *counter.lock().unwrap());
    }
    ```
    
    The above program outputs:
    
    ```rust
    Result: 10
    ```
    
- While the example itself is quite trivial, these are in essence the building blocks of more complex concurrent problems — for example, sharing a complex calculation between threads and having each one update the overall result with their contribution

### Similarities between `RefCell<T>/Rc<T>` and `Mutex<T>/Arc<T>`

- In the above examples, we were able to mutate the value inside the Mutex even though it was immutable
- This is because `Mutex` provides interior mutability, as the `Cell` family does
- Just like we used `RefCell<T>` to wrap and mutate contents inside an `Rc<T>`, we use `Mutex<T>` to mutate contents inside an `Arc<T>`
- Another detail to note is that Rust cna’t protect us from all kinds of logic errors when you use `Mutex<T>`
- Just like using `Rc<T>` comes with the possibility of having reference cycles, `Mutex<T>` comes with the possibility of *deadlocks*
- This happens when an operation needs to lock two resources and two threads havfe each acquired one of the locks, causing them to wait for each other forever

### Quiz

1. In some concurrency APIs, a mutex is separate from the data it guards. For example, imagine a hypothetical Mutex API like this:
    
    ```rust
    let mut data = Vec::new();
    let mx: Mutex = Mutex::new();
    {
        let _guard = mx.lock();
        data.push(0);
    }
    ```
    
    Which of the following best describes why Rust uses `Mutex<T>` instead of just `Mutex`?
    
    - Ans

        To prevent accessing a mutex's data without locking the mutex

2. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    
    ```rust
    use std::{sync::Arc, thread};
    fn main() {
        let s = String::from("Hello world");
        let a = Arc::new(&s);
        let a2 = Arc::clone(&a);
        let t = thread::spawn(move || a2.len());
        let len = t.join().unwrap();
        println!("{} {}", a, len);
    }
    ```
    
    - Ans

        This program **does not** compile.

        **Context**: An `Arc` is still not safe to use if its data contains a reference, since the reference could (in theory) be invalidated before all threads containing the `Arc` finish executing.

## Extensible Concurrency with the `Sync` and `Send` Traits

- The Rust Language as *very* few concurrency features — almost every feature we’ve talked about here so far in this chapter has been part of the standard library, but not the language
- We can also write our own concurrency or use those written by others
- However, there are two concurrency features that are embedded in the language: the `std::marker` traits: `Sync` and `Send`

### Allowing Transferance of Ownership Between Threads with `Send`

- The `Send` marker trait indicates that ownership of values of the type implementing `Send` can be transferred bewteen threads
- Almost every Rust type is `Send`, but there are some exceptions,including `Rc<T>`: this cannot be `Send` because if you cloned `Rc<T>` value and tried to transfer ownership to the clone to another thread, both threads might update the reference count at the same time
- For this reason, `Rc<T>` is implemented to be used only in single-threaded contexts
- Rust’s type system ensures that you can never pass an `Rc<T>` across threads unsafely
- Any composite type composed entirely of `Send` types is automatically marked as `Send` as well
- Almost all primitive types are `Send`, aside from raw pointers (discussed in later chapters)

### Allowing Access from Multiple Threads with `Sync`

- The `Sync` marker trait indicates that it is safe for the type implementing `Sync` to be referenced from multiple threads
- In other words, any type `T` is `Sync` if `&T` (an immutable reference to `T`) is `Send`, meaning the reference can be sent safely to another thread
- Similar to `Send`, primitive types are `Sync`, and types composed entirely of types that are `Sync` are also `Sync`
- `Sync` is the most similar concept in Rust to the colloquial meaning of the phrase `thread-safe` i.e, that a particular piece of data can be safely used by multiple concurrent threads
- The reason for `Send` and `Sync` to be separately defined is that types do not necessarily have to be both
- For example:
    - The smart pointer `Rc<T>` is neigther `Send` nor `Sync`
    - The `RefCell<T>` type and family of releated `Cell<T>` types are `Send` (if `T: Send`) but they are not `Sync`. A `RefCell` can be sent across a thread boundary, but not accessed concurrently because the implementation of borrow checking that `RefCell<T>` does at runtime is not thread-safe
    - The smart pointer `Mutex<T>` is `Send` and `Sync`, and can be used to share access with multiple threads
    - The type `MutexGuard<'a, T>` that is returned by `Mutex::lock` is `Sync` (if `T: Sync`) but not `Send`. It is specifically not `Send` because [some platforms mandate that mutexes are unlocked by the same thread that locked them](https://github.com/rust-lang/rust/issues/23465#issuecomment-82730326).

### Implementing `Send` and `Sync` Manually is Unsafe

- Because types that are made up of `Send` and `Sync` traits are automatically `Send` and `Sync`, we don’t have to implement those traits manually
- As marker traits, they don’t even have any methods to implement — they’re just useful for enforcing invariants related to concurrency
- Manually implementing these traits involves implementing unsafe Rust code
- Building new concurrent types not made up of `Sync` and `Send` parts require careful thought to uphold the safety guarantees

### Quiz

1. Imagine you are designing an API for a database connection like this:
    
    ```rust
    struct DbConnection { /* ... */ }
    impl DbConnection {
        fn query(&self) -> DbResult {
            /* ... */
        }
    }
    ```
    
    Your database does not support concurrent queries from the same connection. Which of the following marker traits should `DbConnection` implement?
    
    - Ans

        `Send`

        **Context**: It is fine to send a `DbConnection` between threads, so `DbConnection` should implement `Send`. But it would not be valid e.g. for an `Arc<Database>` to be shared such that multiple threads concurrently called `db.query()`, so `DbConnection` should not implement `Sync`.
