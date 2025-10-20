# Rust Fundamentals Projects

This repository documents my journey through foundational Rust programming, following along with examples and exercises from the official [Rust Cookbook](https://doc.rust-lang.org/stable/rust-cookbook/). Each mini-project builds upon essential Rust concepts, from setting up Cargo to handling logic, randomness, and basic I/O.

---

### 🦀 Built With

* **Language:** Rust
* **Tools:** Cargo, Rust standard library
* **Reference:** [The Rust Cookbook](https://doc.rust-lang.org/stable/rust-cookbook/)

---

## 📘 Projects Overview

### 1. Hello Cargo

**Goal:** Learn the structure and workflow of a Rust project.
This was my introduction to the Rust build system and package manager, **Cargo**.
I practiced:

* Initializing a new project with `cargo new`
* Managing dependencies through `Cargo.toml`
* Compiling and running with `cargo build` and `cargo run`

> **Core concepts:** project organization, build automation, and Rust toolchain fundamentals.

---

### 2. Number Guessing Game

**Goal:** Combine user input, randomness, and control flow.
This project recreated the classic guessing game where the program generates a random number and the user tries to guess it.

I learned to:

* Use the `rand` crate for random number generation
* Read and parse user input from the console
* Implement loops, conditionals, and comparisons
* Handle errors using `match` and `Result` types

> **Core concepts:** user interaction, randomization, type conversion, and error handling.

---

### 3. Counting Program

**Goal:** Reinforce iteration and basic computation.
This program performs simple counting or tallying operations — for example, counting numbers up to a limit or tracking occurrences in a dataset.

I explored:

* Using `for` and `while` loops effectively
* Working with ranges and iterators
* Printing formatted output
* Structuring clean and readable control flow

> **Core concepts:** iteration, flow control, and standard library fundamentals.

---

## 🧠 What I’m Learning

Through these foundational projects, I’m building a deep understanding of:

* **Ownership and borrowing** | how Rust enforces memory safety without a garbage collector
* **Error handling** | using `Result`, `Option`, and pattern matching effectively
* **Cargo and modules** | organizing code and managing external crates
* **Type system fundamentals** | static typing, conversions, and variable mutability

