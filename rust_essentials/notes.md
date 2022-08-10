# Rust

## Introduction

* compiled
* statically typed
* strongly typed
* memory management: ownership concept (memory save by default)
* resources: [Learn Rust](https://www.rust-lang.org/learn)
* version used here: `1.48`

## Write your first program

* [x] installed Rust on Windows
* [x] installed Rust on WSL-Ubuntu22.04
* Rust has its own package manager: `Rust`
  * `cargo new <name>`:
    * create new directory
    * will initialize a git repo (if so configured)
    * uses TOML (Tom's Obvious Minimal Language) for configuration
  * `cargo run`: compile and run
  * `cargo build`: only compile/build
  * resources: [The Cargo Book](https://doc.rust-lang.org/cargo/)
* Rust history:
  * Rust 2015 (1.0 in May 2015)
  * Rust 2018 (1.31 in December 2018)
  * Rust 2021 (1.56 in October 2021)

## Primitive Data Types

* declare variables with `let var = <initial_value>;`:
  * is immutable by default
  * can be mutable by using `mut` keyword
  * var name cannot start with a number, and is _case-sensetive_
  * value type naming convention: `snake_case`
* 4 basic data types: Integers, Floating Point, Boolean, Characters
* Integer Data Types: number of bits, unsigned/signed
  * `u8/i8`, `u16/i16`, ..., `u128/i128`
  * default: `i32`
  * an overflow is automatically detected a runtime and courses a `thread 'main' panicked at 'attempt to add with overflow'`
