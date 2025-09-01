# Migrating an embedded C application to Rust

## Part 3 – Convert main function to Rust

With the build system in place, we can finally start migrating some of
the code into Rust. We're doing a top down migration, starting with
the main function.

### Rust bindings to C

In order to call C from Rust, we need to know what functions are
defined in the C code. We will use
[`bindgen`](https://docs.rs/bindgen/latest/bindgen/) generate Rust
bindings from C header files.

Add `bindgen` as build dependency in the `legacy` crate:

```console
$ cargo add --build bindgen
```

Create file `legacy/src/wrapper.h`, that includes all the header files
we need to access from Rust.

In `build.rs`, use `bindgen::Builder` to create bindings from
`wrapper.h`. Export the file as `bindings.rs` in `$OUT_DIR` (use
[`std::env::var`](https://doc.rust-lang.org/std/env/fn.var.html) to
get environment variables).

The default constant macro parser in `bindgen` fails to parse some
constants that we need, but we can use `.clang_macro_fallback()` to
use clang for constant macro parsing.

In `legacy/src/lib.rs`, import the bindings:
```Rust
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
```

The C naming convention will result in a lot of warnings, but we can
add some attributes to `lib.rs` to ignore these:

```Rust
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
```

### Move `main()` to Rust

Update `main.h` to include all functions used in `main()`.

Copy the main function from C to Rust, and convert it to valid
Rust. All C code is considered unsafe, so we wrap the whole function
body in an [`unsafe`
block](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html).

For NULL pointers, we can use `core::ptr::null_mut()`.

Annotate the Rust `main()` with `#[unsafe(no_mangle)]` to disable name
mangling.
