# Migrating an embedded C application to Rust

## Part 8 – Use defmt for printing

While implementing `println!()` is fun, there is a deferred printing
framework for embedded Rust called
[`defmt`](https://defmt.ferrous-systems.com/). `defmt` doesn't only
support printing, but logging as well, and supports multiple backends
including RTT, ITM and semihosting.

### Add `defmt`

Add `defmt` and `defmt-rtt` to `blinky`:

```console
$ cargo add defmt defmt-rtt
```

`defmt` also requires an additional linking script in `.cargo/config.toml`:

```
  "-C", "link-arg=-Tdefmt.x",
```

Add this to use `defmt_rtt`:

```Rust
use defmt_rtt as _;
```

### Using `defmt`

We can now print with `defmt::println!()` in stead of our homemade
`println!()` macro. Since we don't use the Segger RTT implementation
anymore, we can delete all the RTT files.
