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

## Understanding Ownership

### What is Ownership?

- Ownership is a set of rules that govern how a Rust program manages memory
- Memory is managed through a system of ownership with a set of rules that the compiler checks, and if any of the rules are violated, the program will not compile
- In a systems programming language like Rust, whether a value is on the stack or heap affects how the language behaves and causes you to make certain decisions
- Storing on the stack is faster than allocating on the heap b/c the allocator never has to search for a place to store new data
- Ownership addressed keeping track of what parts of code are using what data on the heap, minimizes the amoutn of duplicate data on the heap, and cleans up unused data on the heap so you don't run out of space
- Every value in rust has one owner, and when the owner goes out of scope, the value is dropped
- The '::' allows you to access a namespace to avoid having to create funciton names like string_from, you can do String::from
- String literals are hardcoded into the final executable since we know the contents at compile time and it doesn't change.
- In order to support a growing piece of text (String mutable), we need to allocate an amount of memory on the heap at runtime to hold the contents.
    - `let s = String::from("hello");` => String::from requests the memory
    - Since rust has no GC, the way it frees the memory it requests at runtime is that it frees the memory once the variable it owns goes out of scope.
    - When the variable goes out of scope, Rust calls a function called `drop`, which is where we put code to return the memory. Rust calls this function automatically at the closing curly bracket
- If you have a String literal, it's variable is added to the stack, but the actual String contents are in heap memory. 
    - If you make a second variable and set that equal to a string literal, it copies the pointer, length, and capacity which are on the stack and does not duplicate the heap contents because that would be expensive in runtime performance.
    - However, since we need to free the memory at that pointer when the variable goes out of scope, this could cause a panic. To solve this, after you set `s2 = s1`, that will invalidate s1 and will compile error if you try to use it. 
        - We say s1 `moved` to s2. It is no longer a shallow copy or deep copy
        - When s2 goes out of scope it alone will free the memory and s1 will not
        - Rust never automatically creates a deep copy of your data
    - If we do want to deep copy the heap data, we can use the `clone` method
        - Code is expensive
- Stack only data can copy automatically for integers
    - Types like integers have a known size at compile time and are entirely stored on the stacvk, so there's no reason we would want to prevent x from being valid after reassigning it.
- Rust has a special annotation called the `Copy` trait that we can place on types stored in the stack. 
    - If a type implements `Copy`, variables that use it do not move and are trivially copied, thus valid to use
    - Rust will **not** let us annotate a type with `Copy` if it or any of its parts implement the `Drop` trait
        - `Drop` indicates that the type needs something special to happen when the value goes out of scope and thus adding `Copy` causes a compile-time error
        - ALl integer types, boolean types, floating point types, character types, and tuples (if they contain types that also implement `Copy`) have the `Copy` implementation
- When you pass a value to a function, it will move or copy just like it will durring assessment. (Anything with a Copy can still be used after reassigning its value)
- Ownership is so powerful because stuff like primitives, and anything you make a `Copy` trait for can all be pushed to the stack and its value reassigned while keeping the variable accessible, whereas if it was on the Heap with no `Copy`, or if it has a `Drop` trait implementation, if it is reassigned, the previous variable is no longer able to be used
- It is annoying that anything we pass into a funciton needs to be passed back if we want to use it again, but Rust allows us to return multiple values using a tuple.
- However, Rust lets you use a value without transferring ownership, called **references**

### References and Borrowing

If we provide a reference (pointer to an address), rather than the variable, we can access the data

`let len = calculate_length(&s1);`
- Creates a reference that refers to the value of s1 but does not own s1. Thus, the value it points to will not be dropped when its reference is no longer used

The action of creating a reference is called `Borrowing`

Just as variables are immutable by default, so are references. We cannot modify something we have a reference to.

**DOES NOT WORK:**
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}

We can create mutable references:

**DOES WORK:**
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

Mutable References have one big restriction, that if you have a mutable reference to a value, you can have NO OTHER REFERENCES to that value.

By preventing multiple mutable references to the same data at the same time allows for very controlled mutation. 
- Prevents data races at compile time
- Data races are similar to race conditions and cause undefined behavior.

Way around it:

