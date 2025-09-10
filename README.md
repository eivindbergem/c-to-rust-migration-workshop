# Migrating an embedded C application to Rust

## Overview

In this workshop we will migrate a simple embedded application from C
to Rust. Initially I wanted to migrate a real application, but the
task turned out to be too big for a 1 day workshop, so I instead opted
to migrate a simple blinky application.

Our starting point is a blinky application written in C for the
inexpensive STM32F103 dev kit known as the blue pill. It uses the
STM32Cube HAL and FreeRTOS, and consistst of two tasks: one blinking
an LED, and the other printing "hello world" every 2 seconds. It is
built using CMake. While it is a simple toy project, it is complex
enough to demonstrate many of the challenges we are facing when trying
to migrate a C application to Rust.

The workshop is built up of 10 parts:

1. Building with cargo
2. Re-organizing into workspace
3. Convert main function to Rust
4. Use `freertos_rust`
5. Convert blinky task to Rust
6. Use `stm32f1xx_hal`
7. Convert print task to Rust
8. Use `defmt-rtt` for printing
9. Use `cortex-m-rt` for startup
10. Full oxidation with `embassy`

At every step we have a fully functioning application. In this
workshop we will do a full migration from C to Rust, but in the real
world with a real migration C and Rust might co-exist for years. This
workshop aims to demonstrate that migration is not a one-off event,
but a process. There are many possible paths, and this workshop shows
one of these paths.

Some of the tasks are fairly mechanical, while others require that you
write Rust code yourself. If you're not able to finish a step, then
don't worry. There is a full solution for each part – which we will go
through together at the end of each step – and you will be given the
solution as a starting point for the next part.

The application is based on this repo:
https://github.com/lokraszewski/bluepill-blinky I have modified the
application to fix a stack overflow in one of the FreeRTOS tasks, as
well as using RTT instead of semihosting, and removed some unused
code.

## Part 1 - Building with Cargo

### Building the C application

First, make sure we that we have the submodules:

```console
$ git submodule update --init --recursive
```

Now, build the application:

```consloe
$ mkdir build && cd build
$ cmake .. -DCMAKE_TOOLCHAIN_FILE=../toolchain/stm32f103.cmake -DCMAKE_BUILD_TYPE=Debug
$ make -j
```
### Running

```console
$ probe-rs run --chip STM32F103C8 blinky/bluepill_blinky.elf
```

You should now see "Hello world" printed every other second, as well
as an LED blinking on the board.

### Create new binary crate

First, we have to creat a Cargo crate. Initialize new crate with:
```console
$ cargo init
```

Since we are on embedded, and we don't have access to the standard
library, so we have to use
[`no_std](https://docs.rust-embedded.org/book/intro/no-std.html). Additionally,
we don't have a main function (we do have one, but not in the way Rust
is expecting us to) so we also need `no_main`. Add this to the top of
`src/main.rs`:

```Rust
#[no_std]
#[no_main]
```

Since we're on `no_std`, we also need to specify a panic handler
[`panic-halt`](https://docs.rs/panic-halt/1.0.0/panic_halt/) is the
most basic one, putting our application into an eternal loop in case
of a panic.

### Build C code from Cargo

Cargo only bhas native support for building Rust, but using the
[`cc`](https://docs.rs/cc/1.2.37/cc) crate, we can build a c library.

Add `cc` as a build dependency: 

```Rust
cargo add --build cc
```

Create a [build
script](https://doc.rust-lang.org/cargo/reference/build-scripts.html),
and build the application and all dependencies into a library called
`legacy`. Include all the same files and compiler flags that are used
by CMake.

Add
[`.cargo/config.toml`](https://doc.rust-lang.org/cargo/reference/config.html)
with linker options and target triple. Use `arm-none-eabi-gcc` as
linker, and `thumbv7m-none-eabi` as target.

Add linker arguments like this: 
```
rustflags = [ 
  "-C","link-arg=<linker-argument>"
] 
```

I couldn't get `--specs=nosys.specs` to work for some reason, but
`-lnosys` does the same.

### Setup runner

Configure a runner in `.cargo/config.toml`: 
```
runner = 'probe-rs run   --chip STM32F103C8'
```

You should now be able to build and run the project with `cargo run`.
