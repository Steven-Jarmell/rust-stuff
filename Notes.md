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

## Common Programming Concepts  

### Variables and Mutability
  
By writing code in an immutable way you take advantage of the safety and easy concurrency that Rust offers.

The Rust compiler guarantees that when you state a value won't change, it really won't change  

Constants, much like immutable variables, are bound to a name and cannot change
- You cannot use mut with constants. Constants aren't just immutable by default, they are immutable.
- The type of the value must be annotated.
- Example: `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`
- Naming convention is to use all uppercase with underscores in between words.

Shadowing is what we saw in the guessing game where we declared a new variable with the same name as a previous variable
- Different from making a variable as mut because we get a compile-time error if we accidentally try to reassign to this variable without using the let
- By creating a new variable when we use let keyword again, we can change the type as well, using the same name  
  
### Data Types

#### Scalar: Represents a single value
- Integers
- Floating Point
- Booleans
- Characters

8-bit	i8	u8
16-bit	i16	u16
32-bit	i32	u32
64-bit	i64	u64
128-bit	i128	u128
arch	isize	usize  

Uses two's complement
[2(2^(n-1)), ..., 2^(n-1) - 1]

You are allowed to write integer literals in Decimal, Hex, Octal, Binary, or Byte
- Number literals can use '_' as a visual separator like 98_222

**The primary situation in which you'd use `isize` or `usize` is when indexing some sort of a collection**

Integer overflow
- An integer overflow will either result in panic or two's comple ent wrapping
- When compiling in debug mode, Rust includes checks for integer overflow but when you compile in release mode, Rust does not check.

You can explicitly handle the possibility of overflow
- Wrap in all modes with the wrapping_* methods, such as wrapping_add.
- Return the None value if there is overflow with the checked_* methods.
- Return the value and a boolean indicating whether there was overflow with the overflowing_* methods.
- Saturate at the value’s minimum or maximum values with the saturating_* methods.

Since the default type is f64 on modern CPU's it is roughly the same speed as f32 but with more precision, so it is the default
- All floating types are signed.

Rust's `char` type is the language's most primitive alphabetic type
- 4 bytes in size and represents a unicode scalar value

**Compound** types can group multiple values into one type
- Rust has two primitive compound types: **tuples** and **arrays**
- Tuples:
    - Example: `let tup: (i32, f64, u8) = (500, 6.4, 1);`
    - Tuples can be destructured: `let (x, y, z) = tup;`
    - Tuples can be accessed by index. `let five_hundred = x.0;`
- Arrays:
    - Every element must be of the same type, unlike a tuple
    - In Rust, Arrays have a fixed length
    - Useful when you want your data allocated on the stack rather than the heap or you want to ensure you have a fixed number of elements.
    - Vectors are a similar colelction type in the stdlib that are allowed to grow and shrink in size
    - Initializing Array
        - let a = [3; 5]; => [3, 3, 3, 3, 3]
        - let a: [i32; 5] = [1, 2, 3, 4, 5]; (Specifies type and num of elements)
    - Arrays are bracket indexed
    - When you index an array, RUst checks that the index you specify is less than the array length, else it will panic
        - Prevents you from accessing invalid memory

### Functions
- Rust uses snake case as conventional style
- You must declare the type of each parameter
- Declaring type annotations in the function's definition means the compiler almost never needs you to use them elsewhere in the code
- Function defitinitions are statements and do not return values thus you can't assign a let statement to another variable
- Statement: `let x = 3;`
- Expression: `let y = { let x = 3; x + 1};`
- Expressions do not include ending semicolons
- Functions with Return Values
    - We don't name return values but we must declare their type
    - `fn five() -> i32 { 5 }`

### Comments
- Often a comment is on a spearate line above the code its annotating

### Control Flow
- If, else/if, else blocks pretty similar to just normal java
    - You can use if in a let statement since its an expression
        - `let number = if condition { 5 } else { 6 };`
        - Types returned must be the same as each other, since variables must have a single type and cannot have joined types
- Loops
    - Three Kinds, `loop`, `while`, and `for`
    - If you have loops in loops, you can optionally add a loop label to explicitly break or continue a specified loop.
    - While loops are basic stuff
    - You can do `for element in a` to loop through a collection
    - If you do `for number in (1..4).rev()`, it will go backwards
