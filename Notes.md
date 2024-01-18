# Notes

## Introduction

Compile rust programs with `rustc {filename}`  

`rustc` generates a .exe file for the application, and a .pdb file with debugging information  

Rust is an **ahead of time** compiled language, meaning you can compile a program and give the executable to someone else, and they can run it without even having Rust installed  

Bigger projects in Rust use Cargo, which is Rust's build system and package manager  
- Handles tasks like building code, downloading and building dependencies
- Best for multiple files/dependencies

You can create a new project using Cargo using `cargo new {project name}`  
- Creates a new directory with a src directory and `Cargo.toml` file
    - TOML: Tom's Obvious, Minimal Language
- It also initializes a new Git repo with a `.gitignore` 

You build a cargo project with `cargo build`
- When you are ready to release you can add `--release` to the end to compile with optimizations
    
You run a cargo project with `cargo run`  
  
You can check code to make sure it compiles without producing an executable with `cargo check`  

## Programming a Guessing Game

By default, RUst has a set of items defined in the std lib called a prelude that it brings into the scope of every program  

Create a variable with `let {varname} = ...`.  

You can make a variable mutable with the mut keyword after 'let'  

One of Rust's major advantages is how safe and easy it is to use references.  

read_line() simultaneously puts the input into the string we pass, but it also returns a Result enumeration with `Ok` or `Err` indicating the operation was successful or failed.  

When we include an external dependency, Cargo fetches the latest version of everything the dependency needs from the **registry**, which is a copy of data from (Crates.io)["https://crates.io/"]  

WHen you build a project, Cargo creates a Cargo.lock file which it will know if it changes or not.  

When you do want to update a crate, Cargo provides the command `update`, which ignores the Cargo.lock file and figures out all the latest versions for packages in the Cargo.toml. 

`rand::thread_rng()` function gives us a particular random number generator we use, one that is local to the current thread of execution and is seeded by the OS

`std::cmp::Ordering` is another enum that has values of Less, Equal, and Greater which are the 3 outcomes possible when comparing two values.  

The `match` expression is used to decide what to do next based on which variant of Ordering was returned  

Rust has a strong static type system but also has type inference.
- i32 => 32 bit number, **default**
- u32 => unsigned 32 bit number
- i64 => 64 bit number
- Others

Rust allows us to shadow the previous value of guess with a new one, and Shadowing lets us reuse the variable name without forcing us to create two unique variables.  

