# Migrating an embedded C application to Rust

## Part 2 – Convert project into workspace

Now that we build the project using Cargo, we're going convert the
repo into a [Cargo
workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) containing two crates:

1. `legacy` – containing the old C code base
2. `blinky` – containing the Rust code

### Move legacy into sub directory
Move the exising application into a subdirectory called `legacy`. This
should include both the C code and the Rust crate that we made in the
previous part.

Convert the `legacy` crate to a library crate by moving `main.rs` to
`lib.rs`. 

### Create workspace
Create a workspace by makin a `Cargo.toml` file in the
project root with a workspace with one member, the `legacy` crate.

### Add binary crate
Create a new binary crate called `blinky`: `cargo new blinky`. This
crate should also be `no_std`.

Add a linker flag `-llegacy` to link the `blinky` crate to the
`legacy` library in `.cargo/config.toml`. Note that the linker is
sensitive to ordering, so if you get errors about missing `_sbrk`, try
moving `-llegacy` to before `-lnosys`.