let mut s = String::from("hello");

{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;  

**Cannot combine mutable and immutable references**

Multiple immutable references are cool because no on ewho is reading the data has the ability to affect anyone else's reading of the data

In languages with pointers it's easy to create dangling pointers (a pointer that references a location in memory that may have been given to someone else). However, Rust's compiler guarantees that references will never be dangling references

Dangling Pointer Example:

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!  

When dangle is finished executing, s is deallocated due to `Drop`, thus this method would return a pointer to an address that was deallocated. Rust's compiler will throw an error if it recognizes this.

This is the solution:

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

Directly return the string, rather than a reference to it is best here.

### The Slice Type

Rust has string slices, which are references to a part of a string

let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];

Rust's '..' range syntax
- If you want to start at index 0, you can drop the value before, like `&s[..5]`
- [starting_index..ending_index]. Ending index is one more than the last position
- If you want to end at the end of the string, you can drop the value after, like `&s[5..]`
- If you want the whole string, you do &s[..]

When using slices, the compiler ensures that the references to the String remain valid, by not letting a mutable change if it is borrowed as immutable elsewhere.

If we have a string slice, we can pass that directly

In Rust you can take advantage of something called deref coercions, in which you define a method to take a slice instead of a reference which can make APIs more generalizable.

There's a general slice type you can also use for 

## Using Structs to Structuer Related Data

A struct is a custom data type that lets you package together and name multiple related values that make up a group  

Structs are similar to tuples in the sense that they can hold multiple related values. However, with structs you do not have to rely on the order of the data because you have to name every attribute

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

After you define a struct, you can create an instance by utilizing key/value pairs

Structs use dot notation to access values in them

The entire instance of a struct must be mutable and Rust **does not** allow us to mark only certain fields as mutable  

Rust has a field init shorthand syntax that allows you to get rid of repetition, for example:

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}  

**Becomes**

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}  

There is also a struct update syntax which allows for you to use less code when copying one struct instance into a new one.

let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
};  

**Becomes**

let user2 = User {
        email: String::from("another@example.com"),
        ..user1
};  

You can also use the tuple structs without named fields to create different types

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

There are also unit like structs that do not have any field
- Useful when you need to implement a trait on some type but don't have any data you want to store in the type

struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}

When defining structs, we want every instance to own all its data and for that data to be valid for as long as the struct is valid. Thus, we don't want to use slices like `&str` and instead use `String`  

A struct __can__ refer to data owned by something else but it requires the use of lifetimes, which ensure that the data referenced by a struct is valid for as long as the struct is.   

The println! macro can do many kinds of formatting and by default uses the `Display` output. Primitives and other stuff implement this by default.

If we have `println!("rec1 is {:?}", rect1);`, it will print the instance of the struct in one line.  

If we have `println!("rec1 is {:#?}", rect1);`, it will print the instance of the struct in multiple lines.  

There's also a dbg! macro that will print out to `stderr` and will also evaluate the expression.  

### Method Syntax

Methods are similar to functions but are defined __within__ the context of a struct and their first parameter is always `self`  

To define a function within the context of a Struct, you start an implementation block (`impl`)  

The &self in the method parameter is short for `self: &Self`

When you simply want to read the data from a struct and not write to it, it is always best to just borrow the struct reference.  

If we wanted to change the isntance we'd use `&mut self`

If we wanted to take ownership of the instance, we'd use `self`  
- This is rare
- Technique used when the method transforms self into something else and you want to prevent the caller from using the original instance after transforming it  

**The main reason for using a method instead of functions is for organization**  

You can give method names the same as a structs attributes, the difference is that you call methods with () and access the attributes without ()

Rust does not have the `->` operator that is in C and C++, and instead has automatic referncing and dereferencing. Given the reader and name of a method, Rust can figure out whether the method is reading (&self), mutating (&mut self), or consuming (self).

All functions defined within an `impl` block are called `associated functions` since they're assocated with the type named after the `impl`  

Assocated functions that are not methods are used for constructors that will return a new instance of the struct, often called `new`.  

You can separate methods in an `impl` block into multiple separate `impl` blocks, but theres no reason for now.  

## Enums and Pattern Matching

