use embedded_hal::digital::{ErrorType, OutputPin, PinState, StatefulOutputPin};

pub struct Pin<P> {
    inner: P,
    state: PinState,
}

impl<P: OutputPin> Pin<P> {
    pub fn new(mut inner: P) -> Self {
        inner.set_low().unwrap();

        Self {
            inner,
            state: PinState::Low,
        }
    }
}

impl<P> ErrorType for Pin<P>
where
    P: OutputPin,
{
    type Error = P::Error;
}

impl<P: OutputPin> OutputPin for Pin<P> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.state = PinState::Low;
        self.inner.set_low()
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.state = PinState::High;
        self.inner.set_high()
    }
}

impl<P: OutputPin> StatefulOutputPin for Pin<P> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.state == PinState::High)
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.state == PinState::Low)
    }
}
