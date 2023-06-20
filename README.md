# Getting Started

## Setup

### Installation (MacOS or Linux)

- We’ll download Rust via the `rustup` — a command-line tool for managing Rust versions and associated tools.
    
    ```bash
    curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | shs
    ```
    
- We will select the default installtion
- The above will download the following:
    - `cargo` ⇒ the Rust package manager
    - `clippy` ⇒ linter
    - `rust-docs`
    - `rust-std`
    - `rustc` ⇒ the Rust compiler
    - `rustfmt` ⇒ formatter
- Note that a C compiler is required. On MacOS, this means that you need to install with xcode:
    
    ```bash
    xcode-select --install
    ```
    
    And on Linux:
    
    ```bash
    ## ubuntu
    apt install build-essential
    ```
    
- You might need to reload your shell or source the env to have access to the command-line utilities just installed:
    
    ```bash
    source "$HOME/.cargo/env"
    ```
    
- To check whether Rust is installed successfully, run:
    
    ```bash
    rustc --version
    ## output: rustc x.y.z (abcabcabc yyyy-mm-dd)
    ## rustc 1.70.0 (90c541806 2023-05-31)
    ```
    
- If the above does not show an output, you can check if `.cargo/bin` is in your PATH variable:
    
    ```bash
    echo $PATH
    ```
    

### Updating

- Updating rust is as simple as running:
    
    ```bash
    rustup update
    ```
    

### Uninstallation

- You can uninstall rust with:
    
    ```bash
    rustup self uninstall
    ```
    

### Documentation

- You can view the Rust documentation (the Rust book) with:
    
    ```bash
    rustup doc
    ```
    
### Quiz

<details>
 <summary>Answers</summary>

 Command-line tool for managing the version of Rust: **`rustc`**
</details>

## Hello World

### Dev Environment

- We will be using Visual Studio Code with the official [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension

### Writing and Running a Rust Program

- Create a directory and add a file to it called `main.rs`
- `rs` is the extension for Rust programs
- Add the following code:
    
    ```rust
    fn main() {
    	println!("hello, world");
    }
    ```
    
- Note that `rust-analyzer` does not work without a `Cargo.toml` file (more on this later)
- To run the program, we can first compile it with `rustc` to create a binary. We, then, run the binary.
    
    ```bash
    rustc main.rs
    ./main
    ```
    

### Anatomy

- The line `fn main() {` defines the starting of a rust `function`
- The `main` function like in other languages like `C` and `Go` is special in that it is always the first code that runs in every executable program i.e, it it the entrypoint for the program.
- If there were parameters, they would go inside the `parenthesis` `()`.
- The function body is wrapped in curlies: `{}`
- It’s good practice to put the opening curly on the same line as the function name with a space in between (although rustfmt handles this as well, run: `rustfmt main.rs`)
- The body of the main function contains the following code:
    
    ```rust
    println!("hello, world")
    ```
    
- Rust style is to indent with **four** **spaces** and **not** a tab.
- `println!` here calls a Rust macro. If it were to call a function, the `!` would not be present.
- We pass the string `hello, world` to this macro call.
- We must end the line with a semicolon `;`. This indicates that the expression is over and the next one is ready to begin.
- Compiling and running are separate steps.
- Rust is an *ahead-of-time compiled* language i.e., a compiled Rust program does not need a rust runtime meaning that you can distribute a Rust binary and run it on a system that does not itself contain Rust.

## Hello Cargo

### Introduction

- Cargo is Rust’s build system and package manager
- It handles building the code, downloading libraries (dependencies), building these libraries and even running tools
- Rest of the book and these notes assume that you are using `cargo`.
- You can check the version of cargo in your system with:
    
    ```bash
    cargo --version
    ## output: cargo 1.70.0 (ec8a8a0ca 2023-04-25)
    ```
    

### Creating a Project

- You can create a project with cargo using:
    
    ```bash
    cargo new hello_world
    ## this also initializes a git repository
    ## creates a hello-world program (main.rs) inside of a `src/` directory
    ## the top-level directory is only for README files, configs, license info and anything else not related to your code
    ```
    
- On an existing directory, you can run:
    
    ```bash
    cargo init
    ## move main.rs if it exists to src/
    ```
    
- The above will create a `Cargo.toml` file that looks something like this:
    
    ```bash
    [package]
    name = "rust-book-brown-notes"
    version = "0.1.0"
    edition = "2021"
    
    ## See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
    
    [dependencies]
    
    [[bin]]
    name = "rust-book-brown-notes"
    path = "main.rs"
    ```
    
- You can then, restart your Rust language server on VS-Code by clicking `Cmd` + `Shift` + `P` and then, running: `rust-analyzer: Restart Server`. This should create a `Cargo.lock` file.
- You should then, get IDE features like `running`, `debugging`, `viewing docs`, `peeking definitions`, etc.

### `Cargo.toml`

- TOML (Tom’s Obvious, Minimal Language) is Cargo’s configuration format
- The first line `[package]` is a section heading that indicates that the following statements are configuring a package.
- The next three lines set the configuration information Cargo needs to compile your program: then name, version and the edition of Rust to use.
- The `[dependencies]` is the start of a section for you to list any of your project dependencies. Currently, there are no dependencies.
- In Rust, packages of code are referred to as `crates`.

### Building and Running with `cargo`

- Build:
    
    ```bash
    cargo build
    ## creates an executable: target/debug/<name> (from [[bin]] section)
    ```
    
- Run:
    
    ```bash
    ./target/debug/<name>
    ```
    
- We can also combine the above with:
    
    ```bash
    cargo run
    ```
    
- We can also just check if the code compiles with:
    
    ```bash
    cargo check
    ## does not create an executable
    ## often faster than cargo build
    ```
    

### Building for Release

- When your project is ready for release, you can compile it with optimizations using
    
    ```bash
    cargo build --release
    ```
    
- This will create an executable in `target/release` (instead of `target/debug`.
- The optimizations make the code faster but the compilation slower.
- Benchmarks should be based on these optimized versions.