Enums simply say a value is one of a possible set of values.  

Enums in Rust allow you to put data into a variant of an enum, for example:  

`enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));`

However an even better way is this:

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

Enums can have any kind of data, even another enum.  

We can define methods on enums like we can on structs  

### The `Option` Enum 
The `Option` type encodes the very common scenario in which a value could either be something or nothing  W

Rust does not have a null feature that other languages do  

The problem with null values is that if you try to use `null` as a `non-null` value, you'll get an error

Rather than having nulls. Rust has an enum that encodes a value being present or absent.

The Option<T> enum is so useful that it is in the prelude and you don't need to add it explicitly into the scope

When we have an Option<T>, we have to worry about possibly not having a value and then compiler makes sure we handle that case before actually using the value.
- You have to convert an Option<T> to a T before you can perform T operations with it.
- This helps catch one of the most common issues with null: Assuming that something isnt null when it actually is.

The `match` expression is a control flow construct that runs when you have Some(T) and will run something else when it is None

### Match

Match allows you to compare a value against a series of patterns and then execute code based on the matched pattern  

Common Rust pattern: `match` against an enum, bind a variable to the data inside, then execute code based on it

Match allows you to _ or other as a catch-all in a match expression

### `if let`

Good for cases where you want a less verbose way to handle values that match one pattern and ignore the rest  

However, you lose the exhaustive checking that `match` requires

## Managing Growing Projects with Packages, Crates, and Modules

**Packages:** A Cargo feature that lets you build, test, and share crates
**Crates:** A tree of modules that produce a library or executable
**Moduels and use:** Let you control the organization, scope, and privacy of paths
**Paths:** A way of naming an item, such as a struct, funciton, or module

A crate is the smallest amount of code that a rust compiler considers at a time  

Two Types of Crates:
- Binary Crates: Programs you compile to an executable that runs
    - Each must have a main function
- Library Crates
    - No main function 
    - Rather than compile to an executable, they define functionality that you include in a project later on

The crate root is a source file that the Rust compiler starts from and makes up the root module of the crate  

A package is a bundle of one or more crates that provides a set of functionality
- Contain a `Cargo.toml` file that describes how to build the crates
- Packages can only contain one binary crate

Cargo follows a convention that src/main.rs is the crate root of a **binary** crate with the same name as the package.  

Cargo knows that if the package contains src/lib.rs, the package contains a **library** crate with the same name as the package.  

Cargo passes the crate root files to `rustc` to build the library or binary

If a package contains src/main.rs **and** src/lib.rs, it has two crates: a binary and library  

A package can have multiple binary crates by playing files in the src/bin directory and each file will be a separate binary crate  

Paths in Rust can take two forms:  
- Absolute: Full path starting form a crate root
    - For code from an external crate, this begins with the crate name
- Relative: Path starts from the current module and uses `self`, `super`, or an identifier in the current module

Paths are always separated by `::`  

Rust's preference in general is to specify absolute paths because it's more likely we'll want to move code defintions and item calls independently of each other.  

In Rust, all items including functions, methods, structs, enums, modules, and constants are private to parent modules by default.  

Items in a parent module can't use the private items inside child modules but items in child modules can use items in their ancestor modules.  

Making a module public doesn't make its contents public, it only lets code in its ancestor modules refer to it  

We can create relative paths that begin in the parent module rather than the current module by using `super`
- Like starting a filesystem with `..` syntax.

### Public Structs and Enums  

If we use pub before a struct definition, we make the struct public but it's fields will still be private.
- You can thus make each field public or not on a case-by-case basis.

If a struct has a private field, the struct must provide a public associated function that creates an instance of itself. Without this constructor, we couldn't set the value of the private field.  

When you designate an enum as public, we can use all its variants.

### Bringing Paths into Scope with `use`  

You can bring whole modules into scope like this:

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

Use essentially creates a symbolic link in the filesystem. By using `use` in the crate root, `hosting` is now a valid  

THere's a convention that when bringing in functions, just bring the parent module in and call the methods on the parent module like with a hashmap:

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

However, with stuff like enums and structs, just bring them explicitly into the scope.  

You can use the as keyword to give an alias to imports:

