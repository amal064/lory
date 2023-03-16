# Lory

Lory is a work-in-progress interpreter for the Lox programming language written in rust. This project is inspired by the book [Crafting Interpreters][book] by Bob Nystorm, and is being developed while reading the book.

[book]: https://craftinginterpreters.com

## Overview
Lox is a dynamically typed programming language with a C-like syntax. It features standard programming language constructs such as variables, control structures, functions, classes, and closures.

To execute Lox programs, Lory utilizes a tree walk interpreter approach, which involves walking through an abstract syntax tree (AST) to execute the program. While tree walk interpreters are straightforward and easy to implement, they may not be the most efficient interpreter type for larger programs.

## Progress
At present, Lory is work-in-progress and is not sufficiently developed to be run as an interpreter.

- [x] Scanning
- [X] Parsing
- [ ] Tree Walk Interpreting
- [ ] Variables
- [ ] Control Flow
- [ ] Functions
- [ ] Resolution
- [ ] Classes
- [ ] Inheritance
- [ ] Lists
- [ ] Loops

## Testing
The interpreter is tested using the [insta][insta] crate, which compares the interpreter's output against expected reference values (snapshots). Tests are adapted from the test suite in the Crafting Interpreters [repository][cirepo].

To run all tests, use the following command:
```bash
cargo test
```
[insta]: https://crates.io/crates/insta
[cirepo]: https://github.com/munificent/craftinginterpreters

## License
This code is licensed under either of the following, at your option:
 * [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](https://opensource.org/licenses/MIT)