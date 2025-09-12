#![no_std]
#![no_main]

use core::ptr;

use legacy::{
    configMINIMAL_STACK_SIZE, vBlinkTask, vPrintTask, vTaskStartScheduler, xTaskCreate, HAL_Init,
    MX_GPIO_Init, SEGGER_RTT_Init, SystemClock_Config,
};

extern crate panic_halt;

#[unsafe(no_mangle)]
fn main() -> ! {
    unsafe {
        HAL_Init();

        SystemClock_Config();
        MX_GPIO_Init();

        SEGGER_RTT_Init();

        xTaskCreate(
            Some(vBlinkTask),
            c"blink".as_ptr(),
            3 * configMINIMAL_STACK_SIZE as u16,
            ptr::null_mut(),
            3,
            ptr::null_mut(),
        );

        xTaskCreate(
            Some(vPrintTask),
            c"semi".as_ptr(),
            2 * configMINIMAL_STACK_SIZE as u16,
            ptr::null_mut(),
            2,
            ptr::null_mut(),
        );

        vTaskStartScheduler();

        loop {}
    }
}