use std::io::Result as IoResult;

To bring external packages, you use `use {crate_name}::{items we want to bring into scope}`

We can use nested paths to clean up large use lists

use std::cmp::Ordering;
use std::io;

Turns into:  

use std::{cmp::Ordering, io};  

Another example:  

use std::io;
use std::io::Write;

becomes:  

use std::io{self, Write};

The Glob operator: use std::io::*;

## Common Collections

Three collections that are used very often:
- Vectors which allow you to store a variable number of values next to each other
- String which is a colleciton of characters
- Hash map which is a key value pair sotre

### Vectors
- Can only store values of the same type
- Variable number of values  

There are two ways to get values from a vector
- With indexing syntax: &[i]
- With the get method

Gives us two options, using the indexing can cause the program to panic, while get ensures you handle that case  

Because vectors put the values next to each other in memory, adding a new element to the end might require allocating new memory and copying the old elements to the new space.

Iterating over a vector is safe because of the borrow checker's rules. If we attempted to insert or remove items then we would get a compiler error

To get around vectors only storing the same types, we can use an enum  

Like other structs, vectors are freed when they go out of scope.

### Strings

Rust only has one type in the core language: the string slice str -> `&str`
- These are references to some UTF-8 encoded string data stored elsewhere
    - String literals are stored in the program's binary

The `String` type is in the stdlib and is growable, mutable, owned, and UTF-8 encoded. 

Since all strings are UTF-8 encoded, we can include any properly encoded data  

Both push_str and push methods don't take ownership of the caller or argument

The + operator is also used to combine strings, but because of it's method signerature, you lose ownership of the first string.

let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;

s1 in this example loses ownership because the `add` funciton is actually called and takes ownership of s1, appending s2 and s3 onto the string in the heap.

**Rust strings do not support indexing**  

In Rust, a string is a wrapper over `Vec<u8>`  

This is an edge case:

let hello = String::from("Здравствуйте");

The length of this is not 12, it is instead 24.  
These unicode scalar values take 2 bytes of storage instead of 1. Thus an index into the string's bytes will not always correlate to a valid Unicode scalar value.  

To get a character in a string, we are expecting it to take O(1), but it isn't possible to guarantee that performance with String since Rust would have to walk through the contents from the beginning to the index to determine how many valid chars there are.  

Rather than indexing a string with [] you should use a range to create a string slice containing a number of bytes  

The best way to operate on a string is to be explicit about whether you want chars or bytes.  

### Hash Maps  

For elements that we insert that implement the Copy trait, the elements are copied.  

For elements that are owned like String, the values are moved and the hashmap will be the owner of the values and the variables that used to have them will be invalid.  

Rust Hashmaps default to overwriting values at a duplicate key.  

The hashmap `entry` takes the key you want to check, and if it is not there, we can use .or_insert() to add a value at that key

## Errors

### `panic!` macro

- Either we explicitly call it or mess up some code.  

Panics by default print a failure message, unwind, clean the stack, and quit.  

A backtrace is a list of all the functions that were called to get to the panic  

### Recoverable Errors with `Result`  

Match expressions can get large and hard to read. There are methods in the standard library like `unwrap_or_else` that clean up huge nested match expressions  

There is a method `unwrap` which will return the value of a Result if its Ok, else if it is Err then it will call the panic! macro for us.

There is another method `expect` that works like `unwrap` but lets us write a custom error message

In Prod Quality Code, most Rustaceans use expect rather than unwrap and give more context  

Propagating errors (returning the error rather than handling it inside the function itself) gives more control to the calling code.  

Rust added the `?` operator to make propagating easy
- Works in almost the same way as the match expressions shown here:

let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
};

BECOMES:  

let mut username_file = File::open("hello.txt")?;  

There is a standard library function `fs::read_to_string();` that lets us read a file into a string.  

The `?` operator can only be used with functions that return a Result, Option, or another type that implements FromResidual  

A main function that returns a `Result<(), E>`, the executable will exit with a value of 0  

### Guidelines for Error Handling  

It's advisable to have your code panic when it could end up in a bad state
- A bad state is something that is unexpected, as opposed to something that will happen occasionally like bad user input
- Code needs to rely on not being in this bad state

