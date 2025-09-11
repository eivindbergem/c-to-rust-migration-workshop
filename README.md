# Migrating an embedded C application to Rust

## Part 9 – Use `cortex-m-rt`

While we have migrated everything in `main.c`, we still use STM32Cube
for startup files and linker script.

### `cortex-m-rt`

Add `cortex-m-rt` to `blinky`:

```console
$ cargo add cortex-m-rt
```

Use the
[`entry`](https://docs.rs/cortex-m-rt/latest/cortex_m_rt/attr.entry.html)
attribute on the main function to denote the entry point.

In `.cargo/config.toml`, add the `cortex-m-rt` linker script:

```
  "-C", "link-arg=-Tlink.x",
```

`link.x` contains most of the linker script, but we need to specify
the size of ram and flash for our specific microcontroller. In the
project root, add the file `memory.x`:

```
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
  RAM : ORIGIN = 0x20000000, LENGTH = 20K
}
```

### FreeRTOS interrupt handlers

The interrupt handler function names differ between `cortex-m-rt` and
STM32Cube. Update `FreeRTOSConfig.h` with the [interrupt handlers from
`cortex-m-rt`](https://docs.rs/cortex-m-rt/latest/cortex_m_rt/attr.exception.html).

### Removed unused code

We can now remove STM32Cube HAL and startup code. 
