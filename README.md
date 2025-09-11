# Migrating an embedded C application to Rust

## Part 10 – Full oxidation

We have now removed the dependency on STM32Cube. We still use FreeRTOS
from Cube, but we could have replaced that with the regular
FreeRTOS. However, FreeRTOS is still written in C, so our application
is not fully in Rust.

### Embassy

Rust has support for [asynchronous
functions](https://doc.rust-lang.org/book/ch17-00-async-await.html). They
differ from threads:

| | Thread / RTOS task | Async task |
| --- | --- | --- |
| Needs stack? | Yes | No |
| Context switch? | Yes | No |
| Pre-emption? | Yes | No |

Async Rust uses lightweight co-operative tasks. As opposed to tasks in
FreeRTOS, async tasks don't need a full stack. The memory needed for
each task is known at compile time, and is equal to the memory needed
to hold the state of the task at each await point.

There are many async runtimes for Rust, but most of these use the
standard library. We will use [`embassy`](https://embassy.dev/) –
which is an async runtime and framework for embedded systems.

`embassy` comes with batteries included, so it replaces both FreeRTOS
and `stm32f1xx_hal`.

Add embassy:

```console
$ cargo add embassy-executor -F arch-cortex-m -F executor-thread
$ cargo add embassy-stm32 -F memory-x -F stm32f103c8 -F time-driver-any
$ cargo add embassy-time -F tick-hz-1_000
```

Remember to remove `stm32f1xx_hal` as it conflicts with `embassy-stm32`:

```console
$ cargo remove stm32f1xx_hal
```

### Blinky task

Rewrite the blinky task to be async. We can still use the
`StatefulOutputPin`. There are no async GPIO traits, but they have
been merge into main branch, but not released on crates.io yet. In
this case, we know that the operations are not blocking so it's ok to
use the blocking traits.

Now we can use the attribute `embassy_executor::task` to turn the
async function into an embassy task. But, because of some current
limitations in Rust we can't use generic types in tasks. We'd like to
keep our task generic, so that we can add testing or port our
application to a different microcontroller in the future. The solution
is to split the task into two separate functions, one generic async
function and a non-generic task that calls the generic function.

Replace the FreeRTOS delay function with
[`Timer::after()`](https://docs.embassy.dev/embassy-time/git/default/struct.Timer.html)

### Print task

The print task can more or less stay the same, just convert it into an
embassy task.

### Main

Convert the main function to an async function. Add attribute
[`embassy_executor::main`](https://docs.embassy.dev/embassy-executor/git/cortex-m/attr.main.html)
to use embassy as the entry point.

Use
[`embassy_stm32::init()`](https://docs.embassy.dev/embassy-stm32/git/stm32f103c8/fn.init.html)
to get the peripherals, and use
[`Output::new()`](https://docs.embassy.dev/embassy-stm32/git/stm32f103c8/gpio/struct.Output.html)
to get the output pin for the LED.

Spawn the blinky and print tasks.

Now we have a fully oxidized application and we can get rid of
STM32Cube.