If someone calls your code and passes incorrect values, its best to return an error to let the user of the library decide what they want to do  

When failure is expected its more appropriate to return a `Result` like rate limiting  

## Generic Types, Traits, and Lifetimes  

Lifetimes are a variety of generics that give the compiler enough information about how references relate to each other. This esnures that references are valid in more situations. 

Enums can also have generic data types in their variants  

Generics do not have slower programs if they use generics thanks to monomorphization of code using generics at compile time
- Monomorphization is when you turn generic code into specific code by filling in the conrete types used when compiled
    - Essentially, the compiler looks at all the places where the generic code is called and generates code for the conrete types the generic code is called with. 

### Traits
Traits are like interfaces in other languages.

pub trait Summary {
    fn summarize(&self) -> String;
}

Each type implementing Summary must include summarize

To implement a trait for a struct:
impl Summary for Tweet {
    fn summarize(&self) -> String {
        ...
    }
}

We can use traits as parameters  
- The method will accept any item that implements the specified trait
- In the method we can call any item that comes from the trait  

Two ways to use traits as parameters:

Syntactic sugar:
pub fn notify(item1: &impl Summary, item2: &impl Summary) 

Better for more complex
pub fn notify<T: Summary>(item1: &T, item2: &T) 

Multiple traits in parameter:

pub fn notify(item: &(impl Summary + Display))

OR

pub fn notify<T: Summary + Display>(item: &T)

The + is a trait bound which allows you to clain traits, but this can also get hard to read when it gets very long so there is a where clause

Example:  

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32

Using the clause:  

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{}

We can use a trait as a return type to return any value of a type that implements a trait

We can conditionally implement methods using traits

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Will only get implemented if the innter type T implements the PartialOrd trait as well
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

We can also conditionally implement a trait for any type that implements another trait

// If the type implements the Display trait, the ToString will be implemented on T as well
// Lets us call ToString on any type that implements Display
impl<T: Display> ToString for T {
    // --snip--
}

### Validating References with Lifetimes  

Every reference in Rust has a lifetime, or the scope for which that reference is valid for

Most of the time a lifetime is implicit and we must only annotate types when multiple types are possible.

We must annotate lifetimes when the lifetimes of references could be related in different ways. We must annotate the relationships using generic lifetime parameters

#### Lifetime Annotation Syntax
The compiler for Rust has a borrow checker which compares scopes to determine whether all borrows are valid and will not compile if any is invalid.  

The names of lifetime parameters must start with an apostrophe and are usually all lowercase and very short  
- Most use the name `'a`

&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime

If you define a return time as `&'a str`, this means the returned reference will be valid as long as both of the parameters are valid.

By using these, we tell the Rust borrow checker to reject any values that don't adhere to the constraints we specify.  

By having function signatures contain the lifetime contract means the analysis the compiler does can be simpler  

When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters  

Lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. When they're connected, Rust has enough information to allow memory-safe operations and disallow operations that create dangling pointers and violate memory safety.  

There are patterns programmed into rust called the lifetime elision rules.

Basically, a function header like this:

fn first_word(s: &str) -> &str  

Can be written as:  

fn first_word<'a>(s: &'a str) -> &'a str  

Since the Rust team found a lot of rust programmers were entering the same lifetime annotations over and over in particular situations  

If rust can't infer the lifetimes, thats when it will give an error.  

Might want to go back and read this section 10.3

Lifetime Elision Rules
1. The compiler assigns a lifetime parameter to each parameter that's a reference. These parameters are not equal to each other.
2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
3. If there's multiple input lifetime parameters, but one if &self or &mut self, the lifetime of self is assigned to all output lifetime parameters. 

One special lifetime is `'static`, which says the affected reference can live for the entire duration of the program. ALl string literals have the `'static` lifetime.

## Testing

The bodies of test functinos perform these three actions:
1. Set up any needed data or state
2. Run the code you want to test
3. Assert the results are what you expect  

A test in Rust is a funciton thats associated with the test attribute

TO change a funciton into a test function add `#[test]` on the line before fn

assert_eq! macro is very useful  

cargo test is how to run a test suite  

