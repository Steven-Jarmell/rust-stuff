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

