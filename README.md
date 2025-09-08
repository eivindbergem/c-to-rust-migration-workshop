# Migrating an embedded C application to Rust

## Part 5 – Rustifying blinky task

In this part we will convert the blinky task to Rust.

### GPIO abstraction

The Rust embedded ecosystem provides some [useful
abstractions](https://docs.rs/embedded-hal/latest/embedded_hal/) for
common peripherals such as GPIO, I2C and SPI. We will use the
[`OutputPin`](https://docs.rs/embedded-hal/latest/embedded_hal/digital/trait.OutputPin.html)
trait.

Add `embedded_hal` to `blinky`:

```Console
cargo add embedded-hal
```

Create a `struct` called `Pin` that holds the port and pin
id. Implement the `OutputPin` trait for this struct. Use the STM32 HAL
C functions to do GPIO operations.

### Stateful pin

We want to toggle the pin, without having to keep track of the pin
state in the blinky function. We can use
[`StatefulOutputPin`](https://docs.rs/embedded-hal/latest/embedded_hal/digital/trait.StatefulOutputPin.html)
for this.

Implement `StatfulOutputPin` for `Pin`. We have to add a field to keep
track of the pin state. We can use
[`PinState`](https://docs.rs/embedded-hal/latest/embedded_hal/digital/enum.PinState.html)
for this.

### Blinky task

In `main.rs`, write a [generic
function](https://doc.rust-lang.org/rust-by-example/generics.html)
that takes a generic pin `P` that implements `StatefulOutputPin`. Use
the `toggle()` method to toggle the pin.

Create a `Pin` and pass it to the new blinky task.
