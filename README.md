# Learning Rust

Reading and working through the exercises provided in [The Rust Programming Language, aka. The Book](https://doc.rust-lang.org/book). This README is a collection of notes I'm making as I work my way through the book. The goal is to summarize what I learn to help me remember.

## Introduction

- Hello, world. Rust=0.001,0 Node.js=0.026,0.026
- "The Book" is approachable
- Rust is on to something: High-level ergonomics and low-level control are often at odds in programming language design; Rust challenges that conflict. (Introduction, first paragraph)
- Immutability by default!
- Such a nice developer experience (seems to have a unified community)
- rustup, rustup update
- cargo new, cargo run, cargo build, cargo build --release, cargo doc, cargo update
- Cargo.toml, Cargo.lock, semver
- While rust is flexible, it also leverages the benfits of conventions such as `src/bin/*.rs`, `src/main.rs`, `src/lib.rs`, and code privacy by default

## Guessing Game
- use, loop, fn, println! (macro), let, match
- Shadowing is very cool, I wish I had this in other languages

## Data Types

- Scalar Types: integer=u32, floating-point numbers=f64, Booleans=bool, and characters=char
- Compound Types
    - `tup` (tuples) fixed size of different types `let tup: (i32, u32, str) = (-1, 2, "hello");`
    - `array` fixed size of same type `let a = [1, 2, 3, 4, 5];`
- Vectors (dynamic size)
- integer types default to `i32`

## Functions
- fn (snakecase and lowercase: my_awesome_function)
- Rust is an expression-based language
- `Statements` are instructions that perform some action and do not return a value.
- `Expressions` evaluate to a resulting value. Expressions do not have semicolons
- `{ }` are expressions
```rust
let y = {
    let x = 3
    x + 1 
};
```
- the return types are signified by an arrow `->`
```rust
fn foo() -> u32 {
    let bar: u32 = 10;
    bar
}
```
- return is synonymous with the value of the final expression in the body of a function, therefore, `return` is optional

## Control flow

- if, loop, while, for (no parens used)
- they are expressions which means they can be assigned

## Ownership

Three rules of ownership:
1. each value in rust has a variable owner
2. there can only be one owner
3. when the owner goes out of scope, the value is dropped

Notes:

- Rust does not use garbage colleciton or manual allocation
- Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks at compile time.
- `move` passes ownership from one variable to another
- `drop` cleans up memory when variables go out of scope
- stack (of plates, fixed size, last in/first out), heap (less organized, free allocated memory)
- double assignment invalidates the original variable
- `copy` creates shallow copy on the stack
- `clone` duplicates memory on the heep
- simple scalar values use copy annotation (but nothing that uses allocation)
- passing a value to a function changes it's ownership to that function scope (important)

## References

Rules:
1. At any given time, you can have either one mutable reference or any number of immutable references.
2. References must always be valid (no `dangling` refs)

Borrowing:
- having a reference as a function parameter is called `borrowing`
- `&` ampersands are references, and they allow you to refer to some value without taking ownership of it.
- `*` is used for dereferencing

Notes:
- **reference data is immutable** since we do not own the referenced data
- we can mutate references if we use `&mut some_var` and `fn(foo: &mut String)`
- can only have one mutable reference to the same data in the same scope (new Rustaceans struggle with this because most languages let you mutate whenever you’d like but it's used to prevent `data races` at compile time)
- Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used

## Slices

- slices do not have ownership just like simple scalars
- [n..x] or [n..] or [..n] or [..]

```rust
let s = String::from("hello, world.");
let hello = &s[..5]; // slice
```

- string literals are slices and immutable
- there are other slices as well

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
```

## Structs

- structs are like tuples but with keyed indexes so they are more flexible because we do not need to rely on order

```rust
struct User {
    email: String,
    first_name: String,
    last_name: String,
}
```

- `impl` - structs can have methods as well

```rust
impl User {
    fn name(&self) -> String {
        let mut s = String::from(&self.first_name);
        s.push_str(" ");
        s.push_str(&self.last_name);
        s
    }
}
```

## Enums

- enums are similar to structs but with several differences
- enums can hold different amounts of data

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

- in rust the `Option<T>` enum replaces the need for `null` with `Some<T>` and `None`
- `if let` is similar to `matches`. `if let` allows you to match on a subset of possible enums

## Manage growing projects

- `src/main.rs`, `src/lib.rs`, `src/bin/*.rs` are conventions for defining the starting places in a crate
- Helpful concepts from resturants: `front of house` (public) and `back of house` (private)
- `module tree` is how modules are organized in rust projects, they are similar to file system directories
- `paths` are how we access `modules`, two forms `absolute` or `relative` (`self` or `super`), nesting is separated by the use of `::`

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

- all code is private to the outside by default, you make it public with `pub` keyword before definition
- `use` is similar to symlinks for the file system allowing us to bring code into scope
- you can alias a module with `as`, for example: `use std::io::Result as IoResult`
- for modules `use` the parent containing scope, for structs use full path as convention
- `re-exporting` is how we bring a module into scope and allow code which calls us to use it as well with `pub use`
- importing multiple modules shorthand `use std::io::{self, Write};`
- import glob shorthand `use std::io::*`
- import module in a different file with `mod name` which loads `name.rs`

## Common Collections

- All common collections are stored on the `heap`
- Methods exist for collections such as `new` among others

### Vectors

- `Vec<T>` is a dynamic sized array, requires elments to be the same type `let v: Vec<i32> = Vec::new();`
- `let v = vec![1, 2, 3];` is a vec macro with type inference
- getting values from Vectors is easy `&v[0]` or `v.get(0)`
- adding values is easy too `v.push(4)` also `pop` is available
- iterating over a vector `for i in &v { //snip }`

```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

### Strings

- `Strings` are types of collections as well.
- `String::new`, `String::from("Hello, world.")`
- `format!` is a macro for string concatenation, for example: `println!("{}", format!("{} {}!", String::from("Hello"), String::from("world")));`
- `&s[0..1]` is used to access substrings because `&s[0]` will not compile since Strings are complicated
- `s.chars()` to iterate over a string

### Hash maps

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

- `.insert()` to add a new key/value entry
- `.entry(key).or_insert(value)` to insert if key doesn't already have a value, (entry returns a enum)

## Handling errors
