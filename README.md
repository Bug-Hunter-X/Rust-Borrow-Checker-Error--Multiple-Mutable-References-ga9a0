# Rust Borrow Checker Example

This repository demonstrates a common error encountered when working with references in Rust: attempting to create multiple mutable references to the same data.  The Rust compiler's borrow checker prevents this to ensure data integrity and avoid data races.

The `bug.rs` file contains code that will result in a compile-time error because of the multiple mutable reference attempt. The `bugSolution.rs` file shows how to correctly structure the code using immutable references or a single mutable reference at a time to resolve the issue. 