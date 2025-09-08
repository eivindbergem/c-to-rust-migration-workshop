# Migrating an embedded C application to Rust

## Part 6 – Use `stm32f1xx_hal`

Rust has an embedded ecosystem with native support for a large number
of microcontrollers. These are pure Rust crates, and are not just
wrappers around the vendor libraries. In this part we will replace the
STM32Cube HAL with `stm32f1xx_hal`.

### Add `stm32f1xx_hal`

Add the `stm32f1xx_hal` crate with the `stm32f103` feature:
```console
cargo add stm32f1xx_hal -F stm32f103
```

### GPIO

Modify the `Pin` struct to be generic over an `OutputPin`. It should
have a constructor – `Pin::new()` – that takes an `OutputPin` as an
argument.

In `main()`, get the output pin from `stm32f1xx_hal`:

```Rust
    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut gpioc = dp.GPIOC.split(&mut rcc);

    let pin = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
```

Now, pass this output pin as an argument to `Pin::new()`, and pass
this object as the argument to the blinky task.

### Clock config

We also have the `SystemClock_Config()` function in `main.c` that we
want to replace. In `FreeRTOSConfig.h`, the clock is hardcoded to
72MHz, so we need to configure the clock to run at this rate. The
original clock config in C is a bit opaque, but it translates to:

- Use HSE – extern oscillator. The crystal on the blue pill runs at 8MHz.
- Multiply the HSE with 9 using the PLL, to arrive at 72MHz.
- PCLK1 is the system clock divided by two, giving us 36MHz.
- PCLK2 is the same as the system clock.

Use
[`Rcc::freeze`](https://docs.rs/stm32f1xx-hal/latest/stm32f1xx_hal/rcc/struct.Rcc.html#method.freeze)
to configure the clock.

### Remove unused code

We can now remove unused code in `main.c`.
