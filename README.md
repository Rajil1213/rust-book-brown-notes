# Final Project: Multithreaded Web Server

## Introduction

- For this final project, we’ll build a multi-threaded web server that simply says “Hello”
- Here’s the plan:
  - Learn a bit about TCP and HTTP
  - Listen for TCP connections on a socket
  - Parse a small number of HTTP requests
  - Create a proper HTTP response
  - Improve the throughput of the server with a thread pool
- Note that the following implementation is not the best way to build a web server in Rust
- More complete, production-ready crates are available on [crates.io](http://crates.io)
- This chapter explores how to write a basic HTTP server with a thread pool, from scratch so as to learn and the explore the general ideas and techniques behind the crates you might use in the future

## Building a Single-Threaded Web Server

### Before we begin…

- We’ll start with a single-threaded web server first but before that, we must know the basics of the protocols involved
- The two main protocols involved in web servers are *Hypertext Transfer Protocol (HTTP)* and *Transmission Control Protocol (TCP)*
- Both of these are *request-response* protocols — meaning a *client* initiates requests and a *server* listens to the requests and provides a response to the client
- The content of these requests and responses are defined by the protocols
- TCP is the lower-level protocol that describes the details of how information gets from one server to another but doesn’t specify what that information is
- HTTP builds on top of TCP by defining the contents of the requests and responses (although we can use it on top of some other protocols as well)
- We’ll be working with the raw bytes of TCP and HTTP requests and responses

### Listening to the TCP Connection

- The standard library provides `std::net` module that lets us listen to TCP connections
- Let’s first create our project with `cargo`:

    ```bash
    cargo new hello-web
    ```

- Now, change into the resulting `result-web` directory (`cd hello-web`) and in `src/main.rs`, enter the code below:

    ```bash
    use std::net::TcpListener;
    
    fn main() {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
        for stream in listener.incoming() {
            let _stream = stream.unwrap();
    
            println!("connection established");
        }
    }
    ```

- If we run the above code, we should be `nc` or `curl` or open a web browser at `127.0.0.1:7878` and see `connection established` show on the console

    ```bash
    ╰─λ cargo r     
       Compiling hello-web v0.1.0 (/hello-web)
        Finished dev [unoptimized + debuginfo] target(s) in 2.31s
         Running `target/debug/hello-web`
    connection established
    connection established
    connection established
    connection established
    connection established
    ```

- Using `TcpListener`, we can listen for TCP connections at the address `127.0.0.1:7878` (the port is chosen because it is `rust` typed on a telephone ☎️)
- The `bind` function in this scenario works like the `new` function in that it will return a new `TcpListener` instance — named this way since in networking lingo, connection to a port to listen to is known as “binding to a port”.
- The `bind` function returns a `Result<T, E>` indicating that the binding may fail such as if binding to a port requires administrative privileges, the port is already in use.
- For our purposes, we’ll ignore handling these errors more gracefully and just use `unwrap`.
- The `incoming` method on the `TcpListener` returns an iterator that gives us a sequence of streams — more specifically streams of type `TcpStream`
- A single *stream* represents an open connection between the client and the server
- A *connection* is the name for the full request and response process in which a client connects to the server, the server generates a response, and the server closes the connection
- As such, we will read from the `TcpStream` to see what the client sent and then write our responses to the stream to send data back to the client
- Overall, the `for` loop will process each connection in turn, and produce a series of streams for us to handle.
- For now, our handling of the stream consists of calling `unwrap` to terminate our program if the stream has any errors; there there are none, the program prints a message (we’ll add more functionality to the success case later)
- The reason we might receive errors from the `incoming` method when a client connects to the server is that we are not actually iterating over connections but instead, *connection attempts*
- The connection might not be successful for a number of reasons — many of them OS-specific (such as a limit on the number of open connections)
- When the `stream` goes out of scope after the end of the loop, the connection is closed as part of the `drop` implementation on `TcpStream`

### Reading the Request

- Now, we’ll implement the functionality to read the request from the browser
- For a better separation of concerns, we’ll add this functionality as a separate function that takes the `stream`:

    ```bash
    use std::{
        io::{prelude::*, BufReader},
        net::{TcpListener, TcpStream},
    };
    
    fn main() {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
        for stream in listener.incoming() {
            let stream = stream.unwrap();
    
            handle_connection(stream);
        }
    }
    
    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
    
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
    
        println!("Request: {:#?}", http_request);
    }
    ```

- We bring `std::io::prelude` and `std::io::BufReader` into scope to get access to types and traits that let us read from and write to the stream
- Then, instead of printing out that we have a connection, we call the `handle_connection` function and pass the `stream` to it
- Inside the function, we create a new `BufReader` instance that wraps a mutable reference to the stream
- `BufReader` adds buffering by managing calls to the `std::io::Read` trait methods for us
- We then create a variable named `http_request` to collect the lines of the request the browser sends to our server. We indicate that we want to collect these lines in a vector by adding the `Vec<_>` type annotation
- `BufReader` implements the `std::io::BufRead` trait, which provides the `lines` method that returns an iterator of `Result<String, std::io::Error>` by splitting the stream of data whenever it sees a newline byte
- Then, we unwrap the `result` on each item on the iterator to get a `String`
- This will error if the `String` is not a valid UTF-8 encoded sequence of bytes
- The browser signals the end of an HTTP request by sending two newline characters in a row, so to get one request from the stream, we’re printing them out using pretty debug formatting so we can take a look at the instructions the web browser is sending to our server.
- When we run the code and open the address on the browser, we should see something like the following:

    ```bash
    Request: [
        "GET / HTTP/1.1",
        "Host: 127.0.0.1:7878",
        "Connection: keep-alive",
        "Cache-Control: max-age=0",
        "sec-ch-ua: \"Chromium\";v=\"118\", \"Brave\";v=\"118\", \"Not=A?Brand\";v=\"99\"",
        "sec-ch-ua-mobile: ?0",
        "sec-ch-ua-platform: \"macOS\"",
        "Upgrade-Insecure-Requests: 1",
        "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gec
    ko) Chrome/118.0.0.0 Safari/537.36",
        "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,
    */*;q=0.8",
        "Sec-GPC: 1",
        "Sec-Fetch-Site: none",
        "Sec-Fetch-Mode: navigate",
        "Sec-Fetch-User: ?1",
        "Sec-Fetch-Dest: document",
        "Accept-Encoding: gzip, deflate, br",
        "Accept-Language: en-US,en;q=0.9",
    ]
    ```

    The output may be a bit different depending upon the browser and the OS

#### A Closer Look at an HTTP Request

- HTTP is a text-based protocol, and a request takes this format:

    ```bash
    Method Request-URI HTTP-Version CRLF
    headers CRLF
    message-body
    ```

- The first line is the *request line* that holds information about what the client is requesting
- The first part of the request line indicates the *method* being used, such as `GET` or `POSt`, which describes how the client is making this request. Here, our client is making a `GET` requests, which means it is asking for information
- The Next part of the request is the `/` which indicates the *Uniform Request Identifier* (URI) the client is requesting
- It is almost the same as a *Uniform Resource Locator* (URL) but the spec mentions URI so that is what we refer to as
- The last part is the HTTP version the client uses, and then the request line ends in a CRLF sequence (CRLF stands for *carriage returns* and *line feed*) which are terms from the typewriter days!)
- The CRLF sequence can also be written as `\r\n`, where `\r` is a carriage return, and `\n` is a line feed
- This sequence separates the request line from the rest of the request data
- After the request line, the remaining lines starting from `Host:` onward are headers, `GET` requests have no body

