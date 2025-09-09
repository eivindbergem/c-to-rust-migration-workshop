# Migrating an embedded C application to Rust

## Part 7 – Rustify the print task

All that remains in `main.c` is the print task.

### Printing

In the C code, we use newlib with a custom `_write()` that calls the
RTT write function, which allows us to use `printf()`. In Rust, we
want to be able to use `println!()`, just like in the standard
library. Rust does not have variadic functions, so `println!()` is
implemented as a macro.

First, start with creating a unit struct – i.e. a struct with no
body. Implement `core::fmt::Write` for this struct. The `write_str()`
method should do the writing to RTT. Implementing this trait allows us
to use the
[`writeln!()`](https://doc.rust-lang.org/std/macro.writeln.html)
macro.

Write a [macro](https://doc.rust-lang.org/rust-by-example/macros.html)
 – `println!()` – that takes 0 or more arguments and passes them on to
 the `writeln!()`. Note that the trait – `core::fmt::Write` – must be
 in scope when `writeln!()` is called. It should be added to the scope
 by the macro in an
 [hygienic](https://danielkeep.github.io/tlborm/book/mbe-min-hygiene.html)
 way.

### Print task

Move the print task to Rust and use the `println!()` macro you
implemented in the previous section. We can now remove the print task
and `_write()` stub from `main.c`.
