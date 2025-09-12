# Migrating an embedded C application to Rust

## Part 4 - Use `freertos_rust` bindings

Interacting with the FreeRTOS C API is not very ergonomical. Luckily
for us, there is a crate with [safe Rust bindings to
FreeRTOS](https://docs.rs/freertos-rust/latest/freertos_rust/).

### Setting up `freertos_rust`

Add `freertos_rust` as a dependency to the `blinky` crate:

```console
$ cargo add freertos_rust
```

In `legacy/build.rs`, build the FreeRTOS shim together with the legacy
library. Add this line to `cc::Builder`:

```Rust
.file(PathBuf::from(env::var("DEP_FREERTOS_SHIM").unwrap()).join("shim.c"))
```

`freertos_rust` uses the heap allocated version variant of FreeRTOS
and requires us to configure the FreeRTOS heap as the global Rust
allocator. The [allocator
API](https://doc.rust-lang.org/beta/unstable-book/library-features/allocator-api.html)
is currently unstable, so we need to use the nightly compiler. Add a
file called `rust-toolchain.toml` with the following contents in the
project root:

```
[toolchain]
channel = "nightly"
```

In addition, we have to enable the unstable feature, by adding this to `main.rs`:

```Rust
#![feature(allocator_api)]
```

Now, we can add the allocator in `main.rs`:

```Rust
#[global_allocator]
static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;
```

I'm not sure why, but after switching to `freertos_rust` we get
linking errors complaining about missing `abort`. We can define it in `main.rs` to make it go away:

```Rust
#[unsafe(no_mangle)]
pub extern "C" fn abort() -> ! {
    loop {}
}
```

### Using `freertos_rust`

Now, we can start converting the calls to `xTaskCreate()` and
`vTaskStartScheduler()` to their `freertos_rust` counter parts. We'll
leave the task functions themselves in C. Note that the FreeRTOS tasks
in C takes an argument, but the ones in Rust don't. We can use a
[closure](https://doc.rust-lang.org/book/ch20-04-advanced-functions-and-closures.html)
to call the C function with a NULL pointer.
