use core::convert::Infallible;

use embedded_hal::digital::{ErrorType, OutputPin, PinState, StatefulOutputPin};
use legacy::{GPIO_PinState_GPIO_PIN_RESET, GPIO_TypeDef, HAL_GPIO_WritePin};

pub struct Pin {
    port: *mut GPIO_TypeDef,
    pin: u16,
    state: PinState,
}

unsafe impl Send for Pin {}

impl Pin {
    pub fn new(port: *mut GPIO_TypeDef, pin: u16) -> Self {
        let state = if GPIO_PinState_GPIO_PIN_RESET == 0 {
            PinState::Low
        } else {
            PinState::High
        };

        Self { port, pin, state }
    }
}

impl ErrorType for Pin {
    type Error = Infallible;
}

impl OutputPin for Pin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.state = PinState::Low;

        unsafe {
            HAL_GPIO_WritePin(self.port, self.pin, 0);
        }

        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.state = PinState::High;

        unsafe {
            HAL_GPIO_WritePin(self.port, self.pin, 1);
        }

        Ok(())
    }
}

impl StatefulOutputPin for Pin {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.state == PinState::High)
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.state == PinState::Low)
    }
}
