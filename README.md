# Learning Rust

I'm reading and working throug the exercises provided in [The Book](https://doc.rust-lang.org/book).

## Notes

- Hello, world. Rust=0.001,0 Node.js=0.026,0.026
- "The Book" is approachable
- Rust is on to something: High-level ergonomics and low-level control are often at odds in programming language design; Rust challenges that conflict. (Introduction, first paragraph)
- Immutability by default!
- Such a nice developer experience (seems to have a unified community)
- rustup, rustup update
- cargo new, cargo run, cargo build, cargo build --release, cargo doc, cargo update
- Cargo.toml, Cargo.lock, semver

Guessing Game:
- use, loop, fn, println! (macro), let, match
- Shadowing is very cool, I wish I had this in other languages

Data Types:
- Scalar Types: integer=u32, floating-point numbers=f64, Booleans=bool, and characters=char
- Compound Types: tuples=tup (fixed size of different types), arrays=array (fixed size of same type)
- Vectors (dynamic size)

Functions:
- fn (snakecase and lowercase: my_awesome_function)
- Rust is an expression-based language
- Statements are instructions that perform some action and do not return a value.
- Expressions evaluate to a resulting value. Expressions do not have semicolons
- {} are expressions let y = { let x = 3 x + 1 };
- return is signified by an arrow `->` (fn foo() -> u32 {})
- return synonymous with the value of the final expression in the block of the body of a function therefore `return` is optional

Control flow:
- if, loop, while, for (no parens used)
- they are expressions which means they can be assigned