### Writing a Response

- HTTP responses have the following format:

    ```bash
    HTTP-Version Status-Code Reason-Phrase CRLF
    headers CRLF
    message-body
    ```

- The first line is the *status line* that contains the HTTP version used in the response, a numeric status code that summarizes the result of the request, and reason phrase that provides a text description of the status code
- After the CRLF are any headers, another CRLF sequence, and the rest is the response body
- An example of an OK response is: `HTTP/1.1 200 OK\r\n\r\n` where the HTTP version is `1.1`, the status code is `200`, the reason phrase is `OK` and the headers and body are both empty
- Now, let’s try to return this response from our `handle_connection` function:

    ```bash
    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
    
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        println!("Request: {:#?}", http_request);
    
        let response = "HTTP/1.1 200 OK\r\n\r\n";
    
        stream.write_all(response.as_bytes()).unwrap();
    }
    ```

- Here, we simply form the response as a string and write it to the stream as `bytes`
- The `write_all` method takes an `&[u8]` and sends those bytes directly down the connection.
- This operation could fail, so we use `unwrap` on any error result as before

### Returning Real HTML

- Let’s return a sample HTML to the client.
- For this, we first create an HTML file at the root of our repository (not the `src`!)

    ```html
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="utf-8">
        <title>Hello!</title>
      </head>
      <body>
        <h1>Hello!</h1>
        <p>Hi From Rust</p>
      </body>
    </html>
    ```

- Now, let’s return this HTML in the body of the response along with the appropriate headers:

    ```rust
    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
    
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        println!("Request: {:#?}", http_request);
    
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();
    
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
        stream.write_all(response.as_bytes()).unwrap();
    }
    ```

- All we do here is send the HTML back to the client as a string along with its length in the `Content-Length` header. The browser only reads `Content-Length`-worth of `chars` from the response body.

### Validating the Request and Selectively Responding

- In its present form, our server returns the HTML no matter what/how a client requests data form the server.
- So, let’s add validation to the server so that we only return the HTML if the client provides a `GET` request to `/` using HTTP version `1.1`

    ```rust
    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
    
        let request_line = buf_reader.lines().next().unwrap().unwrap();
    
        let mut response = "HTTP/1.1 404 NOT FOUND".to_string();
    
        if request_line == "GET / HTTP/1.1" {
            let status_line = "HTTP/1.1 200 OK";
            let contents = fs::read_to_string("hello.html").unwrap();
            let length = contents.len();
    
            response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        }
    
        stream.write_all(response.as_bytes()).unwrap();
    }
    ```

- Here, we return a `NOT FOUND` response by default but if the request line has what we want, we return the hello page as before.
- Alternatively, we can write up another HTML page that has the NOT FOUND message:

    ```rust
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="utf-8">
        <title>Hello!</title>
      </head>
      <body>
        <h1>Hello!</h1>
        <p>Oops!</p>
        <p>Sorry, I don't know what you're looking for.</p>
      </body>
    </html>
    ```

