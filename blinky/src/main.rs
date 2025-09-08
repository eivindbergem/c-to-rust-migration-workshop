#![no_std]
#![no_main]
#![feature(allocator_api)]

use core::ptr;

use embedded_hal::digital::StatefulOutputPin;
use freertos_rust::{CurrentTask, Duration, FreeRtosAllocator, FreeRtosUtils, Task};
use gpio::Pin;
use legacy::{
    configMINIMAL_STACK_SIZE, led_GPIO_Port, led_Pin, vPrintTask, GPIO_TypeDef, HAL_Init,
    MX_GPIO_Init, SEGGER_RTT_Init, SystemClock_Config,
};

mod gpio;

#[global_allocator]
static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;

extern crate panic_halt;

fn blink_task<P>(mut led: P)
where
    P: StatefulOutputPin,
{
    loop {
        led.toggle().unwrap();

        CurrentTask::delay(Duration::ms(500));
    }
}

#[unsafe(no_mangle)]
fn main() -> ! {
    unsafe {
        HAL_Init();

        SystemClock_Config();
        MX_GPIO_Init();

        SEGGER_RTT_Init();
    }

    let led = Pin::new(led_GPIO_Port as *mut GPIO_TypeDef, led_Pin as u16);

    Task::new()
        .name("blink")
        .stack_size(2 * configMINIMAL_STACK_SIZE as u16)
        .start(move || blink_task(led))
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
