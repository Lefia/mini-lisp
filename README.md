# Mini-LISP

A simplified LISP interpreter written in Rust

## Intorduction
This is a personal project for the course "Compiler (CE3006*)" at National Central University.

## Roadmap
- [x] Basic Features
- [ ] Add tests
- [ ] Bonus Features
- [ ] Documentation

## Features
- [x] Basic Features
    - [x] Syntax validation
    - [x] Print
    - [x] Numerical Operations
    - [x] Logical Operations
    - [x] `if` Expression
    - [x] Variable Definition operations
    - [x] Function
    - [x] Named Function
- [ ] Bonus Features
    - [ ] Recursion
    - [ ] Type Checking
    - [ ] Nested Function
    - [ ] First-class Function

## Installation

```bash
git clone https://github.com/Lefia/mini-lisp.git
cd mini-lisp

# Run the interpreter
cargo run <filename.lsp>
# or ...
cargo build --release
./target/release/mini-lisp <filename.lsp>
```

## References
- [Rust Programming Language](https://www.rust-lang.org/)
- [A thoughtful introduction to the pest parser](https://pest.rs/book/)
- [Rust 开发编译器速成（一）：计算解释器](https://www.less-bug.com/posts/rust-development-compiler-crash-1-calc-interpreter/)
- [ppodds/smli](https://github.com/ppodds/smli)