- Then, we can refactor out the common bits, and what we have is:

    ```rust
    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
    
        let request_line = buf_reader.lines().next().unwrap().unwrap();
    
        let (status_line, page) = if request_line == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
    
        let contents = fs::read_to_string(page).unwrap();
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
        stream.write_all(response.as_bytes()).unwrap();
    }
    ```

- And with that, we have created a single-threaded web server that can respond to web requests!

## Turning Our Single-Threaded Server into a Multi-Threaded One

- At the moment, our server processes one request at a time — meaning it won’t process a second connection until the first request is done processing
- If the server receives a request that takes a long time to be processed, subsequents requests will all be delayed as well even if these new requests themselves can be processed quickly
- We’ll need to fix this by making our server multi-threaded

### Simulating a Slow Request

- For the purposes of this demonstration, we will add a new *route* called `sleep` on our web server that sleeps for five seconds before responding back with `hello`:

    ```rust
    let (status_line, page) = match &request_line[..] {
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
            "GET /sleep HTTP/1.1" => {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", "hello.html")
            }
            _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
        };
    ```

- To match on three conditions, we are now using a `match` expression instead of `if..else`.
- We also need to match on a string slice instead of a `String` because `match` does not perform auto-referencing and dereferencing
- In the `sleep` arm, we pause the thread for 5 seconds
- If we now call the `/sleep` route and then, try to call the `/` route, the call to `/` will also be delayed!

### Improving the Throughput with a Thread Pool

