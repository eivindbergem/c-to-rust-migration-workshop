#![no_std]
#![no_main]
#![feature(allocator_api)]

use core::ptr;

use freertos_rust::{FreeRtosAllocator, FreeRtosUtils, Task};
use legacy::{
    configMINIMAL_STACK_SIZE, vBlinkTask, vPrintTask, HAL_Init, MX_GPIO_Init, SEGGER_RTT_Init,
    SystemClock_Config,
};

#[global_allocator]
static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;

extern crate panic_halt;

#[unsafe(no_mangle)]
fn main() -> ! {
    unsafe {
        HAL_Init();

        SystemClock_Config();
        MX_GPIO_Init();

        SEGGER_RTT_Init();
    }

    Task::new()
        .name("blink")
        .stack_size(2 * configMINIMAL_STACK_SIZE as u16)
        .start(|| unsafe {
            vBlinkTask(ptr::null_mut());
        })
        .unwrap();

    Task::new()
        .name("print")
        .stack_size(2 * configMINIMAL_STACK_SIZE as u16)
        .start(|| unsafe {
            vPrintTask(ptr::null_mut());
        })
        .unwrap();

    FreeRtosUtils::start_scheduler()
}

#[unsafe(no_mangle)]
pub extern "C" fn abort() -> ! {
    loop {}
}
