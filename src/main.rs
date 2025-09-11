#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    Config,
};
use embassy_time::{Duration, Timer};
use embedded_hal::digital::StatefulOutputPin;

extern crate panic_halt;

async fn blink_task_inner<P: StatefulOutputPin>(mut led: P) -> ! {
    loop {
        led.toggle().unwrap();

        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
async fn blink_task(led: Output<'static>) -> ! {
    blink_task_inner(led).await
}

#[embassy_executor::task]
async fn print_task() {
    loop {
        defmt::println!("Hello world");
        Timer::after(Duration::from_millis(2000)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());

    let led = Output::new(p.PC13, Level::High, Speed::Low);

    spawner.spawn(blink_task(led)).unwrap();
    spawner.spawn(print_task()).unwrap();
}
