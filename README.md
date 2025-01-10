# Double Mutable Borrow in Rust
This example demonstrates a common error in Rust: attempting to create multiple mutable borrows of the same variable.  Rust's borrow checker prevents this to avoid data races and memory corruption.

The `bug.rs` file contains code that tries to have two mutable references (`y` and `z`) pointing to the same variable `x` simultaneously. This will result in a compile-time error because it violates Rust's borrowing rules. The solution shows how to avoid this issue by restructuring the code to use single mutable borrow or other suitable approaches.