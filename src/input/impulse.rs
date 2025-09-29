use crate::{input::Input, signal::Signal};
use core::time::Duration;

pub struct Impulse<T> {
    value: Option<T>,
}

impl<T> Impulse<T> {
    pub fn new(value: T) -> Self {
        Impulse { value: Some(value) }
    }
}

impl<T: Default> Input for Impulse<T> {
    type Output = T;

    fn output(&mut self, dt: Duration) -> Signal<Self::Output> {
        match self.value.take() {
            Some(value) => {
                self.value = None; // Reset value after output
                Signal { value, dt }
            }
            None => Signal {
                value: T::default(),
                dt,
            }, // If no value is set, return 0.0
        }
    }
}
