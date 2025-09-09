#![no_std]
#![no_main]
#![feature(allocator_api)]

use core::ptr;

use embedded_hal::digital::StatefulOutputPin;
use freertos_rust::{CurrentTask, Duration, FreeRtosAllocator, FreeRtosUtils, Task};
use gpio::Pin;
use legacy::{configMINIMAL_STACK_SIZE, vPrintTask, SEGGER_RTT_Init};
use stm32f1xx_hal::{
    pac,
    prelude::*,
    rcc::{Config, RccExt},
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
        SEGGER_RTT_Init();
    }

    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain().freeze(
        Config::DEFAULT
            .use_hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz())
            .pclk2(72.MHz()),
        &mut flash.acr,
    );

    let mut gpioc = dp.GPIOC.split(&mut rcc);
    let led = Pin::new(gpioc.pc13.into_push_pull_output(&mut gpioc.crh));

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