Tests fail if something in the test function panics. Tests are all run in a new thread, so when the main thread sees that the test thread has died, the test is marked as failed.  

The assert! macro is also useful to ensure that a condition evaluates to true  

assert_ne! macro is also another common way

You can insert custom failure messages with assert

There is a #[should_panic] attribute you can add to a test function so that the test passes if the function panics  
- You can add an expected message to ensure that the panic message also is what we expect it to be  

cargo test has a lot of optional arguments like how many threads to run, show output, if you want to run one individual tests, etc.

Can add #[ignore] to skip a test case

#[cfg(test)] tells Rust to compile and run the test code only

### Integration Testing  
We can create a tests directory at the top level of our project directory, and Cargo looks for integration tests there.  

You can specify Cargo to just run the integration tests with `cargo test --test {name of integ test file}`  

If we create a `common.rs` file and have a setup function, we can add some code to setup that we want to call from multiple test functions.
- However, we want to create a common folder and a file within it called mod.rs so that Rust doesn't treat common as a integration test 

If our project is only a binary crate and not also a library, we can't create integration test  

## I/O Project  

Rust community has guidelines for when main starts getting large:  
- Split program into a main.rs and a lib.rs and move the logic to lib.rs
- CLI parsing can remain in main as long as it is small
- Move CLI parsing to lib.rs when it starts getting complicated

Main function responsibilities:
- Calling CLI parsing logic with the arg values
- Setting up other config
- Calling a run function in lib.rs
- Handling the error if run returns an error

Main runs the program and lib.rs handles all logic  

Using `clone` to fix ownership problems should be avoided because of its runtime costs, but is acceptable at times due to the simplicity it brings.  

Many programmers expect a new function to never fail, so if there is a case it can return Err, call it build  

`Box<dyn Error>` means that the funciton will return a type that implements the Error trait  

CLI programs are expected to send error messages to stderr so we can still see error messages even if we redirect stdout  

THere is a `eprintln!` macro that prints to stderr  

## Iterators and Closures

An important part of writing idiomatic fast Rust code is mastering closures and iterators  

### Closures  
Closures are anonymous functions that you can save in a variable or pass as an argument  
- Unlike functions, closures can capture values from the scope in which they're defined  

Closures don't usually require you to annotate the types of parameters or the return value  

