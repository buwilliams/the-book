# Learning Rust

Reading and working through the exercises provided in [The Book](https://doc.rust-lang.org/book).

## Notes

- Hello, world. Rust=0.001,0 Node.js=0.026,0.026
- "The Book" is approachable
- Rust is on to something: High-level ergonomics and low-level control are often at odds in programming language design; Rust challenges that conflict. (Introduction, first paragraph)
- Immutability by default!
- Such a nice developer experience (seems to have a unified community)
- rustup, rustup update
- cargo new, cargo run, cargo build, cargo build --release, cargo doc, cargo update
- Cargo.toml, Cargo.lock, semver

## Guessing Game
- use, loop, fn, println! (macro), let, match
- Shadowing is very cool, I wish I had this in other languages

## Data Types

- Scalar Types: integer=u32, floating-point numbers=f64, Booleans=bool, and characters=char
- Compound Types: tuples=tup (fixed size of different types), arrays=array (fixed size of same type)
- Vectors (dynamic size)
- integer types default to `i32`

## Functions
- fn (snakecase and lowercase: my_awesome_function)
- Rust is an expression-based language
- `Statements` are instructions that perform some action and do not return a value.
- `Expressions` evaluate to a resulting value. Expressions do not have semicolons
- {} are expressions let y = { let x = 3 x + 1 };
- return is signified by an arrow `->` (fn foo() -> u32 {})
- return synonymous with the value of the final expression in the block of the body of a function therefore `return` is optional

## Control flow

- if, loop, while, for (no parens used)
- they are expressions which means they can be assigned

## Ownership (move, drop, copy, clone)

Ownership:
- garbage colleciton, manual allocation
- Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks at compile time.
- stack (of plates, fixed size, last in/first out), heap (less organized, free allocated memory)

Three rules of ownership:
1. each value in rust has a variable owner
2. there can only be one owner
3. when the owner goes out of scope, the value is dropped

More on ownership:
- passing ownership is called `move`
- double assignment invalidates the original variable
- shallow and deep copy of memory (stack, heap)
- clone (heap) and copy (stack)
- annotation: copy, drop
- simple scalar values use copy annotation (but nothing that uses allocation)
- passing a value to a function changes it's ownership to that function scope (important)

## References and borrowing:

References:
- `&` denotes a reference, dereferencing uses `*`
- having a referencing is called `borrowing`
- we **cannot mutate references** since we do not own it
- we can mutate references if we use `&mut some_var` and `fn(foo: &mut String)`
- can only have one mutable reference to the same data in the same scope (new Rustaceans struggle with this because most languages let you mutate whenever you’d like but it's used to prevent `data races` at compile time)
- Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used

Reference Rules:
- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valie (no `dangling` refs)

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