- A *thread pool* is a group of spawned threads that are waiting and ready to handle a task
- When the program receives a new task, it assigns one of the threads in the pool to the task, and that thread will process the task
- The remaining threads in the pool are available to handle any other tasks that come in while the first thread is processing
- When the first thread finishes processing its task, it is returned to the pool of idle threads, ready to handle a new task
- A thread pool, thus allows us to process connections concurrently, increasing the throughput of the server
- We’ll limit the number of threads in the pool to a small number to protect ourselves from Denial of Service (DoS) attacks — if we had our program create a new thread for each request as it came in; someone making 10 million requests to our server could wreak havoc by using up all our server’s resources and grinding the processing of requests to a halt
- Requests that come in are sent to the pool for processing and the pool will maintain a queue of incoming requests
- Each of the threads will pop off a request from the queue, handle the request, and then as the queue for another one once it’s done.
- With this design, we can process up to `N` requests concurrently, where `N` is the number of threads
- There is still a chance that the `N+1`th request will be delayed if all threads are processing a long request but we have increased the number of long-running requests with this approach.
- This is one of many approaches to solve this particular problem. Alternatives include *fork/join model*, *single-threaded async I/O model*, or the *multi-threaded async I/O model*
- Before implementing the thread pool, let’s first implement the client design which should then guide us to the implementation — write the API of the code so it’s structured in the way you want to cal it; then implement the functionality within that structure rather than implementing the functionality and then designing the public API
- This is *compiler-driven development` — we’ll write the code that calls the functions we want, and then, we’ll look at errors from the compiler to determine what we should change next to get the code to work

### Before We Begin…

- First, however, we’ll look explore the technique that we’re not going to use as a starting point
- The naive approach is to just spawn a thread every time we get a request:

    ```rust
    fn main() {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
        for stream in listener.incoming() {
            let stream = stream.unwrap();
    
            thread::spawn(|| {
                handle_connection(stream);
            });
        }
    ```

- As mentioned earlier, this has a critical security flow in that if many requests come in at the same time, it will not only overwhelm our web server but the entire machine that the server is running on

### Creating a Finite Number of Threads

- We would like to create only a finite number of threads
- Our interface should look like the following:

    ```rust
    fn main() {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
        const THREAD_COUNT: usize = 4;
        let pool = ThreadPool::new(THREAD_COUNT);
    
        for stream in listener.incoming() {
            let stream = stream.unwrap();
    
            pool.execute(|| {
                handle_connection(stream);
            });
        }
    }
    ```

- This interface lets us create a new pool with `new` and then, execute arbitrary closures with `execute` on the pool (just like with `thread::spawn`
- We designate the type of the input to `new` as `usize` as it holds the number of threads, and it cannot be negative.

### Building the `ThreadPool` Using Compiler Driven Development

- The above does not compile because we don’t have anything called `ThreadPool` in our scope.
- When we run `cargo check`, we get:

    ```rust
    rustc: failed to resolve: use of undeclared type `ThreadPool`
    use of undeclared type `ThreadPool` [E0433]
    ```

- To fix this error, we need to create a struct called `ThreadPool`
- We want our implementation to be independent of the task that our web server will execute. So, we’ll create a library file `src/lib.rs` that will house this implementation:

    ```rust
    pub struct ThreadPool {}
    ```

    Then, we use it our `main` file:

    ```rust
    use hello_web::ThreadPool;
    
    // hello_web is the name of the package
    ```

- Now, when we run `cargo check`, we get:

    ```rust
    rustc: no function or associated item named `new` found for struct `hello_web::ThreadPool` in the current scope
    function or associated item not found in `ThreadPool` [E0599]
    ```

- So, we now need to implement the `new` method on our `ThreadPool` struct:

    ```rust
    pub struct ThreadPool {}
    
    impl ThreadPool {
        pub fn new(num_threads: usize) -> ThreadPool {
            ThreadPool {}
        }
    }
    ```

- Running `cargo check` again, we get the following error:

    ```rust
    rustc: no method named `execute` found for struct `hello_web::ThreadPool` in the current scope
    method not found in `ThreadPool` [E0599]
    ```

- So, we must now implement the `execute` method which should behave the same way as `thread::spawn` does
- The documentation for `spawn` has the following signature:

    ```rust
    pub fn spawn<F, T>(f: F) -> JoinHandle<T>
        where
            F: FnOnce() -> T,
            F: Send + 'static
            T: Send + 'static
    ```

- We are only concerned with `F` generic variable as it represents the trait bounds for the closure (which we are also passing to the `execute` method)
- The trait bounds for a closure can be `FnOnce`, `FnMut` or `Fn`
- Just like for `spawn`, we might also want `FnOnce` as the trait bound for our closure as the closure will process the request exactly once and because, we’ll eventually pass the closure into `spawn` itself.
- We also need `Send` because we need to transfer the closure from one thread (main) to another (spawn), and `'static` because we don’t know how long it will take for the request to be processed
- With these in mind, we can implement the `execute` method as follows:

    ```rust
    impl ThreadPool {
        pub fn new(num_threads: usize) -> ThreadPool {
            ThreadPool {}
        }
    
        pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
        }
    }
    ```

- We use the trait bound `FnOnce` with parentheses `()` because this represents a closure that takes no arguments and returns a unit type.
- Now, when we run `cargo check`, we see no error messages!
- Our code does nothing but at least the compiler is happy!

<aside>
💡 Note: A saying you might hear about languages with strict compilers, such as Haskell and Rust, is “if the code compiles, it works.” But this saying is not universally true. Our project compiles, but it does absolutely nothing! If we were building a real, complete project, this would be a good time to start writing unit tests to check that the code compiles *and* has the behavior we want.

</aside>

### Validating Thread Count in `new`

- While we have written the code so as to reject any negative values, we do not validate against zero
- So, we will add an assertion and an accompanying documentation to the `new` method:

    ```rust
    /// Create a new [`ThreadPool`].                        
    ///                                                     
    /// `num_threads` is the number of threads in the pool. 
    ///                                                     
    /// ## Panics                                            
    ///                                                     
    /// If `num_threads` is zero.                           
    pub fn new(num_threads: usize) -> ThreadPool {          
        assert!(num_threads == 0);                          
                                                            
        ThreadPool {}                                       
    }
    ```

- An alternative approach is to create a `build` method that returns a `Result` that errors if the `num_threads` is equal to zero
- Here, the decision has been made to simply panic out should the `num_threads` is zero

### Creating Space to Store the Threads

- Now that we are certain that there will always be a valid number of threads, we need to store those threads somewhere.
- But how do we even “store” a thread?
- To understand this, let’s look at the implementation of `spawn` again:

    ```rust
    pub fn spawn<F, T>(f: F) -> JoinHandle<T>
        where
            F: FnOnce() -> T,
            F: Send + 'static,
            T: Send + 'static,
    ```

- The `spawn` function returns a `JoinHandle<T>` where `T` is the type that the closure returns
- Let’s store these `JoinHandle`s in a `Vec` in our struct where `T` is the unit type `()` and see what happens:

    ```rust
    use std::thread::JoinHandle;
    
    pub struct ThreadPool {
        threads: Vec<JoinHandle<()>>,
    }
    
    impl ThreadPool {
        pub fn new(num_threads: usize) -> ThreadPool {
            assert!(num_threads == 0);
    
            let mut threads = Vec::with_capacity(num_threads);
    
            for _ in 0..num_threads {
                // create threads and store them in `threads`
            }
    
            ThreadPool { threads }
        }
    ```

- We’ve brought `JoinHandle` into scope from the standard library and added a `threads` field to our `ThreadPool` struct that holds a `Vec` of these `JoinHandle`s over the unit type
- Then, in `new`, we create a vector of capacity equal to `num_threads` and return it with the `ThreadPool` instance
- Using `with_capacity` is slightly more efficient than using `Vec::new` because we are allocating the vector only once up front, rather than during “every” push.
- The question now is what to do inside the `for` loop above — we cannot create handles unless we know what we need to execute and the only way to know it is when the `execute` method is called!

### A Worker Struct Responsible for Sending Code from the `ThreadPool` to a Thread

- We want to create threads that “wait” for the code that is expected to be run on them
- The standard library does not provide any way for us to do that — so we’ll have to implement this manually
- We’ll do this by creating a new data structure between `ThreadPool` and the threads that will managed this new behavior that we’ll call `Worker` (a common term in pooling implementations)
- The `Worker` picks up code that needs to be run and runs the code in the `Worker`'s thread (analogous to how workers wait until orders come in from customers, and then take and fulfill the orders as they arrive)
- So, instead of storing `JoinHandle`'s in our `Vec`, we’ll instead store `Worker`'s.
- Each `Worker` will store a single `JoinHandle<()>` instance
- We’ll then implement a method on Worker that will take a closure of code to run and send it to the already running thread for execution
- We’ll also give each worker an `id` so we can distinguish between the different workers in the pool when logging/debugging.
- Here is the process:
    1. Define a `Worker` struct that holds an `id` and a `JoinHandle<()>`:

        ```rust
        pub struct Worker {
            id: usize,
            thread: JoinHandle<()>,
        }
        ```

    2. Change `ThreadPool` to hold a vector of `Worker` instances (it also might make sense to rename the field to `workers` now):

        ```rust
        pub struct ThreadPool {
            workers: Vec<Worker>,
        }
        ```

    3. Define a `Worker::new` function that takes an `id` number and returns a `Worker` instance that holds the `id` and a thread spawned with an empty closure

        ```rust
        impl Worker {
            pub fn new(id: usize) -> Worker {
                Worker {
                    id,
                    thread: thread::spawn(|| {}),
                }
            }
        }
        ```

    4. In `ThreadPool::new`, use the `for` loop counter to generate an `id`, create a new `Worker` with that `id`, and store it in the vector:

        ```rust
        fn new(num_threads: usize) -> ThreadPool {                      
            assert!(num_threads == 0);                                      
                                                                            
            let mut workers = Vec::with_capacity(num_threads); 
                                                                            
            for id in 0..num_threads {                               
                workers.push(Worker::new(id));                              
            }                                                               
                                                                            
            ThreadPool { workers }                                          
        }
        ```

        This function does not need to be public as it just an implementation detail that other external entities have no business knowing

### Sending Requests to Threads via Channels

- Now, the above code compiles *but* the closures inside each worker is effectively useless as they do nothing
- We want some way for our `Worker`'s to communicate with the `ThreadPool` to receive the code to run on the thread from the queue held in the
- This is the perfect use case for channels.
- We’ll use a channel to function as a queue of jobs, and `execute` will send a job from the `ThreadPool` to the `Worker` instances, which will in turn, send the job to the thread that it spawns
- Here is the plan:
    1. The `ThreadPool` will create a channel and hold on to the sender.

        ```rust
        pub struct ThreadPool {
            workers: Vec<Worker>,
            job_sender: Sender<Job>,
        }
        
        struct Job;
        
        impl ThreadPool {
            pub fn new(num_threads: usize) -> ThreadPool {
                assert!(num_threads == 0);
        
                let (job_queue_tx, job_queue_rx) = mpsc::channel::<Job>();
        
                let mut workers = Vec::with_capacity(num_threads);
        
                for id in 0..num_threads {
                    workers.push(Worker::new(id));
                }
        
                ThreadPool {
                    workers,
                    job_sender: job_queue_tx,
                }
            }
        ```

        For now, each job is just an empty struct.
        We create a multi-producer single consumer channel as multiple web requests can add to the queue but these will be received one at a time by one of the workers.
        We also add a field `job_sender` to the `ThreadPool` so that we can hold on to the sender returned by the call to `channel`.

    2. Each worker will hold on to the receiver

        ```rust
        pub struct Worker {
            id: usize,
            thread: JoinHandle<()>,
            job_receiver: Receiver<Job>,
        }
        
        impl Worker {
            fn new(id: usize, receiver: Receiver<Job>) -> Worker {
                Worker {
                    id,
                    thread: thread::spawn(|| {}),
                    job_receiver: receiver,
                }
            }
        }
        ```

        To hold on to the receiver, the `Worker` will need a field on it. So we have created one and then, modified our definition and implementation of the `new` method as well.

        Now, we will need to modify our `new` method on the `ThreadPool` as well:

        ```rust
        pub fn new(num_threads: usize) -> ThreadPool {                                             
            assert!(num_threads == 0);                                                             
                                                                                                   
            let (job_queue_tx, job_queue_rx) = mpsc::channel::<Job>(); 
                                                                                                   
            let mut workers = Vec::with_capacity(num_threads);                        
                                                                                                   
            for id in 0..num_threads {                                                      
                workers.push(Worker::new(id, job_queue_rx));                             
            }                                                                                      
                                                                                                   
            ThreadPool {                                                                           
                workers,                                                                           
                job_sender: job_queue_tx,                                                          
            }                                                                                      
        }
        ```

        However, the above code does not pass the compiler. We get the following error:

        ```rust
        --> src/lib.rs:32:42
           |
        27 |         let (job_queue_tx, job_queue_rx) = mpsc::channel::<Job>();
           |                            ------------ move occurs because `job_queue_rx` has type `std::sync::mpsc::Receiver<Job>`, which does not implement the `Copy` trait
        ...
        31 |         for id in 0..num_threads {
           |         ------------------------ inside of this loop
        32 |             workers.push(Worker::new(id, job_queue_rx));
           |                                          ^^^^^^^^^^^^ value moved here, in previous iteration of loop
           |
        note: consider changing this parameter type in method `new` to borrow instead if owning the value isn't necessary
          --> src/lib.rs:55:33
           |
        55 |     fn new(id: usize, receiver: Receiver<Job>) -> Worker {
           |        --- in this method       ^^^^^^^^^^^^^ this parameter takes ownership of the value
        ```

        Turns out that the `Receiver` returned by `channel()` does not implement the `Copy` trait

        We can’t clone the receiver either because the channel is *multiple* producer but *single* consumer. We also don’t want to send a message multiple times to multiple consumers; we want one list of messages with multiple workers such that each messages gets processed exactly once.

        Additionally, taking a job off of the channel queue involves mutating the `job_queue_rx`, so the threads need a safe way to share and modify it; otherwise, we might get race conditions

        To get around this issue, we need to use an `Arc` reference along with `Mutex`. `Arc` will let multiple workers to own the receiver, and `Mutex` will ensure that only one worker gets a job from the receiver at a time:

        ```rust
        let receiver = Arc::new(Mutex::new(job_queue_rx)); 
        for id in 0..num_threads {                                                   
            workers.push(Worker::new(id, Arc::clone(&receiver)));           
        }
        ```

        Here, we put the receiver in an `Arc` and a `Mutex`. For each new worker, we clone the `Arc` to bump the reference count so the workers can share ownership of the receiver.

        The signature for the `Worker` will need to change accordingly as well:

        ```rust
        pub struct Worker {
            id: usize,
            thread: JoinHandle<()>,
            job_receiver: Arc<Mutex<Receiver<Job>>>,
        }
        
        impl Worker {
            fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
                Worker {
                    id,
                    thread: thread::spawn(|| {}),
                    job_receiver: receiver,
                }
            }
        }
        ```

    3. The `Job` will hold the closures we want to send down the channel. For this, we need to wrap the closure in a reference as it has no concrete implementation. It makes more sense now for `Job` to be a type:

        ```rust
        type Job = Box<dyn FnOnce() + Send + 'static>;
        ```

        The trait bounds, as before, are the trait bounds in the implementation of `thread::spawn` in the standard library

    4. The `execute` method will send the job it wants to execute through the sender:

        ```rust
        pub fn execute<F>(&self, f: F)          
        where                                   
            F: FnOnce() + Send + 'static,       
        {                                       
            let job = Box::new(f);      
            self.job_sender.send(job).unwrap(); 
        }
        ```

        Here, we first `Box` the closure to create the `Job` and then send it through the `job_sender` channel. This will error if the receiver has already been dropped (this can happen if we stop all our threads). We call `unwrap()` here because we have no way of terminating all the threads without terminating the program itself — so calling `unwrap` here is a safe operation

        With the transmitter end done, we must now work on the receiver. Our worker must continue receiving from the channel and processing it forever:

        ```rust
        pub struct Worker {
            id: usize,
            thread: JoinHandle<()>,
        }
        
        impl Worker {
            fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
                let thread = thread::spawn(move || loop {
                    let job = receiver.lock().expect("mutex poisoned").recv().unwrap();
        
                    println!("Worker {id} got a job; executing...");
                    job();
                });
        
                Worker { id, thread }
            }
        }
        ```

        Here, we create a thread that contains a closure with an infinite loop.

        Inside this closure, we are acquiring the `lock` on the `Mutex` held in the `Arc` reference. We call `unwrap()` on it (this panics if the mutex is in *poisoned* state i.e, if any of the thread panicked after acquiring the mutex and before releasing the lock).

        After we acquire the lock successfully, we should get the receiver. We call `unwrap()` on it as well to get the closure that was passed down the channel. This can error if no more messages can be received from the channel (which can happen if the Sender disconnects or it disconnects while the call is blocking, but won’t happen in our case).

        The call to `recv()` blocks. So, if there is no job on the job queue, the thread will just wait.

        The `Mutex` ensures that only one thread is trying to request a job from the queue.

        Also note that we have removed the `job_receiver` field from the definition of `Worker` because the it is now held by the closure passed to the the `thread`.

  - With all that, we should now be able to run the web server again and should see something like the following when we make requests:

    ```rust
    Finished dev [unoptimized + debuginfo] target(s) in 0.94s
         Running `target/debug/hello_web`
    Worker 0 got a job; executing...
    Worker 3 got a job; executing...
    Worker 1 got a job; executing...
    Worker 2 got a job; executing...
    Worker 3 got a job; executing...
    Worker 1 got a job; executing...
    Worker 2 got a job; executing...
    Worker 3 got a job; executing...
    Worker 0 got a job; executing...
    Worker 1 got a job; executing...
    Worker 1 got a job; executing...
    Worker 3 got a job; executing...
    Worker 0 got a job; executing...
    Worker 2 got a job; executing...
    Worker 1 got a job; executing...
    Worker 3 got a job; executing...
    ```

- If we send four concurrent `sleep` requests, any subsequent requests will still block but as mentioned earlier, we still get a better throughput overall.
- We can notice the blocking behavior by either restricting `num_threads` to 1 or by issuing four concurrent requests:

    ```bash
    for i in `seq 1 4`; do curl http://localhost:7878/sleep &; done;
    ```

    Then, issuing a normal request:

    ```bash
    curl http://localhost:7878/ ## will block until one of the above finishes
    ```

### Why not `while...let`?

- Notice that we use a `loop` to loop infinitely and *then* bind to the `job` with let.
- Why not use the following construct?

    ```bash
    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
            let thread = thread::spawn(move || {
                while let Ok(job) = receiver.lock().expect("mutex poisoned").recv() {
                    println!("Worker {id} got a job; executing...");
                    job();
                }
            });
    
            Worker { id, thread }
        }
    }
    ```

- The answer is a bit subtle:
  - There is no public `unlock()` method on the `Mutex` because the ownership of the loc is based on the lifetime of the `MutexGuard<T>` within the `LockResult<MutexGuard<T>>` that the `lock` method returns
  - At compile time, the borrow checker can then enforce the rule that a resource guarded by a mutex cannot be accessed unless we. hold the lock
  - However, this implementation can also result in the lock being held longer than intended if we aren’t mindful of the lifetime of the `MutexGuard<T>`
  - When we use a `let` binding, any temporary value used on the right hand side of the equals sign are immediately dropped when the `let` statement ends. In this case, the `Mutex` is dropped once we acquire the `job`.
  - But in the case of `while let` (and `if let` and `match`), the temporary values are not dropped until the end of the associated block
  - So, if we were to use the `while let` construct the `mutex` would be locked until the `job` finishes executing effectively blocking other threads and so, causing our multi-threaded web server to become sequential instead of concurrent!
  - For a simple reason, the following code will also execute sequentially:

    ```rust
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {                                         
        let thread = thread::spawn(move || loop {                                              
            let channel: MutexGuard<'_, Receiver<Box<dyn FnOnce() + Send + 'static>>> = receiver.lock().unwrap(); // the lock is not dropped                        
            let job = channel.recv().unwrap(); 
                                                                                                               
            println!("Worker {id} got a job; executing...");                                                   
            job();                                                                                             
        });                                                                                                    
                                                                                                               
        Worker { id, thread }                                                                                  
    }
    ```

## Graceful Shutdown and Cleanup

- When we run `cargo check` on our code, we see warnings about the `workers`, `id` and `thead` fields being unused in a direct way
- This reminds us that we’re not cleaning up anything
- We we use the less elegant `CTRL-C` method to halt the `main` thread, all other threads are stopped immediately as well, even if we’re in the middle of serving a request
- So, we’ll need to implement the `Drop` trait to call `join` on each of the threads in the pool so they can finish the requests they’re working on before closing.
- Then, we’ll implement a way to tell other threads that they should stop accepting new requests and shut down.

### Implementing the `Drop` Trait on `ThreadPool`

- When the pool is dropped, our threads should all join to make sure they finish their work.
- Here is a first attempt:

    ```rust
    impl Drop for ThreadPool {
        fn drop(&mut self) {
            for worker in self.workers {
                worker.thread.join().unwrap();
            }
        }
    }
    ```

- The above, however, does not work and we get the following error:

    ```rust
    error[E0507]: cannot move out of `self.workers` which is behind a mutable reference
       --> src/lib.rs:53:23
        |
    53  |         for worker in self.workers {
        |                       ^^^^^^^^^^^^
        |                       |
        |                       `self.workers` moved due to this implicit call to `.into_iter()`
        |                       move occurs because `self.workers` has type `Vec<Worker>`, which does not implement the `Copy` trait
        |
    note: `into_iter` takes ownership of the receiver `self`, which moves `self.workers`
       --> /Users/rajil/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/iter/traits/collect.rs:267:18
        |
    267 |     fn into_iter(self) -> Self::IntoIter;
        |                  ^^^^
    help: consider iterating over a slice of the `Vec<Worker>`'s content to avoid moving into the `for` loop
        |
    53  |         for worker in &self.workers {
        |                       +
    
    For more information about this error, try `rustc --explain E0507`.
    ```

- The error message tells us that we cannot move out of `self.workers` which is behind a shared reference.
- We can instead iterate over a mutable reference on `self.workers` with:

    ```rust
    for worker in &mut self.workers {
        ...
    }
    ```

    But then, we’ll get another error:

    ```rust
    error[E0507]: cannot move out of `worker.thread` which is behind a mutable reference
        --> src/lib.rs:54:13
         |
    54   |             worker.thread.join().unwrap();
         |             ^^^^^^^^^^^^^ ------ `worker.thread` moved due to this method call
         |             |
         |             move occurs because `worker.thread` has type `JoinHandle<()>`, which does not implement the `Copy` trait
         |
    note: `JoinHandle::<T>::join` takes ownership of the receiver `self`, which moves `worker.thread`
        --> /Users/rajil/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/mod.rs:1570:17
         |
    1570 |     pub fn join(self) -> Result<T> {
         |                 ^^^^
    
    For more information about this error, try `rustc --explain E0507`.
    ```

- Turns out, we need to own the `thread` to be able to call `join` on it!
- To solve this issue, we need to move the `thread` out of the worker.
- Since `drop` requires a mutable reference restricting it even further to take ownership of `self` is not ideal.
- A more elegant solution is for `thread` to be of type `Option<JoinHandle<()>>`. That way, we can `take` out of the `Option<T>` to get ownership:

    ```rust
    impl Drop for ThreadPool {
        fn drop(&mut self) {
            for worker in &mut self.workers {
                println!("Shutting down worker {}", worker.id);
                let thread = worker.thread.take().unwrap();
                thread.join().unwrap();
            }
        }
    }
    
    pub struct Worker {
        id: usize,
        thread: Option<JoinHandle<()>>,
    }
    
    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
            let thread = thread::spawn(move || loop {
                let job = receiver.lock().expect("mutex poisoned").recv().unwrap();
    
                println!("Worker {id} got a job; executing...");
                job();
            });
    
            Worker {
                id,
                thread: Some(thread),
            }
        }
    }
    ```

    Here, we redefine the `Worker`'s thread field to have `Option<JoinHandle<()>>` instead of just `JoinHandle<()>` and then use, `if let Some()` to `take` from it inside the `drop` implementation. If the `Option` has a `Some` value, this operation takes and leaves `None` in its place and if it’s `None`, the thread has already been cleaned up and we need not take any more action

## Signaling to the Threads to Stop Listening for Jobs

- With the changes in the previous iteration, our code compiles *but* it still does not work as expected
- Calling `join` on the thread does not actually shut the threads down because inside each `thread`'s closure, there is an infinite loop running that is constantly listening for jobs
- With our current implementation, the `main` thread will block forever waiting for the infinite loop to finish executing!
- The solution is to close the `sending` channel so that the closure knows to stop listening for more jobs
- As we create the channel in the `ThreadPool`, this should happen in its `Drop` implementation.
- However, we run into a similar problem as before, we need to `drop` the sender explicitly but that is not possible in our current implementation since we only have a mutable reference to `self` inside `drop`
- As before, we need to change our implementation so that `job_sender` field in the `ThreadPool` is an `Option` type:

    ```rust
    pub struct ThreadPool {
        workers: Vec<Worker>,
        job_sender: Option<Sender<Job>>,
    }
    
    type Job = Box<dyn FnOnce() + Send + 'static>;
    
    impl ThreadPool {
        pub fn new(num_threads: usize) -> ThreadPool {
            assert!(num_threads != 0);
    
            let (job_queue_tx, job_queue_rx) = mpsc::channel::<Job>();
    
            let mut workers = Vec::with_capacity(num_threads);
    
            let receiver = Arc::new(Mutex::new(job_queue_rx));
            for id in 0..num_threads {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }
    
            ThreadPool {
                workers,
                job_sender: Some(job_queue_tx),
            }
        }
    
        pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);
            self.job_sender
                .as_ref()
                .expect("channel closed")
                .send(job)
                .unwrap();
        }
    }
    
    impl Drop for ThreadPool {
        fn drop(&mut self) {
            drop(self.job_sender.take());
    
            for worker in &mut self.workers {
                println!("Shutting down worker {}", worker.id);
                if let Some(thread) = worker.thread.take() {
                    thread.join().unwrap();
                }
            }
        }
    }
    ```

- Dropping the `job_sender` closes the channel, which indicates no more messages (`jobs`) will be sent across it
- When this happens, all the threads that call the `recv` method on the `receiving channel` will error out
- So, we add a bit of logic inside the closure to handle this case and stop the infinite loop when the channel closes off:

    ```rust
    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
            let thread = thread::spawn(move || loop {
                // assign separately to drop and release `MutexGuard` lock
                let message = receiver.lock().expect("mutex poisoned").recv();
    
                if let Ok(job) = message {
                    println!("Worker {id} got a job; executing...");
                    job();
                } else {
                    println!("Shutting down thread for worker: {id}");
                    break;
                }
            });
            Worker {
                id,
                thread: Some(thread),
            }
        }
    }
    ```

- That’s it! Now, to see the code in action, let’s modify the `main` loop so that it automatically shuts down after serving `4` requests:

    ```rust
    fn main() {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
        const THREAD_COUNT: usize = 4;
        let pool = ThreadPool::new(THREAD_COUNT);
    
        for stream in listener.incoming().take(THREAD_COUNT) {
            let stream = stream.unwrap();
    
            pool.execute(|| {
                handle_connection(stream);
            });
        }
    
      println!("Shutting down server, gracefully!);
    }
    ```

- Once we start off the server and then spawn a few requests, we should see an output similar to the following:

    ```text
    ╰─λ cargo r
        Finished dev [unoptimized + debuginfo] target(s) in 0.00s
         Running `target/debug/hello_web`
    Worker 3 got a job; executing...
    Worker 0 got a job; executing...
    Worker 1 got a job; executing...
    Shutting down server, gracefully!
    Shutting down worker 0
    Worker 2 got a job; executing...
    Shutting down thread for worker: 2
    Shutting down thread for worker: 3
    Shutting down thread for worker: 1
    Shutting down thread for worker: 0
    Shutting down worker 1
    Shutting down worker 2
    Shutting down worker 3
    ```

    The above output shows the case where we send three `/sleep` requests and then a final, `hello` request.

    The three `/sleep` requests are handled by the first three workers (`3`, `1` and `0`). The fourth `worker` `2` is spawned when the `/` request is sent and then, the loop ends immediately.

    This then, prints out the closing message from the `main` thread.

    Since we `drop` the sending channel before calling `join` on the threads, the closure inside the `thread`s terminate first displaying the `Shutting down thread...` messages, and then we call `join` on the threads by looping over the workers, we get the final `Shutting down worker...` messages.

    Notice, also that we first get the `Shutting down worker 0` message *before* its thread is shut down i.e., before the `join` completed. Therefore, the cleanup process is blocked here. Once the closure running in this worker’s thread discovers that the sending channel has been closed, the thread is shut down and that is when the worker is actually shut down, as well (by termination of the `join` call)