Custom closure:  
`
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};

Closures can capture values from their environment in three ways
- borrowing immutably
- borrowing mutably
- taking ownership

When you want to force the closure to take ownership, you can use the `move` keyword  
- Useful when passing a closure to a new thread to move the data so that its owned by the new thread  

`move` example:

use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

What happens when closure captures a reference or ownership
- Move a captured value out of closure
- Mutate captured value
- Neither more nor mutate
- Capture nothing from the environment to begin with

Closures automatically implement 1-3 of Fn traits:
- `FnOnce` applies to all closures that can be called once and all closures implement this trait since they can be called.
    - A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits
- `FnMut` applies to closures that don't move captured values out of the body but might mutate the captured ones
- `Fn` applies to closures that don't move captured values out of their body and that don't mutate captured values.  

### Iterators  

Iterator patterns allow you to perform some task on a sequence of items in turn.  

Rust iterators are lazy, meaning they have no effect until you call methods that consume the iterator  

Iterator adaptors are methods defined on the `Iterator` trait that don't consume the iterator and produce different iterators  

Iterators have a `collect` method which consumes the iterator and collects the resulting values into a collection data type.  

You can chain multiple calls to iterator adaptors to perform **complex actions in a readable way** but since they are all lazy, you have to call the consuming adaptor methods to get the results.  

Most Rust programmers prefer to use iterator style  

### Performance loops vs iterators  

Although iterators are a high-level abstraction, they get compiled to roughly the same code as if you'd written the lower level code yourself  
- They are one of Rust's zero-cost abstractions  

## Cargo and Crates.io  

In Rust, release profiles are predefined and customizable profiles with different configurations let you have options for compiling code  

Dev: `cargo build`  
Release: `cargo build --release`  

Cargo has a defule settings for optimization level (these are the default ones):  

[profile.dev]  
opt-level = 0  

[profile.release]  
opt-level = 3   

If you push your crates to crates.io, you should comment the code with ///  

You can generate HTML documentation from comments using `cargo doc`  

Sections commonly used in documentation:
- Examples
- Panics
    - The scenarios when it can panic  
- Errors
    - If it returns a result, you can describe the errors that might occur
- Safety
    - If the funciton is `unsafe`, there should be a section why

When you run `cargo test`, it will run the code in the documentation examples  

The //! comment adds documentation to the item that contains the comments rather than to the items after the comments  

You add crate metadata using the `Cargo.toml` file.  

Use `cargo publish` to upload new versions of existing crates  

Use `cargo yank --vers {num}` to deprecate a version, and if you want to undo the deprecation you can add `--undo`  

yank does not delete code----------

`Cargo Workspaes` are used if the library crate keep sgetting bigger and you want to split the package into multiple library crates  

A workspace is a set of packages that share the same `Cargo.lock` and output directory  

You must create a new folder with a Cargo.toml file and rather than a [package] section, it will have a [workspace] section  

## Smart Pointers

The references in Rust are indicated by the & and don't have capabilities other than referring to data and no overhead  

Smart pointers are data structures that act like a pointer but have additional metadata and capabilities.  

Smart pointers own the data they point to  
- String
- Vec<T>

Smart pointers are usually implemented using structs  
- Implement Defer and Drop traits  

You are allowed to write your own smart pointers  
- Box<T> for allocating values on the heap
- Rc<T> a reference counting type that enables multiple ownership
- Ref<T> and RefMut<T> accessed through RefCell<T> is a type that enforces the borrowing rules at runtime instead of compile time  

### Box<T>
Boxes don't have performance overhead other than storing on the heap instead of the stack. Use when:  
- You have a type whose size can't be known at compile time  
- YOu have a large amount of data and you want to transfer ownership but ensure data won't be copied when you do so
- You want to own a value and you care only that its a type that implements a particular trait rather than being a specific type

A value of recursive type can have another value of the same type as part of itself.  
- Since nesting of values of recursive types ould theoretically continue infinitely so Rust can't know how much space the value needs.  
  
A cons list is a data structure that is short for construction function  

Implementing the `Deref` trait lets you customize the behavior of the dereference operator *

After implementing deref for a struct, `*y` becomes `*(y.deref())`  

Deref coercion converts a reference to a type that implements the Deref trait into a refrence to another type  

Because String implements the Deref trait such that it returns &str, we can convert &String to &str  

You can use the DerefMut trait to override the * operator on mutable references  

The second trait important to the smart pointer pattern is `Drop`, which lets you customize what happens when a value is about to go out of scope  

It's not straightforward to disable to automatic drop functionality, and disabling it isn't usually necessary  

We can't call drop because Rust still automatically calls drop at the end of main, thus it would cause a double free error.  

If we want to force a value to be cleaned up early, we can use the `std::mem::drop` function which is different from the Drop trait version of drop. 

There are cases when a single value might have multiple owners, and to enable that you must use `Rc<T>` 
- Allocates some data on the heap for multiple parts of our program to read and we can't determine at compile time which part will finish using the data last. 
- Only for single-threaded scenarios  

Cloning an `Rc<T>` also increases the reference count  

Preventing memory leaks is not one of Rust's guarantees  

RefCell<T> allows the interior mutability pattern.  

The interior mutability design pattern allows you to mutate data even when there are immutable references to the data  
- Uses `unsafe`  

RefCell<T> is also for single-threaded scenaios  

Recap:

- Rc<T> enables owners of the same data, whereas Box<T> and RefCell<T> have single owners  
- Box<T> allows immutable or mutable borrows checked at **compile** time, while Rc<T> allows only immutable borrows checked at **compile** time, and RefCell<T> allows immutable or mutable borrows checked at **runtime**

You essentially wrap whatever you want an immutable/mutable borrow of, and then call the borrow or borrow_mut methods on it.  
- Borrow returns Ref<T> and borrow_mut returns RefMut<T>  
- RefCell<T> still keeps track of how many Ref<T> and RefMut<T> smart pointers there are and allows us to still have immutable borrows or one mutable borrow at any point, and if we violate these rules, we panic.  

A common way to use RefCell<T> is in combination with Rc<T>. If you hvae a Rc<T> that holds a RefCell<T>, then you can get a value that has multiple owners that you can mutate.  

You can create memory leaks with Rc<T> and RefCell<T> since you can make items refer to each other in a cycle, thus the reference count of each item will never reach 0.  

## Building Web Server

The two main protocols involved in web servers are HTTP and TCP  

Both HTTP and TCP are request-response protocols meaning the client initiates the request and a server listens and provides a response  

TCP is lower-level protocol that describes the details of how information gets from one server to another without specifying what the information is  

HTTP defines the contents and is built ontop of TCP, sending data over TCP  

A single stream represents an open connection between client and server, whereas a connection is the name for the full request and response process between client and server  

HTTP is a text-based protocol with the format:

Method Request-URI HTTP-Version CRLF  
headers CRLF  
message-body  

Request: [
    "GET / HTTP/1.1", **{Method Used} {URI} {HTTP VERSION CLIENT USES}**
    "Host: 127.0.0.1:7878",
    "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:99.0) Gecko/20100101 Firefox/99.0",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
    "Accept-Language: en-US,en;q=0.5",
    "Accept-Encoding: gzip, deflate, br",
    "DNT: 1",
    "Connection: keep-alive",
    "Upgrade-Insecure-Requests: 1",
    "Sec-Fetch-Dest: document",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-Site: none",
    "Sec-Fetch-User: ?1",
    "Cache-Control: max-age=0",
]

Responses have this format:

HTTP-Version Status-Code Reason-Phrase CRLF  
headers CRLF  
message-body  

A thread pool is a group of spawned threads that are waiting and ready to handle a task.  
- Each task gets assigned to one of the threads in the pool  
- Best to limit the number of threads rather than open a new one with each request as you can get DoS'd
- Other ways to improve throughput are fork/join model, single-threaded async I/O, and multi-threaded async I/O  

## Concurrency 

Rust has fearless concurrency since concurrency problems and memory problems go well together  

thread::spawn funciton with a closure as an argument containing the code we want creates a new thread with the code.  

When the main Rust thread completes, all spawned threads are shut down whether or not they finished running.  
- If you call .join on the thread to wait for it to finish  

### Message Passing for Transfering Data Between Threads
- Threads can send each other messgages containing data rather than sharing memory itself.  

The Rust standard lib provides an implementation of channels, which is a programming concept where data is sent from one thread to another  
- Has two halves: a transmitter and a receiver  

The transmitter of a channel is the upstream location where you put data 

The receiver is where the data ends up  

If either the transmitter or receiver half of a channel are dropped, the channel is **closed**

`mpsc` stands for multiple producer single consumer  

When using mpsc::channel, it returns a transmitter and receiver. COmmon abbreviations are tx and rx

let (tx, rx) = mpsc::channel();

Receivers have two useful methods:
- `recv`: short for receive, blocks the main thread's execution and waits until a value is sent down the channel. Returns Result<T, E>
- `try_recv`: doesn't block but immediately returns a Result<T, E>
    - Good if the thread has other work to do while waiting for messages since it can just periodically call this in between doing work  

### In rust you can also communicate with shared memory in between threads

Use mutexes to allow access to data from one thread at a time  

Rust doesn't allow you to get unlocking and unlocking wrong
- Won't allow us to access the data without calling lock  

There is Arc<T> which is used instead of Rc<T> which is a shared reference that is safe across threads.  

If the operation we are diong is simple numerical, there are simpler types than Mutex<T> which provide safe concurrent atomic access to primitive types and are provided by the `std::sync::atomic` module.  

Rst does not have many concurrency features in the language but does in the stdlib.  

The std::marker embedds Sync and Send in the language  

The `Send` marker trait indicates that the ownership of values of the type implementing Send can be transferred between Threads  

The `Sync` marker trait indicates that it is safe for the type implementing Sync to be referenced from multiple threads. T is sync if &T is Send

Types made up from Send and Sync traits are automatically Send and Sync. If we want to manually implement these two traits, we need to use unsafe Rust code.  

